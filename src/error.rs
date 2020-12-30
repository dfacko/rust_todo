use actix_web::{HttpRequest, HttpResponse, Responder};
use bcrypt::BcryptError;
use diesel::result::Error as DieselError;
use futures::future::{ready, Ready};
use jsonwebtoken::errors::Error as JWTError;
use serde_json::json;
use serde_json::Error as SerdeJsonError;
use serde_json::Value as JsonValue;
use std::convert::From;
use std::error::Error as StdError;
use std::fmt;
use std::net::AddrParseError;

#[derive(Debug, PartialEq)]
pub enum Error {
    Diesel(DieselError),
    Encryption(String),
    Unauthorized(String),
    JsonError(String),
    Other,
    Custom(JsonValue),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Diesel(ref err) => write!(f, "DB Error: {}", err),
            Error::Unauthorized(ref err) => write!(f, "Authorization error: {}", err),
            Error::Encryption(ref err) => write!(f, "Encryption error: {}", err),
            Error::JsonError(ref err) => write!(f, "Parsing JSON error: {}", err),
            Error::Other => write!(f, "Something went wrong"),
            Error::Custom(ref json) => write!(f, "Json validation error {}", json),
        }
    }
}

impl StdError for Error {
    fn cause(&self) -> Option<&dyn StdError> {
        match self {
            Error::Diesel(ref err) => Some(err),
            Error::Unauthorized(ref _err) => None,
            Error::Encryption(ref _err) => None,
            Error::JsonError(ref _err) => None,
            Error::Other => None,
            Error::Custom(ref _json) => None,
        }
    }
}

impl From<DieselError> for Error {
    fn from(err: DieselError) -> Error {
        Error::Diesel(err)
    }
}

impl From<BcryptError> for Error {
    fn from(err: BcryptError) -> Error {
        Error::Encryption(err.to_string())
    }
}

impl From<SerdeJsonError> for Error {
    fn from(err: SerdeJsonError) -> Error {
        Error::JsonError(err.to_string())
    }
}

impl From<JsonValue> for Error {
    fn from(json: JsonValue) -> Error {
        Error::Custom(json)
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
            "EncryptionError" => Error::Encryption(message),
            "JsonError" => Error::JsonError(message),
            _ => Error::Other,
        }
    }

    pub fn to_response(self) -> HttpResponse {
        match self {
            Error::Diesel(ref err) => {
                HttpResponse::InternalServerError().body(format!("DB Error: {}", err))
            }
            Error::Unauthorized(ref err) => {
                HttpResponse::Unauthorized().body(format!("Jwt auth error: {}", err))
            }
            Error::Encryption(ref err) => {
                HttpResponse::InternalServerError().body(format!("Encription error: {}", err))
            }
            Error::Other => {
                HttpResponse::InternalServerError().body(format!("Something went wrong"))
            }
            Error::Custom(json) => {
                HttpResponse::UnprocessableEntity().json(json!({ "message": json }))
            }
            Error::JsonError(err) => {
                HttpResponse::UnprocessableEntity().body(format!("Parsing JSON error: {}", err))
            }
        }
    }
}
