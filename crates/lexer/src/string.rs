use crate::{errors::PError, iter::{LocationIterator, BufferedIterator}};

pub fn string(it: &mut BufferedIterator<LocationIterator>) -> Result<(usize,String), PError> {
    let mut buf: String = "".to_string();
    let mut delimiter = '"';
    let mut size = 0;
    while let Some((_loc, c)) = it.next() {
        size +=1;
        match c {
            '"' if size == 1 => {
                delimiter = '"';
            },
            '\'' if size == 1 => {
                delimiter = '\'';
            },
            c if c == delimiter && size > 1 => {
                return Ok((size, buf));
            }
            _ => {
                buf.push(c);
            }
        }
    }
    it.last();
    Ok((size,buf))
}
