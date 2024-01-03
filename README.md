# send-email-with-sendgrid-and-rust

A small project showing how to send email with SendGrid and Rust

## Preparation

Add a local .env file in base project folder with the following variables:

```text
SENDGRID_API_KEY=""
SENDER_NAME=""
SENDER_EMAIL=""
RECIPIENT_NAME=""
RECIPIENT_EMAIL=""
```

## Run with

```bash
cargo run
```

## Notes

20240103 Update json body to fix API "from" field error (should be updated on <https://www.twilio.com/blog/send-emails-rust-sendgrid>, too)
