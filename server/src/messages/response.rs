use rocket::http::Status;
use super::respresult::*;
use super::resperror::*;

// generic api response with success and error type
pub type ApiResponse<V, E> = std::result::Result<RespResult<V>, RespError<E>>;


// utility functions
pub fn respond_success<V, E>(data: V) -> ApiResponse<V, E> {
    ApiResponse::Ok(RespResult::new(data))
}

pub fn respond_error<V, E>(err: E) -> ApiResponse<V, E> {
    ApiResponse::Err(RespError::new(err))
}
pub fn respond_error_str<V, E>(err: &str) -> ApiResponse<V, String> {
    ApiResponse::Err(RespError::<String>::new_str(err))
}
pub fn respond_error_status<V, E>(err: E, status: Status) -> ApiResponse<V, E> {
    let mut err = RespError::new(err);
    err.with_status(status);
    ApiResponse::Err(err)
}
pub fn respond_error_str_status<V, E>(err: &str, status: Status) -> ApiResponse<V, String> {
    let mut err = RespError::<String>::new_str(err);
    err.with_status(status);
    ApiResponse::Err(err)
}
pub fn respond_unauthorized<V>() -> ApiResponse<V, String> {
    respond_error_str_status::<V, String>("unauthorized", Status::Unauthorized)
}
