use crate::domain::SubscriberEmail;
use reqwest::Url;

pub struct EmailClient {
    http_client: reqwest::Client,
    base_url: Url,
    sender: SubscriberEmail,
    authorization_token: String,
}

impl EmailClient {
    pub fn new(base_url: Url, sender: SubscriberEmail, authorization_token: String) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            base_url,
            sender,
            authorization_token,
        }
    }

    pub async fn send_email(
        &self,
        recipient: &SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), reqwest::Error> {
        let url = self.base_url.join("email").unwrap();
        let request_body = SendEmailRequest {
            from: self.sender.as_ref().to_string(),
            to: recipient.as_ref().to_string(),
            subject: subject.to_string(),
            html_content: html_content.to_string(),
            text_content: text_content.to_string(),
        };
        let _builder = self
            .http_client
            .post(url)
            .header("X-Postmark-Server-Token", &self.authorization_token)
            .json(&request_body)
            .send()
            .await?;
        Ok(())
    }
}

#[derive(serde::Serialize)]
struct SendEmailRequest {
    from: String,
    to: String,
    subject: String,
    html_content: String,
    text_content: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::{Fake, Faker};
    use wiremock::matchers::any;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn send_email_fires_a_request_to_base_url() {
        // arange
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let url = Url::parse(&mock_server.uri()).unwrap();
        let email_client = EmailClient::new(url, sender, Faker.fake());

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let subscriber_email = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();
        // act
        let _ = email_client
            .send_email(&subscriber_email, &subject, &content, &content)
            .await;
        // assert
    }
}
