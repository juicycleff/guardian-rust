use crate::common::errors::ApiError;
use crate::common::helpers::respond_json;
use crate::data::models::health_model::HealthResponse;
use actix_web::web::Json;

/// Handler to get the liveness of the service
pub async fn get_health() -> Result<Json<HealthResponse>, ApiError> {
    respond_json(HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        database_status: "ok".into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_get_health() {
        let response = get_health().await.unwrap();
        assert_eq!(response.into_inner().status, "ok".to_string());
    }
}
