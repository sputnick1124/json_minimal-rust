use crate::lex::{JsonSyntax, JsonToken, Tokenizable};
use crate::Json;

pub struct JsonStream<I> {
    stream_in: I,
}

pub trait JsonDeser: Iterator {
    fn jsonify(self) -> JsonStream<Self>
    where
        Self: Sized,
    {
        JsonStream { stream_in: self }
    }
}

impl<T: ?Sized> JsonDeser for T where T: Iterator {}

impl<I> Iterator for JsonStream<I>
where
    I: Tokenizable<Item = JsonToken>,
{
    type Item = Json;
    fn next(&mut self) -> Option<Self::Item> {
        self.parse().ok()
    }
}

impl<I> JsonStream<I>
where
    I: Tokenizable<Item = JsonToken>,
{
    fn parse_object(&mut self) -> Result<Json, ()> {
        let mut json_vec = Vec::new();
        loop {
            if let Some(JsonToken::String(s)) = self.stream_in.next() {
                if let Some(JsonToken::Char(JsonSyntax::Colon)) = self.stream_in.next() {
                    json_vec.push(Json::OBJECT {
                        name: s,
                        value: Box::new(self.parse()?),
                    })
                } else {
                    return Err(());
                }
            } else {
                return Ok(Json::JSON(json_vec));
            }

            match self.stream_in.next() {
                Some(JsonToken::Char(JsonSyntax::RightBrace)) => {
                    return Ok(Json::JSON(json_vec));
                }
                Some(JsonToken::Char(JsonSyntax::Comma)) => {
                    continue;
                }
                _ => {
                    return Err(());
                }
            }
        }
    }

    fn parse_array(&mut self) -> Result<Json, ()> {
        let mut json_array = Vec::new();
        loop {
            json_array.push(self.parse()?);

            match self.stream_in.next() {
                Some(JsonToken::Char(JsonSyntax::RightBracket)) => {
                    return Ok(Json::ARRAY(json_array));
                }
                Some(JsonToken::Char(JsonSyntax::Comma)) => {
                    continue;
                }
                _ => {
                    return Err(());
                }
            }
        }
    }

    fn parse(&mut self) -> Result<Json, ()> {
        if let Some(token) = self.stream_in.next() {
            match token {
                JsonToken::Char(JsonSyntax::LeftBrace) => self.parse_object(),
                JsonToken::Char(JsonSyntax::LeftBracket) => self.parse_array(),
                JsonToken::Number(n) => Ok(Json::NUMBER(n)),
                JsonToken::String(s) => Ok(Json::STRING(s)),
                JsonToken::Bool(b) => Ok(Json::BOOL(b)),
                JsonToken::Null => Ok(Json::NULL),
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}
