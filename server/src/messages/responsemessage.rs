use rocket::response;
use rocket::response::{Responder, Response};
use rocket::request::{Request};
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};

// wrapper for outgoing message
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseMessage<V, E> {
    pub success: bool,
    pub data: Option<V>,
    pub error: Option<E>,
}

impl<V, E> ResponseMessage<V, E> {
    pub fn success(data: V) -> Self {
        ResponseMessage {
            success: true,
            data: Some(data),
            error: None,
        }
    }
    pub fn error(err: E) -> Self {
        ResponseMessage {
            success: false,
            data: None,
            error: Some(err),
        }
    }
}

impl<'r, V, E> Responder<'r> for ResponseMessage<V, E>
where
    V: Serialize,
    E: Serialize,
{
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(Json(self).respond_to(req).unwrap()).ok()
    }
}