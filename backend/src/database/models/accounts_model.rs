//! Account model crate
use chrono::{DateTime, Utc};
use mongodb::bson::serde_helpers::*;
use serde::de::{MapAccess, Visitor};
use serde::{de, Deserialize, Deserializer, Serialize};
use std::fmt;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::str::FromStr;
use void::Void;

fn string_from_object_id<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: bson::oid::ObjectId = Deserialize::deserialize(deserializer)?;
    let id = s.to_hex();
    Ok(id)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountModel {
    #[serde(
        rename = "_id",
        serialize_with = "serialize_hex_string_as_object_id",
        deserialize_with = "string_from_object_id"
    )]
    pub id: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub password: String,
    pub last_login_at: Option<DateTime<Utc>>,
    pub current_login_at: Option<DateTime<Utc>>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub locked_at: Option<DateTime<Utc>>,
    pub confirmation_sent_at: Option<DateTime<Utc>>,
    pub password_changed_at: Option<DateTime<Utc>>,
    pub remember_created_at: Option<DateTime<Utc>>,
    pub reset_password_created_at: Option<DateTime<Utc>>,
    pub login_count_at: Option<i32>,
    pub failed_attempts: Option<i32>,
    pub reset_password_token: Option<String>,
    pub confirmation_token: Option<String>,
    pub unlock_token: Option<String>,
    pub last_login_ip: Option<String>,
    pub current_login_ip: Option<String>,
    pub unconfirmed_email: Option<String>,
    pub locked: bool,
    pub require_new_password: bool,
    pub created_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_by: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub delete_flag: bool,
}
