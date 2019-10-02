use rocket::response;
use rocket::response::{Responder, Response};
use rocket::request::{Request};
use rocket_contrib::json::Json;
use serde::{Serialize};
use super::responsemessage::ResponseMessage;

#[derive(Debug)]
pub struct RespResult<V> {
    data: V,
}

impl<V> RespResult<V> {
    pub fn new(val: V) -> Self {
        RespResult { data: val }
    }
}

impl<'r, V> Responder<'r> for RespResult<V>
where
    V: Serialize,
{
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        let msg = ResponseMessage::<V, ()> {
            success: true,
            data: Some(self.data),
            error: None,
        };
        Response::build_from(Json(msg).respond_to(req).unwrap()).ok()
    }
}