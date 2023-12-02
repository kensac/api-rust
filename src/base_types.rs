use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;

use thiserror::Error;
use validator::Validate;

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
    // get rid of exposing this as public function later
    pub fn base_error(status_code: StatusCode, message: String) -> BaseError {
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

pub type StandardResponse<T> = Result<BaseResponse<T>, BaseError>;
pub type DeleteResponse = Result<BaseResponse<()>, BaseError>;
pub type GetResponse<T> = Result<BaseResponse<T>, BaseError>;

// Will migrate to this version of app_state later
#[derive(Clone)]
pub struct AppState {
    pub client: PrismaClient,
}

impl AppState {
    pub async fn get_app_state() -> AppState {
        let client = PrismaClient::_builder()
            .build()
            .await
            .expect("Didn't connect to database");

        AppState { client }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: serde::de::DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = Rejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedJson(value))
    }
}

#[derive(Debug, Error)]
pub enum Rejection {
    #[error(transparent)]
    JsonRejection(JsonRejection),

    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
}

impl IntoResponse for Rejection {
    fn into_response(self) -> Response {
        match self {
            Rejection::ValidationError(_) => {
                let message = format!("Input validation error: [{self}]").replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message)
            }
            Rejection::JsonRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        }
        .into_response()
    }
}

impl From<JsonRejection> for Rejection {
    fn from(e: JsonRejection) -> Self {
        Rejection::JsonRejection(e)
    }
}
