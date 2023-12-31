use std::sync::{Arc, OnceLock};

use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use socketioxide::SocketIo;
use utoipa::ToSchema;

use crate::prisma::PrismaClient;

#[derive(ToSchema)]
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
    pub fn new(status_code: StatusCode, message: String) -> Self {
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
    fn _new(status_code: StatusCode, data: T) -> BaseResponse<T> {
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

type _StandardResponse<T> = Result<(StatusCode, T), (StatusCode, String)>;
pub type GetResponse<T> = Result<(StatusCode, T), (StatusCode, String)>;
pub type CreateResponse = Result<(StatusCode, ()), (StatusCode, String)>;
pub type DeleteResponse = Result<(StatusCode, ()), (StatusCode, String)>;
pub type UpdateResponse = Result<(StatusCode, ()), (StatusCode, String)>;

/* pub type UpdateResponse<T> = Result<BaseResponse<T>, BaseError>; */

#[derive(Clone, Debug)]
pub struct AppState {
    pub client: Arc<PrismaClient>,
    pub reqwest_client: reqwest::Client,
    pub io: Arc<SocketIo>,
}

impl AppState {
    pub async fn new(socket: SocketIo) -> AppState {
        let client = PrismaClient::_builder()
            .build()
            .await
            .expect("Didn't connect to database");

        AppState {
            client: Arc::new(client),
            reqwest_client: reqwest::Client::new(),
            io: Arc::new(socket),
        }
    }
}

pub static APP_STATE: OnceLock<AppState> = OnceLock::new();
