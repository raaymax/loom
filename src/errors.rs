use std::fmt::Display;

use crate::loc::Location;

#[derive(Debug)]
pub struct PError {
    location: Location,
    message: String,
}

impl Display for PError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.location {
            Location::Range { line_pos, line, .. } => write!(f,"'{}' in line {} at pos {}", self.message, line, line_pos),
            Location::Point { line_pos, line, .. } => write!(f,"{} in line {} at pos {}", self.message, line, line_pos)
        }
    }
}

impl PError {
    pub fn new(location: Location, message: &str) -> Self {
        PError {
            location,
            message: message.to_string(),
        }
    }
    pub fn format_error(&self, text: &str) -> String {
        let Location::Range { pos, line_pos, length, .. } = self.location.to_range() else {
            panic!("Location is not a range");
        };
        let line = text.chars()
            .skip(pos - line_pos)
            .take_while(|c| *c != '\n').collect::<String>();
        println!("line: {} {}", line, length);
        let mut mask = line.clone();
        for l in 0..line.len() {
            let ch = {
                if (line_pos..line_pos+ length).contains(&l) { 
                    "^" 
                }else{
                    " "
                }
            };
            mask.replace_range(l..l+1, ch);
        }
        format!("{}\n{}\n{}", line, mask, self.message)
    }
}

