use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use crate::err_handler::LineError;
use crate::symbols::{Symbols, Section};

// ********************* VARIABLES AND TYPE DEFINITIONS ******************** //

// Line Content: Categorizes the kinds of expressions
// that can be found in a source file. Errors and Sections
// are further categorized into their own variants
enum LineContent {
    Label(String),
    Instruction(String, Vec<String>),
    Data(Vec<u8>),
    Section(Section),
    NonRelevant
}

// *********************** MAIN ASSEMBLING FUNCTIONS *********************** //

// FIRST PASS OF ASSEMBLY PROCESS: Getting all label names and 
// storing them alongside their address in a symbol table.
// Returns a Symbol struct containing the symbol table (labels),
// And the ranges for start and end line of code and data sections
pub fn parse_symbols(file: &str) -> Result<Symbols, LineError> {
    let file = File::open(file).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut symbols = Symbols::new();
    let mut address = 0x00;
    let mut line_idx = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        match parse_line(&line, line_idx)? {
            // LABELS: Append label to symbol table
            LineContent::Label(k) => {
                match symbols.labels.insert(k, address) {
                    Some(_) => Err(LineError::LabelMultiple(line_idx)),
                    _       => Ok(())
                }
            }
            // SECTION: Determine line ranges for each program section
            LineContent::Section(s) => Ok(symbols.update_sections(s, line_idx)),
            // DATA: Increment address by size of data
            //       Divide by 2 as d is a vec of bytes, and words are 2 bytes
            LineContent::Data(d) => Ok(address += (d.len()/2) as u16),   
            // INSTRUCTIONS: Increment address by 1
            LineContent::Instruction(_,_) => Ok(address += 1),  
            // Empty lines or comments not relevant to do any action
            LineContent::NonRelevant => Ok(()),              
        }?;
        line_idx += 1;
    }

    // Check if section declarations are valid and populate 
    // upper bound of one of them
    symbols.check_sections_valid(line_idx)?;

    return Ok(symbols);
}

// SECOND PASS OF ASSEMBLY PROCESS: Traverses each section (code and data)
// line by line. Instructions and data are decoded and written to a binary
// output file
pub fn assemble_program(file: &str, syms: Symbols, out_file: &str) 
-> Result<(), LineError> {
    use super::encoder::MNEMONICS;

    let mut out_file = {
        let f = File::create(out_file).expect("Could not create file");
        BufWriter::new(f)
    };
    let lines = {
        let file = File::open(file).expect("Could not open file");
        BufReader::new(file).lines()
    };

    // Traverse entire file
    for (idx, line) in lines.enumerate() {
        let line = line.unwrap();

        // Assemble Code Section
        if syms.code_range().is_some() && 
           syms.code_range().unwrap().contains(&idx) 
        {
            match parse_line(&line, idx)? {
                LineContent::Instruction(m, args) => {
                    // Check if Mnemonic exists. If not, throw error
                    match MNEMONICS.get(m.as_str()) {
                        Some(func) => {
                            // Encode instruction into two bytes [msb, lsb]
                            let bytes = func(trim_comments(args), &syms, idx)?;
                            // Write encoded bytes to output file
                            out_file.write_all(&bytes).expect("Can not write output file");
                            Ok(())
                        }
                        None => Err(LineError::Unrecognized(m, idx))
                    }
                }
                LineContent::Data(_) => Err(LineError::SectionMismatch(idx)),
                _ => Ok(()) // Only care if line is an instruction
            }?
        }
        // Assemble Data Section
        else if syms.data_range().is_some() && 
                syms.data_range().unwrap().contains(&idx) 
        {
            match parse_line(&line, idx)? {
                LineContent::Data(d) => {
                    out_file.write_all(&d).expect("Can not write output file");
                    Ok(())
                }
                LineContent::Instruction(_, _) => {
                    Err(LineError::SectionMismatch(idx))
                }
                _ => Ok(()) // Only care if line is data
            }?
        }
    }

    Ok(())
}

// Parse Line: Takes a single line from the file and determines
// what kind of line content it is. On instructions, it tokenizes
// the mnemonic and arguments into a (String, Vec<String>) for 
// further processing
fn parse_line(line: &String, line_num: usize) -> Result<LineContent, LineError> {
    let line = line.trim();

    // Line is either a comment or pure whitespace
    if line.starts_with("//") || line.is_empty() {
        Ok(LineContent::NonRelevant)
    }
    // Line declares the start of a section
    else if line.starts_with(".section") {
        parsed_section(line, line_num)
    }
    // Line is declaring Data
    else if line.starts_with(|x: char| x == '\"' || x.is_ascii_digit()) {
        parsed_data(line, line_num)
    }
    // Line is a label
    else if line.contains(":") {
        parsed_label(line, line_num)
    }
    // Line is either an instruction or a syntax error
    else  {
        parsed_instruction(line, line_num)
    }
}

// **************************** HELPER FUNCTIONS **************************** //

fn trim_comments(args: Vec<String>) -> Vec<String>{
    let mut v = args;
    for i in 0..v.len() {
        if v[i].starts_with("//") {
            v.truncate(i);
            break;
        }
    }
    let v = v.iter().filter(|s| s.trim().len() != 0).map(|s| s.to_string()).collect();
    v
}

fn parsed_section(line: &str, line_num: usize) -> Result<LineContent, LineError> {
    if line.contains("Code") || line.contains("code") {
        Ok(LineContent::Section(Section::Code))
    }
    else if line.contains("Data") || line.contains("data") {
        Ok(LineContent::Section(Section::Data))
    }
    else {
        Err(LineError::WrongSection(line.trim().to_string(), line_num))
    } 
}

// Using the as_bytes() method on a string apparently treats control 
// characters as two individual bytes. Example '\n' yields [b'\\', 'n']
// This function finds these two-byte occurrences matching control chars
// and replaces the first byte with the actual value for the control char
// and removes the second byte 
fn replace_control_ascii(s: &mut Vec<u8>) {
    let mut vec_len = s.len();
    let mut i = 0;
    while i < (vec_len-1) {
        if s[i] == b'\\' && (s[i+1] == b'n' || s[i+1] == b't') {
            match s[i + 1] {
                b'n' => s[i] = b'\n',
                b't' => s[i] = b'\t',
                _ => ()
            }
            s.remove(i+1); // Remove t or n (from newline or tab)
            vec_len = vec_len - 1;
        }
        i += 1;
    }
}

fn parsed_data(line: &str, line_num: usize) -> Result<LineContent, LineError> {
    let data: Vec<u16>;
    if line.starts_with("\"") {
        // Line is a string
        let mut char_arr = line.trim_matches('\"').as_bytes().to_vec();
        replace_control_ascii(&mut char_arr);
        char_arr.push(0); // Push NULL termination character for string
        data = char_arr.iter().map(|c| *c as u16).collect();
    } else if line.chars().nth(0).unwrap().is_ascii_digit() {
        // Line is an array
        data = line.split(',')
                    .map(|s| match s.trim().parse::<i16>() {
                        Ok(v) => v,
                        Err(e) => {println!("{} line_number: {}", e, line_num); 0},
                    }).map(|c| c as u16).collect();
    } else {
        return Err(LineError::Unrecognized(line.to_string(), line_num));
    }

    // Convert the 16 bit data vectors to an 8-bit data vector with
    // their msb and lsb
    let byte_size: u8 = 8;
    let mut new_data: Vec<u8> = Vec::new();
    for i in 0..data.len() {
        new_data.push((data[i] >> byte_size) as u8); // msb
        new_data.push(data[i] as u8); // lsb
    }

    return Ok(LineContent::Data(new_data));
}

fn parsed_label(line: &str, line_num: usize) -> Result<LineContent, LineError> {
    if line.matches(":").count() > 1 {
        Err(LineError::LabelMoreColon(line.trim().to_string(), line_num))
    }
    else {
        // Trim whitespace and eliminate the ':' character at the end
        let l = line[0..line.len()-1].to_string();
        if l.contains(" ") || l.contains("\t") {
            Err(LineError::LabelWhitespace(l, line_num))
        } else {
            Ok(LineContent::Label(l))
        }
    }
}

fn parsed_instruction(line: &str, line_num: usize) -> Result<LineContent, LineError> {
    let mut instr: Vec<_> = line.split(" ").map(|s| s.to_string()).collect();
    if instr.len() >= 1 && instr[0].is_ascii() {
        let mnemonic = instr.remove(0).to_lowercase().to_string();
        let params = instr;
        return Ok(LineContent::Instruction(mnemonic, params));
    } else {
        // Unrecognized string pattern
        return Err(LineError::Unrecognized(line.trim().to_string(), line_num));
    } 
}

// ***************************** TESTING MODULE ***************************** //
#[cfg(test)]
mod tests {
    use std::{io::Read, fs::remove_file};
    use std::collections::HashMap;
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
        assert_eq!(compare_symbols.labels, 
                   parse_symbols("test/file1.s").unwrap().labels);
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
        assert_eq!(compare_symbols.labels, 
                   parse_symbols("test/file2.s").unwrap().labels);
    }

    #[test]
    // Tests for section ranges [Test File 1]
    fn sections_file1() {
        let code_sec = 1..21;
        let data_sec = 21..49;
        let symbols = parse_symbols("test/file1.s").unwrap();
        assert_eq!(code_sec, symbols.code_range().unwrap());
        assert_eq!(data_sec, symbols.data_range().unwrap());
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
        let symbols = parse_symbols(&ref_asm_path).unwrap();
        if let Err(_) = assemble_program(&ref_asm_path, symbols, &result_bin_name) {
            panic!();
        }

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