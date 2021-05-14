use async_trait::async_trait;

use crate::common::helpers::AppResult;
use crate::integrations::email::email_integration::{EmailIntegration, MessageOption};

/// Sendgrid email client
#[cfg(sendgrid_integration)]
pub struct SendgridEmailIntegration {
    client: sendgrid::v3::Sender,
}

#[cfg(sendgrid_integration)]
#[async_trait]
impl EmailIntegration for SendgridEmailIntegration {
    type Client = SendgridEmailIntegration;

    /// Initialize a new instance of `SendgridEmailClient` which contains a `sendgrid` client
    fn new(&self, api_key: String) -> SendgridEmailIntegration {
        SendgridEmailIntegration {
            client: sendgrid::v3::Sender::new(api_key),
        }
    }

    /// Helper method that sends method using a generic interface
    async fn send_message(&self, opt: MessageOption) -> AppResult<()> {
        use sendgrid::v3::*;

        let p = Personalization::new(Email::new(opt.to));
        if opt.headers.is_some() {
            p.add_headers(opt.headers?)
        }

        let message = sendgrid::v3::Message::new(Email::new(opt.from));
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
