//! Allow to get error information.

use crate::is_empty::IsEmpty;
use crate::query::Response;
use serde::Deserialize;

/// A `Result` alias where the `Err` case is [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// Error returned by TCGDEX API in some cases of bad request.
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct TcgdexError {
    /// Error details.
    pub message: String,
}

/// The errors that may occur.
#[derive(Debug)]
pub enum Error {
    /// Error from reqwest
    Reqwest(reqwest::Error),

    /// Error from TCGDEX API.
    TcgdexApi(TcgdexError),

    /// Response is empty.
    EmptyResponse,
}

impl Error {
    /// Returns true if the error is from reqwest
    #[must_use]
    pub const fn is_reqwest(&self) -> bool {
        matches!(self, Self::Reqwest(_))
    }

    /// Returns true if the error is from the TCGDEX API.
    #[must_use]
    pub const fn is_tcgdexapi(&self) -> bool {
        matches!(self, Self::TcgdexApi(_))
    }

    /// Returns true if the error is from an empty response.
    #[must_use]
    pub const fn is_empty_response(&self) -> bool {
        matches!(self, Self::EmptyResponse)
    }

    /// Returns the TCGDEX error message or None if error is not from TCGDEX.
    #[must_use]
    pub fn get_tcgdex_message(&self) -> Option<&str> {
        match &self {
            Self::TcgdexApi(err) => Some(err.message.as_str()),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    /// Construct an error from a reqwest error
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}

// NOTE: reqwest error cannot be compared.
impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::TcgdexApi(a), Self::TcgdexApi(b)) => a == b,
            (Self::EmptyResponse, Self::EmptyResponse) => true,
            _ => false,
        }
    }
}

pub(crate) fn set_error<T>(response: Response<T>) -> Result<T>
where
    T: IsEmpty,
{
    match response {
        Response::Data(obj) => {
            if obj.is_empty() {
                Err(Error::EmptyResponse)
            } else {
                Ok(obj)
            }
        }
        Response::Error { error } => Err(Error::TcgdexApi(error)),
    }
}
