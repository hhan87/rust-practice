use crate::http::request;
use std::str::Utf8Error;
use super::Method;
use super::MethodError;
use std::str;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Formatter, Debug};
pub struct Request{
    path: String,
    query_string: Option<String>,
    method: Method
}

impl TryFrom<&[u8]> for Request{
    type Error = ParseError;
    // GET /search?name=abc HTTP/1.1
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        match str::from_utf8(buf) {
            Ok(request) => {},
            Err(_) => return Err(ParseError::InvalidEncoding),
        }

        match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)){
            Ok(request) =>{},
            Err(e)=> return Err(e)
        }
        let request = str::from_utf8(buf)?;
        match get_next_word(request) {
            Some((method, request)) => {},
            None => return Err(ParseError::InvalidRequest),
        }
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1"{
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        unimplemented!()
    }
}
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl From<MethodError> for ParseError {
    fn from(value: MethodError) -> Self {
        Self::InvalidMethod
    }
}
impl From<Utf8Error> for ParseError {
    fn from(value: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}
impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}
impl Error for ParseError {

}
