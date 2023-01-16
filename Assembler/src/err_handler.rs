use colored::Colorize;

// LineError Enum: Lists the different kinds of errors that may be 
// present while parsing the file. Some variants take Strings as 
// parameters to show additional information in the error message
#[derive(Debug)]
pub enum LineError {
    LabelMultiple(usize),
    OnlyDataSection,
    NoSectionDecl,
    StartWithAmp(usize),
    StartWithHash(usize),
    WrongSection(String, usize),
    WrongArgs(String, usize),
    LabelWhitespace(String, usize),
    Unrecognized(String, usize)
}

// Error Handler: Takes a LineError enum and panics while displaying
// a corresponding message to the screen
pub fn error_handler(e: &LineError, file_name: &str) -> () {
    let header = format!("\n{} in file {}\n", "Syntax Error".red().bold(), 
                        file_name.red());
    match e {
        LineError::LabelMultiple(n) => {
            println!("{}Can not declare multiple labels with the same name\n
                       Line Number: {}\n", header, n + 1);
        }
        LineError::OnlyDataSection => {
            println!("{}Can not assemble program with only a data\
                        section\n", header);
        }
        LineError::NoSectionDecl => {
            println!("{}Need to declare at least a Code section to\
                        assemble\n", header);
        }
        LineError::StartWithAmp(n) => {
            println!("{}Registers used as addresses must be prefixed with '{}'\n
                    Line Number: {}\n", header, "&".bold(), n + 1);
        }
        LineError::StartWithHash(n) => {
            println!("{}Immediate values must be prefixed with '{}'\n
                        Line Number: {}", header, "#".bold(), n + 1);
        }
        LineError::WrongSection(msg, n) => {
            println!("{}Did not recognize '{}'. Sections may only be {} or {}\n
                    Line Number: {}\n", 
                    header, msg.bold(), "code".bold(), "data".bold(), n + 1);
        }
        LineError::WrongArgs(msg, n) => {
            println!("{}Invalid number of arguments in {}\nLine Number: {}",
                        header, msg.bold(), n + 1);
        }
        LineError::LabelWhitespace(msg, n) => {
            println!("{}Label name must be alone in a line and \
                        without any whitespaces in between:\n'{}'\n\
                        Please do not use '{}' if you did not intend to \
                        declare a label\nLine Number: {}\n", 
                        header, msg.bold(), ":".red().bold(), n + 1);
        }
        LineError::Unrecognized(msg, n) => {
            println!("{}Did not recognize '{}'\nLine Number: {}\n", 
                        header, msg.bold(), n + 1);
        }
    }
}