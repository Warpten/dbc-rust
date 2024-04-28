use std::str::Utf8Error;

pub enum Error {
    String(Utf8Error),
    Malformed,
    Memory,
    Magic
}

impl From<Utf8Error> for Error {
    fn from(e : Utf8Error) -> Self {
        Self::String(e)
    }
}