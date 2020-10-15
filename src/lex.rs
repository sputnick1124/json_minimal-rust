use std::iter;

pub enum JsonToken {
    Char(JsonSyntax),
    String(String),
    Number(f64),
    Bool(bool),
    Null,
    LexError { reason: String },
}

pub enum JsonSyntax {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
}

pub struct TokenStream<I> {
    stream_in: I,
}

pub trait Tokenizable: Iterator {
    fn tokenize(self) -> TokenStream<Self>
    where
        Self: Sized,
        Self::Item: Into<char> + Copy,
    {
        TokenStream { stream_in: self }
    }
}

impl<T: ?Sized> Tokenizable for T where T: Iterator {}

impl<I> Iterator for TokenStream<I>
where
    I: Iterator,
    I::Item: Into<char> + Copy,
{
    type Item = JsonToken;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.stream_in.next() {
            match c.into() {
                '"' => Some(JsonToken::String(
                    self.stream_in
                        .by_ref()
                        .take_while(|&c| c.into() != '"')
                        .map(|c| c.into())
                        .collect::<String>(),
                )),
                '-' | '0'..='9' => {
                    let num_str = iter::once(c.into())
                        .chain(
                            self.stream_in
                                .by_ref()
                                .take_while(|&c| matches!(c.into(), '0'..='9' | '.' | 'e' | 'E' | '-'))
                                .map(|c| c.into()),
                        )
                        .collect::<String>();
                    if let Ok(num) = num_str.parse::<f64>() {
                        Some(JsonToken::Number(num))
                    } else {
                        Some(JsonToken::LexError {
                            reason: format!("Unable to lex {} as f64", num_str),
                        })
                    }
                }
                'n' => {
                    let maybe_null = iter::once(c)
                        .chain(self.stream_in.by_ref())
                        .take(4)
                        .map(|c| c.into())
                        .collect::<String>();
                    if maybe_null == "null" {
                        Some(JsonToken::Null)
                    } else {
                        Some(JsonToken::LexError {
                            reason: "Expected null value".to_owned(),
                        })
                    }
                }
                't' => {
                    let maybe_true = iter::once(c)
                        .chain(self.stream_in.by_ref())
                        .take(4)
                        .map(|c| c.into())
                        .collect::<String>();
                    if maybe_true == "true".to_owned() {
                        Some(JsonToken::Bool(true))
                    } else {
                        Some(JsonToken::LexError {
                            reason: "Expected boolean true value".to_owned(),
                        })
                    }
                }
                'f' => {
                    let maybe_false = iter::once(c)
                        .chain(self.stream_in.by_ref())
                        .take(5)
                        .map(|c| c.into())
                        .collect::<String>();
                    if maybe_false == "false".to_owned() {
                        Some(JsonToken::Bool(false))
                    } else {
                        Some(JsonToken::LexError {
                            reason: "Expected boolean false value".to_owned(),
                        })
                    }
                }
                '[' => Some(JsonToken::Char(JsonSyntax::LeftBracket)),
                ']' => Some(JsonToken::Char(JsonSyntax::RightBracket)),
                '{' => Some(JsonToken::Char(JsonSyntax::LeftBrace)),
                '}' => Some(JsonToken::Char(JsonSyntax::RightBrace)),
                ':' => Some(JsonToken::Char(JsonSyntax::Colon)),
                ',' => Some(JsonToken::Char(JsonSyntax::Comma)),
                ' ' | '\n' | '\t' | '\r' => self.next(),
                _ => Some(JsonToken::LexError {
                    reason: format!("Found invalid character during lexing: {}", c.into()),
                }),
            }
        } else {
            None
        }
    }
}
