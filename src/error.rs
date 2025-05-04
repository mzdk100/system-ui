use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    str::Utf8Error,
    sync::PoisonError,
};

#[derive(Debug)]
pub enum UiError {
    Init(String),
    Poison(String),
    Utf8(Utf8Error),
}

impl Display for UiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "UiError: ")?;
        match self {
            Self::Init(e) => write!(f, "InitError: {}", e),
            Self::Poison(e) => Display::fmt(e, f),
            Self::Utf8(e) => Display::fmt(e, f),
        }
    }
}

impl Error for UiError {}

impl From<Utf8Error> for UiError {
    fn from(value: Utf8Error) -> Self {
        Self::Utf8(value)
    }
}

impl<T> From<PoisonError<T>> for UiError {
    fn from(value: PoisonError<T>) -> Self {
        Self::Poison(value.to_string())
    }
}
