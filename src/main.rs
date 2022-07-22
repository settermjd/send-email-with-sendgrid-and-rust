use reqwest::header;
use reqwest::Client;
use serde_json::json;
use std::env;

fn main() ->  Result<(), Err()> {
    struct User {
        name: String,
        email: String
    }

    // Load environment variables

    let api_key = env::var("SENDGRID_API_KEY");
    let auth_token = format!("Bearer {}", api_key.unwrap());

    let sender = User{ name: String::from(""), email: String::from("")};
    let recipient = User{ name: String::from(""), email: String::from("")};
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
    let client = reqwest::blocking::Client::new();
    let response = client.post("https://api.sendgrid.com/v3/mail/send")
        .json(&body)
        .basic_auth("",Some(auth_token))
        .send()?;

    // Handle/Check the response
    match response.status() {
        StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => println!("Email sent!"),
        _ => println!("Unable to send your email"),
    }

    Ok(())
}