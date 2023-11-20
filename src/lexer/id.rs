use crate::{errors::PError, iter::{LocationIterator, BufferedIterator}};

pub fn id(it: &mut BufferedIterator<LocationIterator>) -> Result<(usize,String), PError> {
    let mut buf: String = "".to_string();
    let mut size = 0;
    let mut pos = 0;
    while let Some((_loc, c)) = it.peek() {
        size +=1;
        match c {
            c if c.is_alphabetic() => {
                buf.push(c);
                it.next();
            }
            c if (c.is_alphanumeric() || c == '_') && pos > 0 => {
                buf.push(c);
                it.next();
            }
            _ => {
                return Ok((size, buf));
            }
        }
        pos += 1;
    }
    it.last();
    Ok((size,buf))
}
