
#[macro_export]
macro_rules! accept {
    ( $iter:ident, $token:ident ) => {
        {
            let Some(token) = $iter.next() else {
                return Err(PError::new(Location::Eof, "Unexpected end of file"));
            };
            if !matches!(token, Token::$token(..)) {
                return Err(PError::new(token.get_location(), format!("Unexpected token: {}", token).as_str()))
            }
            token
        }
    };
}
#[macro_export]
macro_rules! expect {
    ( $ret:expr, $token:ident ) => {
        let Some(token) = $ret else {
            return Err(PError::new(Location::Eof, "Unexpected end of file"))
        };
        if !matches!(token, Token::$token(..)) {
            return Err(PError::new(token.get_location(),format!("Unexpected token: {}", token).as_str()))
        }
    };
}
