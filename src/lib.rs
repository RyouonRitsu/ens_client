use std::collections::HashMap;
use std::error::Error;

use clap::{Parser, Subcommand};
use serde::Serialize;

use crate::domain::{Event, Response, Subscriber};
use crate::utils::Mode::{Get, Post};
use crate::Commands::*;

mod domain;
mod utils;

#[derive(Parser, Debug)]
#[command(author = "RyouonRitsu <zhouhongxi@bytedance.com>")]
#[command(version)]
#[command(about = "A client program for an event notification server.", long_about = None)]
pub struct Cli {
    /// The root URL of the event notification server.
    #[arg(short, long)]
    addr: String,

    /// The subcommand to execute.
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Serialize, Debug)]
enum Commands {
    /// Publish an event.
    ///
    /// When publishing a notification, it will try to send a notification to all subscribers.
    /// When all sending fails or there are no subscribers, the notification will enter the pending state, waiting for someone to (re)subscribe to it.
    Publish {
        #[arg(short, long)]
        tag: String,
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        message: String,
    },

    /// Subscribe to a tag.
    ///
    /// Subscribing to the notification will add the subscriber's email address to the subscriber list of the notification.
    /// If there is a notification in the pending state, it will try to send the notification to the subscriber immediately.
    Subscribe {
        #[arg(short, long)]
        tag: String,
        #[arg(short, long)]
        email: String,
    },

    /// Unsubscribe from a tag.
    ///
    /// To unsubscribe from a notification, the subscriber's email address will be removed from the notification's subscriber list.
    Unsubscribe {
        #[arg(short, long)]
        tag: String,
        #[arg(short, long)]
        email: String,
        #[arg(short, long)]
        verification_code: String,
    },

    /// Send a verification code to a email address.
    ///
    /// Send a verification code, a verification code will be sent to the specified email address,
    /// which is used for identity verification when unsubscribing from notifications.
    SendVerificationCode {
        #[arg(short, long)]
        email: String,
    },
}

impl Cli {
    /// Parse the command line arguments, and execute the corresponding subcommand.
    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut map = HashMap::new();
        match &self.command {
            Some(Publish { tag, name, message }) => {
                map.insert(String::from("tag"), tag.clone());
                if let Some(name) = name {
                    map.insert(String::from("name"), name.clone());
                }
                map.insert(String::from("message"), message.clone());
                let response = self.exec("/publish", Post(map)).await?;
                Cli::print_response::<Response<Event>>(&response);
                Ok(())
            }
            Some(Subscribe { tag, email }) => {
                map.insert(String::from("tag"), tag.clone());
                map.insert(String::from("email"), email.clone());
                let response = self.exec("/subscribe", Post(map)).await?;
                Cli::print_response::<Response<Subscriber>>(&response);
                Ok(())
            }
            Some(Unsubscribe {
                tag,
                email,
                verification_code,
            }) => {
                map.insert(String::from("tag"), tag.clone());
                map.insert(String::from("email"), email.clone());
                map.insert(String::from("verificationCode"), verification_code.clone());
                let response = self.exec("/unsubscribe", Post(map)).await?;
                println!("{}", response);
                Ok(())
            }
            Some(SendVerificationCode { email }) => {
                map.insert(String::from("email"), email.clone());
                let response = self.exec("/sendVerificationCode", Get(map)).await?;
                println!("{}", response);
                Ok(())
            }
            None => Ok(()),
        }
    }
}
