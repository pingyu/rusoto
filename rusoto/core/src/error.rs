use std::error::Error;
use std::fmt;
use std::io;

use crate::credential::CredentialsError;

use super::proto::xml::util::XmlParseError;
use super::request::{BufferedHttpResponse, HttpDispatchError};
use crate::client::SignAndDispatchError;

/// Generic error type returned by all rusoto requests.
#[derive(Debug, PartialEq)]
pub enum RusotoError<E> {
    /// A service-specific error occurred.
    Service(E),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// The endpoint sub-domain has invalid DNS name. (Only S3 service will generate this error)
    InvalidDnsName(InvalidDnsNameError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
    /// An error occurred when attempting to run a future as blocking
    Blocking,
}

/// Result carrying a generic `RusotoError`.
pub type RusotoResult<T, E> = Result<T, RusotoError<E>>;

/// Header used by AWS on responses to identify the request
pub const AWS_REQUEST_ID_HEADER: &str = "x-amzn-requestid";

impl<E> From<XmlParseError> for RusotoError<E> {
    fn from(err: XmlParseError) -> Self {
        let XmlParseError(message) = err;
        RusotoError::ParseError(message)
    }
}

impl<E> From<serde_json::error::Error> for RusotoError<E> {
    fn from(err: serde_json::error::Error) -> Self {
        RusotoError::ParseError(err.to_string())
    }
}

impl<E> From<CredentialsError> for RusotoError<E> {
    fn from(err: CredentialsError) -> Self {
        RusotoError::Credentials(err)
    }
}

impl<E> From<HttpDispatchError> for RusotoError<E> {
    fn from(err: HttpDispatchError) -> Self {
        RusotoError::HttpDispatch(err)
    }
}

impl<E> From<SignAndDispatchError> for RusotoError<E> {
    fn from(err: SignAndDispatchError) -> Self {
        match err {
            SignAndDispatchError::Credentials(e) => Self::from(e),
            SignAndDispatchError::Dispatch(e) => Self::from(e),
        }
    }
}

impl<E> From<io::Error> for RusotoError<E> {
    fn from(err: io::Error) -> Self {
        RusotoError::HttpDispatch(HttpDispatchError::from(err))
    }
}

impl<E: Error + 'static> fmt::Display for RusotoError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RusotoError::Service(ref err) => write!(f, "{}", err),
            RusotoError::Validation(ref cause) => write!(f, "{}", cause),
            RusotoError::Credentials(ref err) => write!(f, "{}", err),
            RusotoError::HttpDispatch(ref dispatch_error) => write!(f, "{}", dispatch_error),
            RusotoError::InvalidDnsName(ref dns_error) => write!(f, "{}", dns_error),
            RusotoError::ParseError(ref cause) => write!(f, "{}", cause),
            RusotoError::Unknown(ref cause) => write!(
                f,
                "Request ID: {:?} Body: {}",
                cause.headers.get(AWS_REQUEST_ID_HEADER),
                cause.body_as_str()
            ),
            RusotoError::Blocking => write!(f, "Failed to run blocking future"),
        }
    }
}

impl<E: Error + 'static> Error for RusotoError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            RusotoError::Service(ref err) => Some(err),
            RusotoError::Credentials(ref err) => Some(err),
            RusotoError::HttpDispatch(ref err) => Some(err),
            _ => None,
        }
    }
}

/// The endpoint sub-domain has invalid DNS name. (Only S3 service will generate this error)
#[derive(Clone, Debug, PartialEq)]
pub struct InvalidDnsNameError {
    message: String,
}

impl InvalidDnsNameError {
    /// Creates a new `InvalidDnsNameError` with the message.
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Error for InvalidDnsNameError {}

impl fmt::Display for InvalidDnsNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<io::Error> for InvalidDnsNameError {
    fn from(err: io::Error) -> InvalidDnsNameError {
        InvalidDnsNameError::new(err.to_string())
    }
}
