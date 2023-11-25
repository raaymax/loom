use std::fmt::Display;
use colored::Colorize;

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
            Location::Point { line_pos, line, .. } => write!(f,"{} in line {} at pos {}", self.message, line, line_pos),
            Location::Eof => write!(f,"{}", self.message)
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
    pub fn format_error(&self, text: &str, file: &str, colors: bool) -> FormatedError{
        let Location::Range { pos, line_pos, length, line: l } = self.location.to_range() else {
            return FormatedError{
                message: self.message.to_string(),
                location: format!("{}",file),
                line_number: None,
                line: None,
                mask: None,
                colors,
            }
        };
        let err_pos = "";//format!("[ pos: {}, line: {}, line_pos: {}, length: {} ]", pos, l, line_pos, length);
        let mut line = text.chars()
            .skip(pos - line_pos)
            .take_while(|c| *c != '\n')
            .collect::<String>();
        //line.push(' ');
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
        FormatedError {
            message: self.message.to_string(),
            location: format!("{}:{}:{}",file, l, line_pos),
            line_number: Some(l),
            line: Some(line), 
            mask: Some(mask),
            colors,
        }
    }
}


pub struct FormatedError {
    pub message: String,
    pub location: String,
    pub line_number: Option<usize>,
    pub line: Option<String>,
    pub mask: Option<String>,
    pub colors: bool,
}

impl Display for FormatedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.colors {
            if let Some(line) = &self.line {
                let line_number = self.line_number.unwrap().to_string();
                let space = " ".repeat(line_number.len());
                writeln!(f, "{} {}\n {}--> {}", "error:".red().bold(), self.message.bold(), space, self.location);
                writeln!(f," {} |\n {} | {}", space, line_number, line);
                if let Some(mask) = &self.mask {
                    writeln!(f, " {} | {}", space, mask.red());
                };
            }else {
                writeln!(f, "{} {}\n  --> {}", "error:".red().bold(), self.message.bold(), self.location.yellow());
            }
        }else{
            write!(f, "{}: {}", self.location, self.message);
            if let Some(line) = &self.line {
                writeln!(f, "\t|\n\t| {}", line);
            };
            if let Some(mask) = &self.mask {
                writeln!(f, "\t| {}", mask);
            };
        };
        Ok(())
    }
}
