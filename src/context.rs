use regex::Regex;

use crate::error::Error;
use crate::message::{ChatroomType, MessageChain, Sender, SingleMessage};
use crate::{Bot, Result};

#[allow(dead_code)]
pub struct Context {
    bot: Bot,

    is_at_me: bool,

    is_command: bool,
    command_name: String,

    chatroom_type: ChatroomType,
    chatroom_id: i64,
    chatroom_name: String,

    sender_id: i64,
    sender_nickname: String,

    message_id: i64,
    message_chain: MessageChain,
}

impl Context {
    pub fn new<S>(bot: Bot, sender: S, mut message_chain: MessageChain) -> Result<Self>
    where
        S: Sender,
    {
        let source_message = message_chain[0].clone();

        let message_id = match source_message {
            SingleMessage::Source { id, .. } => id,
            _ => {
                return Err(Error::new(
                    "[Error] Receiving error message type when creating context.",
                ))
            }
        };

        // remove the source message
        message_chain.remove(0);

        let chatroom_type = sender.chatroom_type();

        let is_at_me = match chatroom_type {
            ChatroomType::Friend => false,
            ChatroomType::Group => match message_chain[0] {
                SingleMessage::At { target, .. } => target == bot.qq(),
                _ => false,
            },
        };

        // remove the at message
        if is_at_me {
            message_chain.remove(0);
        }

        let is_command = match chatroom_type {
            ChatroomType::Friend => match &message_chain[0] {
                SingleMessage::Plain { text } => text.as_str().trim().starts_with("/"),
                _ => false,
            },
            ChatroomType::Group => {
                is_at_me
                    // the second `SingleMessage` will be the content
                    && match &message_chain[0] {
                        SingleMessage::Plain { text } => text.as_str().trim().starts_with("/"),
                        _ => false,
                    }
            }
        };

        let mut command_name = "";
        let mut command_attrs = vec![];

        if is_command {
            let command_pattern = Regex::new(r"^\s*/(\S+)").unwrap();
            if let SingleMessage::Plain { text } = &message_chain[0] {
                if let Some(caps) = command_pattern.captures(text) {
                    command_name = caps.get(1).map_or("", |m| m.as_str());

                    let mut split = text.split(" ");
                    split.next();
                    for attr in split {
                        command_attrs.push(attr)
                    }
                }
            }
        }


        let mut content_message_chain = vec![];

        content_message_chain.push(SingleMessage::Plain{
            text: command_attrs.join(" ")
        });

        content_message_chain.extend_from_slice(&message_chain[1..]);

        Ok(Context {
            bot,

            is_at_me,

            is_command,
            command_name: command_name.to_string(),

            chatroom_type,
            chatroom_id: sender.chatroom_id(),
            chatroom_name: sender.chatroom_name(),

            sender_id: sender.sender_id(),
            sender_nickname: sender.sender_nickname(),

            message_id,
            message_chain: content_message_chain,
        })
    }

    pub fn clone(&self) -> Self {
        Context {
            bot: self.bot.clone(),

            is_at_me: self.is_at_me,

            is_command: self.is_command,
            command_name: self.command_name.clone(),

            chatroom_type: self.chatroom_type.clone(),
            chatroom_id: self.chatroom_id,
            chatroom_name: self.chatroom_name.clone(),

            sender_id: self.sender_id,
            sender_nickname: self.sender_nickname.clone(),

            message_id: self.message_id,
            message_chain: self.message_chain.clone(),
        }
    }

    pub fn chatroom_type(&self) -> ChatroomType {
        self.chatroom_type.clone()
    }

    pub fn chatroom_id(&self) -> i64 {
        self.chatroom_id
    }

    pub fn chatroom_name(&self) -> String {
        self.chatroom_name.clone()
    }

    pub fn sender_id(&self) -> i64 {
        self.sender_id
    }

    pub fn sender_nickname(&self) -> String {
        self.sender_nickname.clone()
    }

    pub fn message_chain(&self) -> MessageChain {
        let mut message_chain = vec![];
        for single_message in &self.message_chain {
            // remove the `At` message from the chain.
            match single_message {
                SingleMessage::At { .. } => continue,
                _ => message_chain.push(single_message.clone()),
            }
        }

        message_chain
    }

    pub fn is_at_message(&self) -> bool {
        match self.chatroom_type {
            ChatroomType::Friend => false,
            ChatroomType::Group => match self.message_chain[0] {
                SingleMessage::At { .. } => true,
                _ => false,
            },
        }
    }

    pub fn is_at_me(&self) -> bool {
        self.is_at_me
    }

    pub fn is_command(&self) -> bool {
        self.is_command
    }

    pub fn command_name(&self) -> &str {
        if !self.is_command {
            println!(
                "{}",
                Error::new(
                    "[Warning] Call `command_name` only when the `is_command()` return true."
                )
            )
        }

        &self.command_name
    }

    pub async fn reply(&self, message_chain: MessageChain) -> Result<()> {
        self.bot
            .send_message(
                self.chatroom_type.clone(),
                self.chatroom_id,
                message_chain,
                None,
            )
            .await?;

        Ok(())
    }

    pub async fn quote_reply(&self, message_chain: MessageChain) -> Result<()> {
        self.bot
            .send_message(
                self.chatroom_type.clone(),
                self.chatroom_id,
                message_chain,
                Some(self.message_id),
            )
            .await?;

        Ok(())
    }
}
