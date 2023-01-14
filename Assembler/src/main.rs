pub mod encoder;
pub mod parser;
pub mod err_handler;
use parser::*;

// TODO: Add command line arguments to specify the names of
// the input file, flags and name of the output file instead
// of harcoding the names and paths

fn main() {
    let file = "test/file2.s";
    let out_file = "file2.bin";

    // First Pass of Assembly Process.
    // Returns a symbol table for labels and ranges for Code and Data sections
    // Function panics on syntax errors
    let symbols = parse_symbols(&file);
    
    // Second Pass of Assembly Process.
    // Parses instructions and encodes them into an output file
    assemble_program(&file, symbols, out_file);
}