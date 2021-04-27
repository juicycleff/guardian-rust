//! OneTimeCode model crate
use chrono::NaiveDateTime;
use mongodb::bson::serde_helpers::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OneTimeCodeModel {
    #[serde(serialize_with = "serialize_hex_string_as_object_id")]
    pub id: String,
    pub code: String,
    pub expire_at: Option<NaiveDateTime>,
    pub created_by: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_by: Option<String>,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub delete_flag: bool,
}
