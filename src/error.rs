use actix_web::HttpResponse;
use bcrypt::BcryptError;
use diesel::result::Error as DieselError;
use jsonwebtoken::errors::Error as JWTError;
use serde_json::Error as SerdeJsonError;
use std::convert::From;
use std::error::Error as StdError;
use std::fmt;
use std::net::AddrParseError;

#[derive(Debug, PartialEq)]
pub enum Error {
    Diesel(DieselError),
    Unauthorized(String),
    Other,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Diesel(ref err) => write!(f, "DB Error: {}", err),
            Error::Unauthorized(ref err) => write!(f, "Authorization error: {}", err),
            Error::Other => write!(f, "Something went wrong"),
        }
    }
}

impl StdError for Error {
    fn cause(&self) -> Option<&dyn StdError> {
        match self {
            Error::Diesel(ref err) => Some(err),
            Error::Unauthorized(ref _err) => None,
            Error::Other => None,
        }
    }
}

impl From<DieselError> for Error {
    fn from(err: DieselError) -> Error {
        Error::Diesel(err)
    }
}

impl Error {
    pub fn throw(err: &str, message: Option<&str>) -> Error {
        let message = match message {
            Some(m) => m.to_string(),
            None => "Something went wrong".to_string(),
        };

        match err {
            "Unauthorized" => Error::Unauthorized(message),
            _ => Error::Other,
        }
    }

    pub fn to_response(self) -> HttpResponse {
        match self {
            Error::Diesel(ref err) => {
                HttpResponse::InternalServerError().body(format!("DB Error: {}", err))
            }
        }
    }
}
