use reqwest::StatusCode;
use serde_json::json;
use std::env;

struct User {
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    // Load environment variables
    let api_key = env::var("SENDGRID_API_KEY").unwrap();

    let sender = User {
        name: String::from(""),
        email: String::from(""),
    };
    let recipient = User {
        name: String::from(""),
        email: String::from(""),
    };

    let body = json!({
        "personalizations": [
            {
                "to": [
                    {
                        "email": sender.email,
                        "name": sender.name
                    }
                ]
            }
        ],
        "from": {
            "email": recipient.email,
            "name": recipient.name
          },
        "subject": "Let's Send an Email With Rust and SendGrid",
        "content": [
            {
                "type": "text/html",
                "value": "Here is your <strong>AMAZING</strong> email!"
            },
            {
                "type": "text/plain",
                "value": "Here is your AMAZING email!"
            }
        ]
    });

    // Send the email
    let response = Client::new()
        .post("https://api.sendgrid.com/v3/mail/send")
        .json(&body)
        .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
        .header(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"))
        .send()
        .await
        .unwrap();

    // Handle/Check the response
    match response.status() {
        StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => println!("Email sent!"),
        _ => eprintln!("Unable to send your email"),
    }

    Ok(())
}
