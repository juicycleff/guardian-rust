#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub database_status: String,
}
