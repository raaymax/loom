use std::{iter::Enumerate, str::Chars};
use crate::loc::Location;

#[derive(Clone)]
pub struct LocationIterator<'a> {
    it: Enumerate<Chars<'a>>,
    last_line_pos: usize,
    finished: bool,
    line: usize,
}

impl<'a> LocationIterator<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            it: text.chars().enumerate(),
            last_line_pos: 0,
            finished: false,
            line: 0,
        }
    }
}

impl Iterator for LocationIterator<'_> {
    type Item = (Location, char);

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let (pos, c) = self.it.next()?;
        let line_pos = pos - self.last_line_pos;
        if c == '\n' {
            self.last_line_pos = pos;
            self.line += 1;
        }
        Some((Location::new_point(pos, self.line, line_pos), c))
    }
}

