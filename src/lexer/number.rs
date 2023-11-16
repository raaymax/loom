use crate::{errors::PError, iter::LocationIterator};

pub fn number(it: &mut LocationIterator) -> Result<(usize,u32), PError> {
    let mut buf:u32 = 0;
    let mut base = 10;
    let mut size = 0;
    let mut iter = it.clone().enumerate();
    while let Some((pos,(start, c))) = iter.next() {
        size += 1;
        match c {
            '0' => {
                if pos == 0 {
                    base = 8;
                }
                buf *= base;
            },
            '1'..='7' if base == 8 => {
                buf = buf * base + (c as u32 - '0' as u32);
            },
            '1'..='9' if base == 10 || base == 16 => {
                buf = buf * base + (c as u32 - '0' as u32);
            },
            'a'..='f' if base == 16 => {
                buf = buf * base + (c as u32 - 'a' as u32 + 10);
            },
            'x' if pos == 1 && buf == 0 => {
                base = 16;
            },
            _ if c.is_alphanumeric() => {
                println!("c: {} {} {:?}", c, pos, start);
                return Err(PError::new(start.set_range(pos), "Invalid number"));
            },
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
