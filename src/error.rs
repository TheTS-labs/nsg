//! [`Nsg`](crate::Nsg) error

use serde::{Deserialize, Serialize};

pub type NsgResult<T> = Result<T, NsgError>;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum NsgError {
    /// Relogin failed due to missing session cookie in response
    NoSessionCookie,
    /// Failed to relogin due to another [`NsgError`]
    FailedToRelogin(Box<NsgError>),
    ReqwestTimeout,
    ReqwestRequestError,
    ReqwestConnectError,
    ReqwestBodyError,
    ReqwestDecodeError,
    ReqwestUnknownError,
    UrlEncodeError,
}

impl From<reqwest::Error> for NsgError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            return NsgError::ReqwestTimeout;
        } else if err.is_request() {
            return NsgError::ReqwestRequestError;
        } else if err.is_connect() {
            return NsgError::ReqwestConnectError;
        } else if err.is_decode() {
            return NsgError::ReqwestDecodeError;
        } else if err.is_body() {
            return NsgError::ReqwestBodyError;
        } else if err.is_decode() {
            return NsgError::ReqwestDecodeError;
        }

        NsgError::ReqwestUnknownError
    }
}
