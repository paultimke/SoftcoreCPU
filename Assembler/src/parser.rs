use std::fs::{File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::collections::HashMap;
use crate::err_handler::*;

// ********************* VARIABLES AND TYPE DEFINITIONS ******************** //

// Struct containig symbol table (labels) and two tuples
// describing the range of lines in the assembly source
// file that make up the code section and data section
#[derive(PartialEq, Debug)]
pub struct Symbols {
    pub labels: HashMap<String, u16>,
    pub code_section: (Option<usize>, Option<usize>),
    pub data_section: (Option<usize>, Option<usize>)  
}

impl Symbols {
    pub fn new() -> Symbols {
        Symbols {
            labels: HashMap::new(),
            code_section: (None, None),
            data_section: (None, None)
        }
    }
}

// Line Content: Categorizes the kinds of expressions
// that can be found in a source file. Errors and Sections
// are further categorized into their own variants
enum LineContent {
    Label(String),
    Instruction(String, Vec<String>),
    Data(Vec<u8>),
    Section(Section),
    Error(LineError),
    NonRelevant
}

enum Section {
    Code,
    Data
}

// *************************** PARSING FUNCTIONS *************************** //

// FIRST PASS OF ASSEMBLY PROCESS: Getting all label names and 
// storing them alongside their address in a symbol table.
// Returns a Symbol struct containing the symbol table (labels),
// And the ranges for start and end line of code and data sections
pub fn parse_symbols(file: &str) -> Symbols {
    let reader = {
        let file = File::open(file).expect("Could not open file");
        BufReader::new(file)
    };
    let mut symbols = Symbols::new();
    let mut address = 0x00;
    let mut line_idx = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        match parse_line(&line) {
            // LABELS: Append label to symbol table
            LineContent::Label(k) => {
                match symbols.labels.insert(k, address) {
                    Some(_) => error_handler(LineError::LabelMultiple, line_idx),
                    _       => ()
                }
            },
            // SECTION: Determine line ranges for each program section
            LineContent::Section(s) => match s {
                Section::Code => {
                    if symbols.code_section == (None, None) &&
                       symbols.data_section == (None, None)
                    {
                        // Code Section was declared first
                        symbols.code_section.0 = Some(line_idx);
                    } else {
                        // Code Section was declared last
                        symbols.code_section.0 = Some(line_idx);
                        symbols.data_section.1 = Some(line_idx);
                    }
                },
                Section::Data => {
                    if symbols.code_section == (None, None) &&
                       symbols.data_section == (None, None)
                    {
                        // Data Section was declared first
                        symbols.data_section.0 = Some(line_idx);
                    } else {
                        // Code Section was declared last
                        symbols.data_section.0 = Some(line_idx);
                        symbols.code_section.1 = Some(line_idx);
                    } 
                }
            },
            // DATA: Increment address by size of data
            LineContent::Data(d) => address += d.len() as u16,   
            // INSTRUCTIONS: Increment address by 1
            LineContent::Instruction(_,_) => address += 1,  
            // ERRORS: Calls error handler on corresponding error     
            LineContent::Error(e) => error_handler(e, line_idx),
            // Empty lines or comments not relevant to do any action
            LineContent::NonRelevant => ()                       
        }

        line_idx += 1;
    }

    match (symbols.code_section, symbols.data_section) {
    ((Some(_), Some(x)), (Some(y), None)) if x == y => {
        // Code Section was declared first
        symbols.data_section.1 = Some(line_idx);
    },
    ((Some(x), None), (Some(_), Some(y))) if x == y => {
        // Data section was declared first
        symbols.code_section.1 = Some(line_idx);
    },
    ((Some(_), None), (None, None)) => {
        // Only Code section was declared
        symbols.code_section.1 = Some(line_idx);
    },
    ((None, None), (Some(_), None)) => {
        // Only Data section was declared
        error_handler(LineError::OnlyDataSection, 0);
    },
    ((None, None), (None, None)) => {
        // No sections at all were declared
        error_handler(LineError::NoSectionDecl, 0)
    }
    _ => () // Unreachable
    }

    symbols
}

// SECOND PASS OF ASSEMBLY PROCESS: Traverses each section (code and data)
// line by line. Instructions and data are decoded and written to a binary
// output file
pub fn assemble_program(file: &str, symbols: Symbols, out_file: &str) -> () {
    use super::encoder::MNEMONICS;

    let mut out_file = {
        let f = File::create(out_file).expect("Could not create file");
        BufWriter::new(f)
    };

    let lines = {
        let file = File::open(file).expect("Could not open file");
        BufReader::new(file).lines()
    };

    let code_start = symbols.code_section.0.unwrap();
    let code_end = symbols.code_section.1.unwrap();
    let data_start = symbols.data_section.0.unwrap();
    let data_end = symbols.data_section.1.unwrap();

    // Read entire file
    for (idx, line) in lines.enumerate() {
        let line = line.unwrap();

        // Assemble Code Section
        if idx > code_start && idx < code_end {
            match parse_line(&line) {
                LineContent::Instruction(m, args) => {
                    // Check if Mnemonic exists. If not, throw error
                    match MNEMONICS.get(m.as_str()) {
                        Some(func) => {
                            let [msb, lsb] = func(args, &symbols, idx);
                            write!(out_file, "{}{}", msb, lsb)
                            .expect("Could not write to file");
                        }
                        None => error_handler(LineError::Unrecognized(m), idx)
                    }
                }
                _ => () // Only care if line is an instruction
            }
        }

        // Assemble Data Section
        else if idx > data_start && idx < data_end {

        }
    }
}

// Parse Line: Takes a single line from the file and determines
// what kind of line content it is. On instructions, it tokenizes
// the mnemonic and arguments into a (String, Vec<String>) for 
// further processing
fn parse_line(line: &String) -> LineContent {
    let line = line.trim();

    // Line is either a comment or pure whitespace
    if line.starts_with("//") || line.is_empty() {
        return LineContent::NonRelevant;
    }
    // Line declares the start of a section
    else if line.starts_with(".section") {
        if line.contains("Code") || line.contains("code") {
            return LineContent::Section(Section::Code);
        }
        else if line.contains("Data") || line.contains("data") {
            return LineContent::Section(Section::Data);
        }
        else {
            return LineContent::Error(LineError::WrongSection(line.trim()
                                                                  .to_string()));
        }
    }
    // Line is declaring Data
    else if line.starts_with(|x: char| x == '\"' || x.is_ascii_digit()) {
        if line.starts_with("\"") {
            // Line is a string
            let data = line.trim_matches('\"').as_bytes().to_vec();
            return LineContent::Data(data);
        }
        else {
            // Line is an array
            let data = line.split(',')
                            .map(|s| match s.trim().parse() {
                                Ok(v) => v,
                                Err(e) => {println!("{}", e); 0},
                            });
            return LineContent::Data(data.map(|x| x as u8).collect()); 
        }
    }
    // Line is a label
    else if line.contains(":") {
        // Trim whitespace and eliminate the ':' character at the end
        let symbol = {
            let s = line[0..line.len()-1].to_string();
            if s.contains(" ") || s.contains("\t") {
                return LineContent::Error(LineError::LabelWhitespace(s));
            } else {
                s
            }
        };
        return LineContent::Label(symbol)
    }
    // Line is either an instruction or a syntax error
    else  {
        let mut instr: Vec<_> = line.split(" ").map(|s| s.to_string()).collect();
        if instr.len() >= 1 && instr[0].is_ascii() {
            let mnemonic = instr.remove(0).to_lowercase().to_string();
            let params = instr;
            return LineContent::Instruction(mnemonic, params);
        } else {
            // Unrecognized string pattern
            return LineContent::Error(LineError::Unrecognized(line.trim().to_string()));
        }
    }
}


// TESTING MODULE
#[cfg(test)]
mod tests {
    use std::{io::Read, fs::remove_file};

    use super::*;

    #[test]
    // Tests only for labels and nothing else [Test File 1]
    fn labels_file1() {
        let compare_symbols = Symbols {
            labels: HashMap::from(
                [("start".to_string(), 0u16), 
                ("loop".to_string(), 4u16), 
                ("end_loop".to_string(), 11u16),
                ("arr".to_string(), 13u16)]
            ),
            code_section: (None, None),
            data_section: (None, None)
        };
        assert_eq!(compare_symbols.labels, parse_symbols("test/file1.s").labels);
    }

    #[test]
    // Tests only for labels and nothing else [Test File 2]
    fn labels_file2() {
        let compare_symbols = Symbols {
            labels: HashMap::from(
                [("sum2nums".to_string(), 7u16), 
                ("sub2nums".to_string(), 9u16)]
            ),
            code_section: (None, None),
            data_section: (None, None)
        };
        assert_eq!(compare_symbols.labels, parse_symbols("test/file2.s").labels);
    }

    #[test]
    fn assemble_test1() {
        match compare_files("test/file1.s", "test/file1.bin") {
            Ok(_) => (),
            Err(_) => panic!()
        };
    }

    fn compare_files(ref_asm_path: &str, ref_bin_path: &str) -> Result<(), ()> {
        let result_bin_name = &format!("{}_test.bin", ref_bin_path);
        let symbols = parse_symbols(&ref_asm_path);
        assemble_program(&ref_asm_path, symbols, &result_bin_name);

        let f_ref = File::open(ref_bin_path).expect("could not open file");
        let f_res = File::open(result_bin_name).expect("Could not open file");

        // Check if file sizes are different
        if f_ref.metadata().unwrap().len() != f_res.metadata().unwrap().len() {
            remove_file(result_bin_name).unwrap();
            return Err(());
        }

        // Use buf readers since they are much faster
        let f_ref = BufReader::new(f_ref);
        let f_res = BufReader::new(f_res);

        // Do a byte to byte comparison of the two files
        for (b1, b2) in f_ref.bytes().zip(f_res.bytes()) {
            if b1.unwrap() != b2.unwrap() {
                remove_file(result_bin_name).unwrap();
                return Err(());
            }
        }

        remove_file(result_bin_name).unwrap();
        return Ok(());
    }
}