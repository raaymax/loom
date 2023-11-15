#![allow(dead_code)]
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum Location {
    Range{
        pos: usize,
        line_pos: usize,
        line: usize,
        length: usize,
    },
    Point{
        pos: usize,
        line_pos: usize,
        line: usize,
    },
    Eof,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Location::Range { pos, length, .. } => write!(f,"[{}:{}]", pos, length),
            Location::Point { pos, .. } => write!(f,"[{}]", pos),
            Location::Eof => write!(f,"[EOF]")
        }
    }
}

impl Location {
    pub fn new_point(pos: usize, line: usize, line_pos: usize) -> Self {
        Self::Point { pos, line_pos, line } 
    }

    pub fn new_range(pos: usize, line: usize, line_pos: usize, length: usize) -> Self {
        Self::Range { pos, line_pos, line, length} 
    }

    pub fn to_point(&self) -> Self {
        match self {
            Self::Range { pos, line_pos, line, .. } => Self::Point { pos: *pos, line_pos: *line_pos, line: *line },
            Self::Point { pos, line_pos, line } => Self::Point { pos: *pos, line_pos: *line_pos, line: *line },
            Self::Eof => Self::Eof,
        }
    }
    pub fn to_range(&self) -> Self {
        match self {
            Self::Range { pos, line_pos, line, length } => Self::Range{ pos: *pos, line_pos: *line_pos, line: *line, length: *length },
            Self::Point { pos, line_pos, line } => Self::Range{ pos: *pos, line_pos: *line_pos, line: *line, length: 1 },
            Self::Eof => Self::Eof,
        }
    }

    pub fn set_range(&self, length: usize) -> Self {
        match self {
            Self::Range { pos, line_pos, line, length: _ } => Self::Range{ pos: *pos, line_pos: *line_pos, line: *line, length },
            Self::Point { pos, line_pos, line } => Self::Range{ pos: *pos, line_pos: *line_pos, line: *line, length },
            Self::Eof => Self::Eof,
        }
    }

}


