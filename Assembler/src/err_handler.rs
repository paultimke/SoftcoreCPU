use colored::Colorize;

// LineError Enum: Lists the different kinds of errors that may be 
// present while parsing the file. Some variants take Strings as 
// parameters to show additional information in the error message
pub enum LineError {
    LabelMultiple,
    OnlyDataSection,
    NoSectionDecl,
    StartWithAmp,
    StartWithHash,
    WrongSection(String),
    WrongArgs(String),
    LabelWhitespace(String),
    Unrecognized(String)
}

// Error Handler: Takes a LineError enum and panics while displaying
// a corresponding message to the screen
pub fn error_handler (e: LineError, line_number: usize) -> () {
    let header = format!("\n{}\n{}: {}\n", "Syntax Error".red(), 
                                           "Line Number".red(),
                                            line_number + 1);
    match e {
        LineError::LabelMultiple => {
            panic!("{}Can not declare multiple labels with the same name\n\n", header)
        }
        LineError::OnlyDataSection => {
            panic!("Can not assemble program with only a data section\n");
        }
        LineError::NoSectionDecl => {
            panic!("Need to declare at least a Code section to assemble");
        }
        LineError::StartWithAmp => {
            panic!("{}Registers used as addresses must be prefixed with '{}'", 
                    header, "&".bold())
        }
        LineError::StartWithHash => {
            panic!("{}Immediate values must be prefixed with '{}'", header, "#".bold())
        }
        LineError::WrongSection(msg) => {
            panic!("{}Did not recognize '{}'. Sections may only be {} or {}\n\n", 
                    header, msg.bold(), "code".bold(), "data".bold());
        }
        LineError::WrongArgs(msg) => {
            panic!("{}Invalid number of arguments in {}", header, msg.bold());
        }
        LineError::LabelWhitespace(msg) => {
            panic!("{}Label name must be alone in a line and \
                    without any whitespaces in between:\n'{}'\n\
                    Please do not use '{}' if you did not intend to \
                    declare a label\n\n", header, msg.bold(), ":".red().bold())
        }
        LineError::Unrecognized(msg) => {
            panic!("{}Did not recognize '{}'\n\n", header, msg.bold());
        }
    }
}