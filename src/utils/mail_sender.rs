use crate::prelude::*;

use lettre::message::{Mailbox, MultiPart, SinglePart};
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

use std::fs;

use super::person::SecretFriend;

pub struct MailConfig {
    pub smtp_server: String,
    pub smtp_port: u16,
    pub username: String,
    pub password: String,
}

pub async fn send_secret_friend_email(config: &MailConfig, friend: &SecretFriend) -> Result<()> {
    let template = fs::read_to_string("public/template.html")?;

    let email_content = template
        .replace("{to_name}", &friend.person.name)
        .replace("{secret_friend_name}", &friend.secret_friend.name);

    let from_email = format!("Amigo Secreto dos Rodrigues <{}>", config.username)
        .parse::<Mailbox>()
        .unwrap();
    let to_email = format!("{} <{}>", friend.person.name, friend.person.email)
        .parse::<Mailbox>()
        .unwrap();

    let email = Message::builder()
        .from(from_email)
        .to(to_email)
        .subject("🎁 Seu Amigo Secreto foi Sorteado!")
        .multipart(MultiPart::alternative().singlepart(SinglePart::html(email_content)))
        .unwrap();

    let creds = Credentials::new(config.username.clone(), config.password.clone());

    let mailer = SmtpTransport::starttls_relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email enviado para {}", friend.person.email),
        Err(e) => println!("Erro ao enviar email: {:?}", e),
    }

    Ok(())
}
