use reqwest::header;
use reqwest::StatusCode;
use reqwest::{blocking::Client, Error};
use serde_json::json;
use std::env;

struct User {
    name: String,
    email: String,
}

fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    let api_key = env::var("SENDGRID_API_KEY").unwrap();

    let sender = User {
        name: env::var("SENDER_NAME").unwrap(),
        email: env::var("SENDER_EMAIL").unwrap(),
    };

    let recipient = User {
        name: env::var("RECIPIENT_NAME").unwrap(),
        email: env::var("RECIPIENT_EMAIL").unwrap(),
    };

    // personalizations should not contain a from field, otherwise we get from the API:
    // "The from address does not match a verified Sender Identity. Mail cannot be sent until this error is resolved. 
    // Visit https://sendgrid.com/docs/for-developers/sending-email/sender-identity/ to see the Sender Identity requirements"
    let body = json!(
        {
            "personalizations": [{
                "to": [{
                    "email": recipient.email,
                    "name": recipient.name
                }]
            }],
            "from": {
                "email": sender.email,
                "name": sender.name
            },
            "subject": "Let's Send an Email With Rust and SendGrid",
            "content": [
                {
                    "type": "text/plain",
                    "value": "Here is your AMAZING email!"
                },
                {
                    "type": "text/html",
                    "value": "Here is your <strong>AMAZING</strong> email!"
                },
            ]
        }
    );

    // println!("try to send email with body: {:?}", body.to_string());

    let client = Client::new()
        .post("https://api.sendgrid.com/v3/mail/send")
        .json(&body)
        .bearer_auth(api_key)
        .header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

    let response = client.send()?;

    match response.status() {
        StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => println!("Email sent!"),
        _ => eprintln!(
            "Unable to send your email. Status code was: {}. Body content was: {:?}",
            response.status(),
            response.text()
        ),
    }

    Ok(())
}
