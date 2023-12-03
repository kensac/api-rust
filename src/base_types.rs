use axum::response::{IntoResponse, Response};
use hyper::StatusCode;

use crate::prisma::PrismaClient;

pub struct BaseError {
    status_code: StatusCode,
    message: String,
}

impl IntoResponse for BaseError {
    fn into_response(self) -> Response {
        (self.status_code, self.message).into_response()
    }
}
impl BaseError {
    pub fn new(status_code: StatusCode, message: String) -> BaseError {
        BaseError {
            status_code,
            message,
        }
    }
}

pub struct BaseResponse<T> {
    status_code: StatusCode,
    data: T,
}

impl<T> IntoResponse for BaseResponse<T>
where
    T: IntoResponse,
{
    fn into_response(self) -> Response {
        (self.status_code, self.data).into_response()
    }
}

impl<T> BaseResponse<T>
where
    T: IntoResponse,
{
    fn _base_response(status_code: StatusCode, data: T) -> BaseResponse<T> {
        BaseResponse { status_code, data }
    }

    pub fn get_response(status_code: StatusCode, data: T) -> BaseResponse<T> {
        BaseResponse { status_code, data }
    }
}

impl BaseResponse<()> {
    pub fn delete_response(status_code: StatusCode) -> BaseResponse<()> {
        BaseResponse {
            status_code,
            data: (),
        }
    }
}

/* impl<T> From<(StatusCode, T)> for BaseResponse<T>
where
    T: IntoResponse,
{
    fn from((status_code, data): (StatusCode, T)) -> BaseResponse<T> {
        BaseResponse { status_code, data }
    }
} */

// Still unsure about which method to use for the responses
pub type StandardResponse<T> = Result<(StatusCode, T), (StatusCode, String)>;
pub type GetResponse<T> = Result<(StatusCode, T), (StatusCode, String)>;
pub type CreateResponse = Result<(StatusCode, ()), (StatusCode, String)>;
pub type DeleteResponse = Result<(StatusCode, ()), (StatusCode, String)>;

pub type UpdateResponse<T> = Result<BaseResponse<T>, BaseError>;

#[derive(Clone)]
pub struct AppState {
    pub client: PrismaClient,
}

impl AppState {
    pub async fn new() -> AppState {
        let client = PrismaClient::_builder()
            .build()
            .await
            .expect("Didn't connect to database");

        AppState { client }
    }
}
