pub mod average;
#[macro_use]
pub mod macros;

use std::collections::HashMap;

use async_trait::async_trait;
use tokio::sync::mpsc::UnboundedReceiver;
use twitch_irc::message::ServerMessage;

use crate::{
    commands::average::Average,
    state::State,
};

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("{}", _0)]
    Server(#[from] anyhow::Error),

    #[error("{}", _0)]
    Client(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArgumentLength {
    Fixed(u32),
    Flexible,
}

#[async_trait]
pub trait Command: Send {
    fn command_prefix(&self) -> &'static str;

    fn length(&self) -> ArgumentLength;

    async fn invoke(&self, state: State, author: &str, args: &[&str]) -> Result<(), CommandError>;
}

pub async fn listen_to_commands(
    mut incoming_messages: UnboundedReceiver<ServerMessage>,
    state: State,
) -> anyhow::Result<()> {
    let command_config = &state.config.command_config;

    let commands = commands![
        command_config.average_command.enabled => Average,
    ];

    let command_map = create_command_map(commands);

    while let Some(server_message) = incoming_messages.recv().await {
        if let ServerMessage::Privmsg(msg) = server_message {
            // there will sometimes be a `\u{e0000}` character at the end.
            // this character is NOT from twitch but rather add-ons (like 7tv)
            // to bypass twitch's anti spam (for duplicate messages)
            let split = msg.message_text.split(" ").collect::<Vec<_>>();

            let Some(sent_command) = split.first() else {
                continue;
            };

            let Some(command) = command_map.get(sent_command) else {
                continue;
            };

            let args = &split[1..];

            if let ArgumentLength::Fixed(num) = command.length()
                && args.len() as u32 != num
            {
                state
                    .client
                    .say(
                        state.channel_name.to_string(),
                        format!("This command should have {num} arguments!"),
                    )
                    .await?;
                continue;
            }

            if let Err(err) = command.invoke(state.clone(), &msg.sender.name, args).await {
                match err {
                    CommandError::Server(error) => {
                        tracing::error!(?error);
                        return Err(error);
                    }
                    CommandError::Client(message) => {
                        state
                            .client
                            .say(state.channel_name.to_string(), message)
                            .await?;
                    }
                }
            }
        }
    }

    Ok(())
}

fn create_command_map(commands: Vec<Box<dyn Command>>) -> HashMap<&'static str, Box<dyn Command>> {
    let mut map = HashMap::new();

    for command in commands {
        map.insert(command.command_prefix(), command);
    }

    map
}
