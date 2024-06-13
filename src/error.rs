use serde::{Deserialize, Serialize};

pub type NsgResult<T> = Result<T, NsgError>;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum NsgError {
    /// Relogin failed due to missing session cookie in response
    NoSessionCookie,
    /// Request failed due to timeout
    ReqwestTimeout,
    /// Request failed due to request error
    ReqwestRequestError,
    /// Request failed due to connect error
    ReqwestConnectError,
    /// Request failed due to request or response body error
    ReqwestBodyError,
    /// Request failed due to decoding response's body error
    ReqwestDecodeError,
    /// Request failed due to unknown error
    ReqwestUnknownError,
    /// Failed to url encode payload
    UrlEncodeError,
    /// Failed to relogin
    FailedToRelogin(Box<NsgError>),
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
