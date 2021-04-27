use crate::common::errors::ApiError;
use async_trait::async_trait;
use std::collections::HashMap;

/// Generic message option for all email clients
pub struct MessageOption {
    to: String,
    from: String,
    from_name: Option<String>,
    content: Option<String>,
    content_type: String,
    subject: Option<String>,
    headers: Option<HashMap<String, String>>,
}

/// Generic email trait for all email clients. This will enable
/// us have a generic interface for all clients
#[async_trait]
pub trait EmailClient {
    type Client;

    fn new(&self, api_key: String) -> Self::Client;

    async fn send_message(&self, opt: MessageOption) -> Result<(), ApiError>;
}

/// Sendgrid email client
#[cfg(sendgrid_email)]
pub struct SendgridEmailClient {
    client: v3::Sender,
}

#[cfg(sendgrid_email)]
#[async_trait]
impl EmailClient for SendgridEmailClient {
    type Client = SendgridEmailClient;

    /// Initialize a new instance of `SendgridEmailClient` which contains a `sendgrid` client
    fn new(&self, api_key: String) -> SendgridEmailClient {
        SendgridEmailClient {
            client: v3::Sender::new(api_key),
        }
    }

    /// Helper method that sends method using a generic interface
    async fn send_message(&self, opt: MessageOption) -> Result<(), ApiError> {
        use sendgrid::v3::*;

        let p = Personalization::new(Email::new(opt.to));
        if opt.headers.is_some() {
            p.add_headers(opt.headers?)
        }

        let message = v3::Message::new(Email::new(opt.from));
        message.add_personalization(p);

        if opt.subject.is_some() {
            message.set_subject(opt.message.unwrap().as_str());
        }

        if opt.content.is_some() {
            let c = Content::new()
                .set_content_type(opt.content_type)
                .set_value(opt.content?);

            message.add_content(c);
        }

        self.client.send(&message).await
    }
}
