use std::collections::HashMap;

use async_trait::async_trait;

use crate::common::errors::ApiError;

/// Generic message option for all email clients
pub struct MessageOption {
    pub to: String,
    pub from: String,
    pub from_name: Option<String>,
    pub content: Option<String>,
    pub content_type: String,
    pub subject: Option<String>,
    pub headers: Option<HashMap<String, String>>,
}

/// Generic email trait for all email integration. This will enable
/// us have a generic interface for all clients
#[async_trait]
pub trait EmailIntegration {
    type Client;

    fn new(&self, api_key: String) -> Self::Client;

    async fn send_message(&self, opt: MessageOption) -> Result<(), ApiError>;
}
