use actix_web::dev::HttpResponseBuilder;
use actix_web::{
    error::{BlockingError, ResponseError},
    http::StatusCode,
    HttpResponse,
};
use celery::error::CeleryError;
use derive_more::Display;
use futures::task::SpawnError;
use mongodb::error::Error as MongoError;
use r2d2::Error as PoolError;
use uuid::Error as ParseError;

#[derive(Debug, Display, PartialEq)]
#[allow(dead_code)]
pub enum ApiError {
    BadRequest(String),
    BlockingError(String),
    CacheError(String),
    CannotDecodeJwtToken(String),
    CannotEncodeJwtToken(String),
    InternalServerError(String),
    NotFound(String),
    Conflict(String),
    RequestTimeout(String),
    Gone(String),
    PaymentRequired(String),
    PayloadTooLarge(String),
    TooManyRequests(String),
    DatabaseError(String),
    ParseError(String),
    PoolError(String),
    #[display(fmt = "")]
    ValidationError(Vec<String>),
    Unauthorized(String),
}

/// User-friendly error messages
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    errors: Vec<String>,
}

/// Automatically convert ApiErrors to external Response Errors
impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::BadRequest(_error) => StatusCode::BAD_REQUEST,
            ApiError::NotFound(_message) => StatusCode::NOT_FOUND,
            ApiError::ValidationError(_errors) => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::Unauthorized(_error) => StatusCode::UNAUTHORIZED,
            ApiError::Conflict(_message) => StatusCode::CONFLICT,
            ApiError::Gone(_errors) => StatusCode::GONE,
            ApiError::PaymentRequired(_error) => StatusCode::PAYMENT_REQUIRED,
            ApiError::PayloadTooLarge(_error) => StatusCode::PAYLOAD_TOO_LARGE,
            ApiError::TooManyRequests(_error) => StatusCode::TOO_MANY_REQUESTS,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::BadRequest(error) => {
                HttpResponseBuilder::new(self.status_code()).body::<String>(error.into())
                // HttpResponse::BadRequest().json(error.into())
            }
            ApiError::NotFound(message) => {
                HttpResponseBuilder::new(self.status_code()).body::<String>(message.into())
                // HttpResponse::NotFound().json(message.into())
            }
            ApiError::ValidationError(errors) => {
                HttpResponseBuilder::new(self.status_code()).json(errors.to_vec())
                // HttpResponse::UnprocessableEntity().json(errors.to_vec().into())
            }
            ApiError::Unauthorized(error) => {
                HttpResponseBuilder::new(self.status_code()).body::<String>(error.into())
            }
            ApiError::Conflict(message) => {
                HttpResponseBuilder::new(self.status_code()).body::<String>(message.into())
                // HttpResponse::Conflict().json(message.into())
            }
            ApiError::Gone(errors) => {
                HttpResponseBuilder::new(self.status_code()).body::<String>(errors.into())
                // HttpResponse::Gone().json(errors.into())
            }
            ApiError::PaymentRequired(error) => {
                HttpResponseBuilder::new(self.status_code()).body::<String>(error.into())
                // HttpResponse::PaymentRequired().json(error.into())
            }
            ApiError::PayloadTooLarge(error) => {
                HttpResponseBuilder::new(self.status_code()).body::<String>(error.into())
                // HttpResponse::PayloadTooLarge().json(error.into())
            }
            ApiError::TooManyRequests(error) => {
                HttpResponseBuilder::new(self.status_code()).body::<String>(error.into())
                // HttpResponse::TooManyRequests().json(error.into())
            }
            ApiError::DatabaseError(error) => {
                HttpResponseBuilder::new(self.status_code()).body::<String>(error.into())
                // HttpResponse::TooManyRequests().json(error.into())
            }
            _ => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

/// Utility to make transforming a string reference into an ErrorResponse
impl From<&String> for ErrorResponse {
    fn from(error: &String) -> Self {
        ErrorResponse {
            errors: vec![error.into()],
        }
    }
}

/// Utility to make transforming a vector of strings into an ErrorResponse
impl From<Vec<String>> for ErrorResponse {
    fn from(errors: Vec<String>) -> Self {
        ErrorResponse { errors }
    }
}

/// Convert PoolErrors to ApiErrors
impl From<PoolError> for ApiError {
    fn from(error: PoolError) -> ApiError {
        ApiError::PoolError(error.to_string())
    }
}

/// Convert std::io::Error to ApiErrors
impl From<std::io::Error> for ApiError {
    fn from(error: std::io::Error) -> ApiError {
        ApiError::InternalServerError(error.to_string())
    }
}

/// Convert std::io::Error to ApiErrors
impl From<serde_yaml::Error> for ApiError {
    fn from(error: serde_yaml::Error) -> ApiError {
        ApiError::InternalServerError(error.to_string())
    }
}

/// Convert ParseErrors to ApiErrors
impl From<ParseError> for ApiError {
    fn from(error: ParseError) -> ApiError {
        ApiError::ParseError(error.to_string())
    }
}

/// Convert Thread BlockingErrors to ApiErrors
impl From<BlockingError> for ApiError {
    fn from(error: BlockingError) -> ApiError {
        ApiError::BlockingError(error.to_string())
    }
}

/// Convert Mongo Error to ApiErrors
impl From<MongoError> for ApiError {
    fn from(error: MongoError) -> ApiError {
        ApiError::DatabaseError(error.to_string())
    }
}

/// Convert Mongo Error to ApiErrors
impl From<mongodb::bson::de::Error> for ApiError {
    fn from(error: mongodb::bson::de::Error) -> ApiError {
        ApiError::DatabaseError(error.to_string())
    }
}

/// Convert Bson Ser Error to ApiErrors
impl From<mongodb::bson::ser::Error> for ApiError {
    fn from(error: mongodb::bson::ser::Error) -> ApiError {
        ApiError::DatabaseError(error.to_string())
    }
}

/// Convert Pool Error to ApiErrors
impl From<SpawnError> for ApiError {
    fn from(error: SpawnError) -> ApiError {
        ApiError::InternalServerError(error.to_string())
    }
}

/// Convert CeleryError Error to ApiErrors
impl From<CeleryError> for ApiError {
    fn from(error: CeleryError) -> ApiError {
        ApiError::InternalServerError(error.to_string())
    }
}
