use crate::{errors::PError, iter::{LocationIterator, BufferedIterator}};

pub fn number(it: &mut BufferedIterator<LocationIterator>) -> Result<(usize,u32), PError> {
    let mut buf:u32 = 0;
    let mut base = 10;
    let mut size = 0;
    let mut pos = 0;
    while let Some((start, c)) = it.peek() {
        size += 1;
        match c {
            '0' => {
                if pos == 0 {
                    base = 8;
                }
                buf *= base;
                it.next();
            },
            '1'..='7' if base == 8 => {
                buf = buf * base + (c as u32 - '0' as u32);
                it.next();
            },
            '1'..='9' if base == 10 || base == 16 => {
                buf = buf * base + (c as u32 - '0' as u32);
                it.next();
            },
            'a'..='f' if base == 16 => {
                buf = buf * base + (c as u32 - 'a' as u32 + 10);
                it.next();
            },
            'x' if pos == 1 && buf == 0 => {
                base = 16;
                it.next();
            },
            _ if c.is_alphanumeric() => {
                println!("c: {} {} {:?}", c, pos, start);
                return Err(PError::new(start.set_range(pos), "Invalid number"));
            },
            _ => {
                return Ok((size, buf));
            }
        }
        pos += 1;
    }
    it.last();
    Ok((size,buf))
}
