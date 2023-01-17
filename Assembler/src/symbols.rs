use std::collections::HashMap;
use crate::err_handler::LineError;

pub enum Section {
    Code,
    Data
}

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

    pub fn code_range(&self) -> Option<std::ops::Range<usize>> {
        match self.code_section {
            (Some(start), Some(end)) => Some(start..end),
            _ => None
        }
    }

    pub fn data_range(&self) -> Option<std::ops::Range<usize>> {
        match self.data_section {
            (Some(start), Some(end)) => Some(start..end),
            _ => None
        }
    }

    pub fn update_sections(&mut self, s: Section, line: usize) -> () {
        let neither_declared = || self.code_section == (None, None) &&
                                  self.data_section == (None, None);
        match s {
            Section::Code => {
                if neither_declared() {
                    // Code Section was declared first
                    self.code_section.0 = Some(line);
                } else {
                    // Code Section was declared last
                    self.code_section.0 = Some(line);
                    self.data_section.1 = Some(line);
                }
            },
            Section::Data => {
                if neither_declared() {
                    // Data Section was declared first
                    self.data_section.0 = Some(line);
                } else {
                    // Code Section was declared last
                    self.data_section.0 = Some(line);
                    self.code_section.1 = Some(line);
                } 
            }
        }
    }

    pub fn check_sections_valid(&mut self, line_num: usize) 
    -> Result<(), LineError> {
        match (self.code_section, self.data_section) {
            ((Some(_), Some(x)), (Some(y), None)) if x == y => {
                // Code Section was declared first
                Ok(self.data_section.1 = Some(line_num))
            },
            ((Some(x), None), (Some(_), Some(y))) if x == y => {
                // Data section was declared first
                Ok(self.code_section.1 = Some(line_num))
            },
            ((Some(_), None), (None, None)) => {
                // Only Code section was declared
                Ok(self.code_section.1 = Some(line_num))
            },
            ((None, None), (Some(_), None)) => {
                // Only Data section was declared
                Err(LineError::OnlyDataSection)
            },
            ((None, None), (None, None)) => {
                // No sections at all were declared
                Err(LineError::NoSectionDecl)
            }
            _ => Ok(())  // Unreachable
        }
    }
}