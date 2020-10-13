pub enum JsonToken {
    Char(JsonSyntax),
    String(String),
    Num(String),
    Bool(bool),
    Null,
    LexError{ reason: String },
}

pub enum JsonSyntax {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
}

pub struct TokenStream<I> 
    where I: Iterator,
          I::Item: Into<char> + Copy,
{
    stream_in: I,
}

impl<I> Iterator for TokenStream<I> 
    where I: Iterator,
          I::Item: Into<char> + Copy,
{
    type Item = JsonToken;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.stream_in.next() {
            let c: char = c.into();
            match c {
                '"' => {
                    let mut s = String::new();
                    while let Some(c) = self.stream_in.next() {
                        if c.into() == '"' {
                            break
                        } else {
                            s.push(c.into());
                        }
                    }
                    return Some(JsonToken::String(s))
                }
                '-' | '0'..='9' => {
                    let mut has_exponent = false;
                    let mut n = String::new();
                    n.push(c);
                    while let Some(c) = self.stream_in.next() {
                        let c: char = c.into();
                        match c {
                            '0'..='9' => n.push(c),
                            'e' => {
                                if !has_exponent {
                                    if let Some(c) = self.stream_in.next() {
                                        if c.into() == '-' {
                                            if let Some(c_next) = self.stream_in.next() {
                                                if '0' <= c_next.into() || c_next.into() <='9' {
                                                    n.push('e');
                                                    n.push(c.into());
                                                    n.push(c_next.into());
                                                    has_exponent = true;
                                                } else {
                                                    return Some(JsonToken::LexError{reason:"Improper scientific notation".to_owned()});
                                                }
                                            } else {
                                                return Some(JsonToken::LexError{reason:"Stream ended before exponent was complete".to_owned()});
                                            }
                                        } else if '0' <= c.into() || c.into() <='9' {
                                            n.push(c.into());
                                            has_exponent = true;
                                        }
                                    }
                                } else {
                                    return Some(JsonToken::LexError{reason: "Found exponential markers in scientific notation".to_owned()});
                                }
                            }
                            _ => {
                                return Some(JsonToken::LexError{reason: "Not a valid number".to_owned()});
                            }
                        }
                    }
                    return Some(JsonToken::Num(n));
                }
                'n' => {
                    let ull = self.stream_in.by_ref().take(3).map(|c| c.into()).collect::<String>();
                    if ull == "ull".to_owned() {
                        return Some(JsonToken::Null);
                    } else {
                        return Some(JsonToken::LexError{reason: "Expected null value".to_owned()});
                    }
                }
                't' => {
                    let rue = self.stream_in.by_ref().take(3).map(|c| c.into()).collect::<String>();
                    if rue == "rue".to_owned() {
                        return Some(JsonToken::Bool(true));
                    } else {
                        return Some(JsonToken::LexError{reason: "Expected boolean true value".to_owned()});
                    }

                }
                'f' => {
                    let alse = self.stream_in.by_ref().take(4).map(|c| c.into()).collect::<String>();
                    if alse == "alse".to_owned() {
                        return Some(JsonToken::Bool(false));
                    } else {
                        return Some(JsonToken::LexError{reason: "Expected boolean false value".to_owned()});
                    }

                }
                '[' => {return Some(JsonToken::Char(JsonSyntax::LeftBracket));},
                ']' => {return Some(JsonToken::Char(JsonSyntax::RightBracket));},
                '{' => {return Some(JsonToken::Char(JsonSyntax::LeftBrace));},
                '}' => {return Some(JsonToken::Char(JsonSyntax::RightBrace));},
                ',' => {return Some(JsonToken::Char(JsonSyntax::Comma));},
                ' ' | '\n' | '\t' | '\r' => {continue},
                _ => {
                    return Some(JsonToken::LexError{reason: format!("Found invalid character during lexing: {}", c)});
                }
            }
        }
        None
    }
}
