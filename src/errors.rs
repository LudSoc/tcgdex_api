//! Allow to get error information.

use crate::is_empty::IsEmpty;
use crate::query::Response;
use serde::Deserialize;

/// A `Result` alias where the `Err` case is [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// Error returned by TCGDEX API in some cases of bad request.
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct TcgdexError {
    /// A URL that identifies the problem type.
    #[serde(rename = "type")]
    pub _type: String,

    /// Summary of the problem.
    pub title: String,

    /// The HTTP status code.
    pub status: u16,

    /// Query that cause the problem.
    pub endpoint: String,

    /// Method used for query
    pub method: String,

    /// Lang used.
    #[serde(default)]
    pub lang: String,

    /// Details about the problem.
    #[serde(default)]
    pub details: String,
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
    pub fn is_reqwest(&self) -> bool {
        matches!(self, Self::Reqwest(_))
    }

    /// Returns true if the error is from the TCGDEX API.
    #[must_use]
    pub fn is_tcgdexapi(&self) -> bool {
        matches!(self, Self::TcgdexApi(_))
    }

    /// Returns true if the error is from an empty response.
    #[must_use]
    pub fn is_empty_response(&self) -> bool {
        matches!(self, Self::EmptyResponse)
    }

    /// Returns the TCGDEX error message or None if error is not from TCGDEX.
    #[must_use]
    pub fn get_tcgdex_error(self) -> Option<TcgdexError> {
        match self {
            Self::TcgdexApi(err) => Some(err),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EmptyResponse => writeln!(f, "Response is empty"),
            Error::Reqwest(err) => err.fmt(f),
            Error::TcgdexApi(err) => writeln!(f, "Error type : {}", err._type),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
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
        Response::Error(error) => Err(Error::TcgdexApi(error)),
    }
}
