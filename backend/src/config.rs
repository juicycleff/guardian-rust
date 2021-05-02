use std::env;
use std::fs::File;
use std::io::Read;

use json_dotpath::DotPaths;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use voca_rs::chop;

use crate::common::utils::serde_utils::merge_json_and_yaml;
use sentry::types::Dsn;
use std::borrow::Cow;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum AppEnvironment {
    Production,
    Development,
}

impl Default for AppEnvironment {
    fn default() -> Self {
        AppEnvironment::Development
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum AuthSecurityLevel {
    Simple,
}

impl Default for AuthSecurityLevel {
    fn default() -> Self {
        AuthSecurityLevel::Simple
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Config {
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub environment: AppEnvironment,
    #[serde(rename = "ssl_address")]
    pub ssl_address: String,
    pub datastore: DatastoreConfig,
    pub security: SecurityConfig,
    pub features: FeaturesConfig,
    pub logging: LoggingConfig,
    pub integration: IntegrationConfig,
    pub files: FilesConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatastoreConfig {
    #[serde(rename = "db_url")]
    pub db_url: String,
    #[serde(rename = "db_name")]
    pub db_name: String,
    #[serde(rename = "redis_url")]
    pub redis_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilesConfig {
    #[serde(rename = "security_cert")]
    pub security_cert: String,
    #[serde(rename = "security_key")]
    pub security_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityConfig {
    #[serde(rename = "auth_salt")]
    pub auth_salt: String,
    #[serde(default = "Default::default")]
    pub session_path: String,
    #[serde(rename = "jwt_expiration")]
    pub jwt_expiration: i64,
    #[serde(rename = "jwt_key")]
    pub jwt_key: String,

    pub jwt_issuer: String,

    #[serde(rename = "session_key")]
    pub session_key: String,
    #[serde(rename = "session_name")]
    pub session_name: String,
    #[serde(rename = "session_secure")]
    pub session_secure: bool,
    #[serde(rename = "session_max_age_secs")]
    pub session_timeout: i64,
    #[serde(rename = "password_strength")]
    pub password_strength: i64,
    #[serde(rename = "onetime_code_duration")]
    pub onetime_code_duration: i32,
    #[serde(rename = "onetime_code_length")]
    pub onetime_code_length: u8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthFeaturesConfig {
    #[serde(rename = "enable_signup")]
    pub enable_signup: bool,
    #[serde(rename = "enable_login")]
    pub enable_login: bool,
    #[serde(rename = "login_require_confirmation")]
    pub login_require_confirmation: bool,
    #[serde(rename = "login_with_signup")]
    pub login_with_signup: bool,
    #[serde(rename = "security_level")]
    pub security_level: AuthSecurityLevel,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiFeaturesConfig {
    pub enable_graphql: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeaturesConfig {
    pub auth: AuthFeaturesConfig,
    #[serde(default = "Default::default")]
    pub api: ApiFeaturesConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub sentry: SentryLoggingConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SentryLoggingConfig {
    pub dns: Option<Dsn>,
    pub environment: Option<Cow<'static, str>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntegrationConfig {
    pub sendgrid: SendgridIntegrationConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendgridIntegrationConfig {
    #[serde(rename = "api_key")]
    pub api_key: String,
}

pub fn read_file(filename: String) -> Option<String> {
    let mut file_handle = File::open(filename).expect("config file not found");
    let mut content = String::new();
    match file_handle.read_to_string(&mut content) {
        Ok(_) => Some(content),
        Err(_) => None,
    }
}

fn parse_json(json_value: Value) -> Option<Config> {
    let parsed = serde_json::from_value::<Config>(json_value);
    match parsed {
        Ok(value) => Some(value),
        Err(error) => panic!("app config parse error: {:#?}", error),
    }
}

fn read_env_vars(prefix: &str, split_by: Option<&str>) -> Value {
    let mut obj = Value::Null;

    for (key, value) in env::vars() {
        if key.starts_with(prefix) {
            let actual_key = chop::removeprefix(key.as_str(), prefix);
            let keys = actual_key
                .as_str()
                .replace(split_by.unwrap_or(""), ".")
                .to_lowercase();
            obj.dot_set(&*keys, value).unwrap();
        }
    }
    obj
}

fn load_config_file(filename: Option<String>) -> Config {
    #[cfg(not(test))]
    let config_path = env::var("GUARDIAN_CONFIG_PATH")
        .unwrap_or_else(|_| "./backend/config/config.yaml".to_string());

    #[cfg(test)]
    let config_path =
        env::var("GUARDIAN_CONFIG_PATH").unwrap_or_else(|_| "./config/config.yaml".to_string());

    let mut env_value = read_env_vars("APP_", Some("__"));

    let real_path = config_path;
    let content = match read_file(filename.unwrap_or(real_path)) {
        Some(c) => c,
        None => panic!("error: can't read config file"),
    };

    merge_json_and_yaml(&mut env_value, content);
    match parse_json(env_value) {
        Some(config) => config,
        None => panic!("error parsing configuration"),
    }
}

// Throw the Config struct into a CONFIG lazy_static to avoid multiple processing
lazy_static! {
    pub static ref CONFIG: Config = load_config_file(None);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_a_config() {
        let config = load_config_file(None);
        assert_ne!(config.address, "".to_string());
    }

    #[test]
    fn it_gets_a_config_from_the_lazy_static() {
        let config = &CONFIG;
        assert_ne!(config.address, "".to_string());
    }
}
