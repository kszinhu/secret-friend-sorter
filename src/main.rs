include!("lib.rs");

use prelude::*;
use utils::{
    mail_sender::{send_secret_friend_email, MailConfig},
    person::Person,
    sorter::sort_secret_friends,
};

use dotenv::dotenv;
use serde_json;
use std::{env, fs};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let mail_config = MailConfig {
        smtp_server: env::var("SMTP_SERVER").expect("SMTP_SERVER must be set"),
        username: env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set"),
        password: env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set"),
        smtp_port: env::var("SMTP_PORT")
            .expect("SMTP_PORT must be set")
            .parse()
            .expect("SMTP_PORT must be a number"),
    };

    let json_content = fs::read_to_string("friends.json")?;

    let people: Vec<Person> = serde_json::from_str(&json_content)?;

    match sort_secret_friends(
        people,
        Some(
            env::var("AMOUNT")
                .unwrap_or("1".into())
                .parse()
                .expect("Invalid AMOUNT"),
        ),
    ) {
        Ok(secret_friends) => {
            for secret_friend in secret_friends {
                println!(
                    "Send mail to \"{}\" with your secret friend",
                    secret_friend.person.email
                );

                send_secret_friend_email(&mail_config, &secret_friend).await?;
            }
        }
        Err(e) => println!("Erro: {:?}", e),
    }

    Ok(())
}
