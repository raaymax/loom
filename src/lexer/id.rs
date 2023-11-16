use crate::{errors::PError, iter::LocationIterator};

pub fn id(it: &mut LocationIterator) -> Result<(usize,String), PError> {
    let mut buf: String = "".to_string();
    let mut size = 0;
    let mut iter = it.clone().enumerate();
    while let Some((pos,(_loc, c))) = iter.next() {
        size +=1;
        match c {
            c if c.is_alphabetic() => {
                buf.push(c);
            }
            c if (c.is_alphanumeric() || c == '_') && pos > 0 => {
                buf.push(c);
            }
            _ => {
                for _ in 0..pos {
                    it.next();
                }
                return Ok((size, buf));
            }
        }
    }
    it.last();
    Ok((size,buf))
}
