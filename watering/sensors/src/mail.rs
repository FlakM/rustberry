use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use anyhow::Result;

pub fn send_mail(to: &str, smtp_username: &str, smtp_password:&str) -> Result<()> {
    let email = Message::builder()
        .from("rustberrypi <rustberrypibot@gmail.com>".parse().unwrap())
        .reply_to("rustberrypi <rustberrypibot@gmail.com>".parse().unwrap())
        .to(to.parse().unwrap())
        .subject("water empty")
        .body("Be happy!")
        .unwrap();

    let creds = Credentials::new(smtp_username.to_string(), smtp_password.to_string());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    mailer.send(&email)?;
    Ok(())
}
