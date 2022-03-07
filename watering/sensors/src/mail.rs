use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

use anyhow::Result;

/// sends email using gmails smtp
/// remember to enable less secure apps https://support.google.com/accounts/answer/6010255
pub fn send_alert_mail() -> Result<()> {
    let to = env::var("MAIL_TO")?;
    let smtp_username = env::var("SMTP_USERNAME")?;
    let smtp_password = env::var("SMTP_PASSWORD")?;

    let email = Message::builder()
        .from("rustberrypi <rustberrypibot@gmail.com>".parse()?)
        .reply_to("rustberrypi <rustberrypibot@gmail.com>".parse()?)
        .to(to.parse()?)
        .subject("water empty")
        .body("Be happy!".to_string())?;

    let creds = Credentials::new(smtp_username.to_string(), smtp_password.to_string());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    // Send the email
    mailer.send(&email)?;
    Ok(())
}
