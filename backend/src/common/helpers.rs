use crate::common::errors::ApiError;
#[cfg(feature = "actix")]
use actix_web::{
    body::Body,
    web::{HttpResponse, Json},
};
use serde::Serialize;

// Just a generic Result type to ease error handling for us. Errors in multithreaded
// async contexts needs some extra restrictions
pub type AppResult<T> = std::result::Result<T, ApiError>;

#[allow(unused)]
#[cfg(feature = "actix")]
/// Helper function to reduce boilerplate of an OK/Json response
pub fn respond_json<T>(data: T) -> Result<Json<T>, ApiError>
where
    T: Serialize,
{
    Ok(Json(data))
}

#[allow(unused)]
#[cfg(feature = "actix")]
/// Helper function to reduce boilerplate of an empty OK response
pub fn respond_ok() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().body(Body::Empty))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
    pub struct TestResponse {
        pub first_name: String,
    }

    #[cfg(feature = "actix")]
    #[test]
    fn it_responds_json() {
        let response = TestResponse {
            first_name: "matoshi".into(),
        };
        let result = respond_json(response.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().into_inner(), response);
    }

    #[cfg(feature = "actix")]
    #[test]
    fn it_responds_ok() {
        let result = respond_ok();
        assert!(result.is_ok());
    }
}
