use rocket::http::Status;
use rocket::response;
use rocket::response::{Responder, Response};
use rocket::request::{Request};
use rocket_contrib::json::Json;
use serde::{Serialize};
use super::responsemessage::ResponseMessage;

#[derive(Debug)]
pub struct RespError<E> {
    error: E,
    status_code: Status,
}

impl From<&str> for RespError<String> {
    fn from(msg: &str) -> Self {
        RespError::<String>::new_str(msg)
    }
}
impl From<String> for RespError<String> {
    fn from(msg: String) -> Self {
        RespError::<String>::new_string(msg)
    }
}
impl From<diesel::result::Error> for RespError<String> {
    fn from(err: diesel::result::Error) -> Self {
        RespError::<String>::new_string(format!("database error: {}", err))
    }
}

impl<E> RespError<E> {
    pub fn new(err: E) -> Self {
        RespError {
            error: err,
            status_code: Status::InternalServerError,
        }
    }
    pub fn new_str(msg: &str) -> RespError<String> {
        RespError {
            error: msg.to_string(),
            status_code: Status::InternalServerError,
        }
    }
    pub fn new_string(msg: String) -> RespError<String> {
        RespError {
            error: msg,
            status_code: Status::InternalServerError,
        }
    }
    pub fn with_status(&mut self, code: Status) -> &mut Self {
        self.status_code = code;
        self
    }
}

// trait implementations
impl<'r, E> Responder<'r> for RespError<E>
where
    E: Serialize,
{
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        let msg = ResponseMessage::<(), E> {
            success: false,
            data: None,
            error: Some(self.error),
        };
        Response::build_from(Json(msg).respond_to(req).unwrap())
            .status(self.status_code)
            .ok()
    }
}
