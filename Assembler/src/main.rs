pub mod encoder;
pub mod parser;
pub mod err_handler;
pub mod symbols;
use err_handler::error_handler;
use parser::*;

// TODO: Add command line arguments to specify the names of
// the input file, flags and name of the output file instead
// of harcoding the names and paths
fn main() {
    let file = "test/file1.s";
    let out_file = "out.bin";

    // First Pass of Assembly Process.
    // Returns a symbol table for labels and ranges for Code and Data sections
    // Function panics on syntax errors
    let symbols = parse_symbols(&file);
    if let Err(e) = &symbols {
        error_handler(e, &file);
        return;
    }
    let symbols = symbols.unwrap();

    // Second Pass of Assembly Process.
    // Parses instructions and encodes them into an output file
    if let Err(e) = assemble_program(&file, symbols, out_file) {
        error_handler(&e, &file);
    }
}