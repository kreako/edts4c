use chrono;
use diesel;
use log::error;
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder};
use rocket::Response;
use rocket_contrib::json;
use std::error;
use std::fmt;
use std::io::Cursor;

macro_rules! impl_from_error {
    ($type: ty, $variant: path, $from: ty) => {
        impl From<$from> for $type {
            fn from(e: $from) -> $type {
                $variant(e)
            }
        }
    };
}

/// Error that could happen when processing a request
#[derive(Debug)]
pub enum Error {
    /// Couldn't connect to the database.
    DatabaseConnectionError(diesel::ConnectionError),
    /// Error while running a database request.
    DatabaseRequestError(diesel::result::Error),
    /// Error when parsing a date
    ChronoParseError(chrono::ParseError),
    /// Invalid cycle
    InvalidCycleError,
    /// Logger Error
    LoggerError,
}

impl_from_error!(
    Error,
    Error::DatabaseConnectionError,
    diesel::ConnectionError
);
impl_from_error!(Error, Error::DatabaseRequestError, diesel::result::Error);
impl_from_error!(Error, Error::ChronoParseError, chrono::ParseError);

impl Error {
    /// Returns the HTTP status corresponding to the error
    pub fn status(&self) -> Status {
        Status::InternalServerError
    }

    /// Returns the complementary message.
    pub fn message(&self) -> String {
        match self {
            Error::DatabaseConnectionError(e) => {
                format!("failed to connect to the database: {}", e)
            }
            Error::DatabaseRequestError(e) => format!("request to database failed: {}", e),
            Error::ChronoParseError(e) => format!("Unable to parse a date: {}", e),
            Error::InvalidCycleError => "Invalid cycle should be C1, C2, C3 or C4".to_string(),
            Error::LoggerError => "Internal Logger error".to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}: {}", self.status(), self.message())
    }
}

impl error::Error for Error {}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        error!("Responding with {}", self);
        Ok(Response::build()
            .status(self.status())
            .header(ContentType::JSON)
            .sized_body(Cursor::new(
                json!({
                    "status": self.status().to_string(),
                    "message": self.message(),
                })
                .to_string(),
            ))
            .finalize())
    }
}
