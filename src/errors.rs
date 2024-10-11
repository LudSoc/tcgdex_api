//! Allow to get error information.

use crate::is_empty::IsEmpty;
use crate::query::Response;
use serde::Deserialize;
use thiserror::Error;

/// A `Result` alias where the `Err` case is [`Error`].
pub type Result<T> = std::result::Result<T, ApiError>;

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
#[derive(Debug, Error)]
pub enum ApiError {
    /// Error from reqwest
    #[error("Reqwest error : {}", .0)]
    Reqwest(#[from] reqwest::Error),

    /// Error from TCGDEX API.
    #[error("Tcgdex error : {}", .0.title)]
    TcgdexApi(TcgdexError),

    /// Response is empty.
    #[error("Response is empty")]
    EmptyResponse,
}

impl ApiError {
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

// NOTE: reqwest error cannot be compared.
impl PartialEq for ApiError {
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
                Err(ApiError::EmptyResponse)
            } else {
                Ok(obj)
            }
        }
        Response::Error(error) => Err(ApiError::TcgdexApi(error)),
    }
}
