use std::ops::Range;
pub mod encoder;
pub mod parser;
use parser::*;

/*fn f() {
    let file = File::open("test/file1.s").unwrap();
    let reader = BufReader::new(file);
    let thing =  reader.lines().enumerate();
    let thing2 = thing.filter(|x| x.0<10);
    for i in thing2 {
        println!("{}", i.1.unwrap());
    }
}*/

fn main() {
    let file = "test/file2.s";

    // First Pass of Assembly Process.
    // Returns a symbol table for labels and ranges for Code and Data sections
    // Function panics on syntax errors
    let symbols = parse_symbols(file);
    
    for (k, v) in symbols.labels {
        println!("{} {}", k, v);
    }
}

fn _range_from_tup(tup: (usize, usize)) -> Range<usize> {
    let range = tup.1 - tup.0;
    0 .. range
}