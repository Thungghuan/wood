use regex::Regex;

use crate::error::Error;
use crate::message::{ChatroomType, MessageChain, Sender, SingleMessage};
use crate::{Bot, Result};

#[allow(dead_code)]
pub struct Context {
    bot: Bot,

    is_at_me: bool,

    chatroom_type: ChatroomType,
    chatroom_id: i64,
    chatroom_name: String,

    sender_id: i64,
    sender_nickname: String,

    message_id: i64,
    message_chain: MessageChain,
}

impl Context {
    pub fn new<S>(bot: Bot, sender: S, message_chain: &MessageChain) -> Result<Self>
    where
        S: Sender,
    {
        let source_message = message_chain[0].clone();
        let mut content_message_chain = vec![];
        content_message_chain.extend_from_slice(&message_chain[1..]);

        let message_id = match source_message {
            SingleMessage::Source { id, .. } => id,
            _ => {
                return Err(Error::new(
                    "[Error] Receiving error message type when creating context.",
                ))
            }
        };

        let chatroom_type = sender.chatroom_type();

        let is_at_me = match chatroom_type {
            ChatroomType::Friend => false,
            ChatroomType::Group => match content_message_chain[0] {
                SingleMessage::At { target, .. } => target == bot.qq(),
                _ => false,
            },
        };

        if is_at_me {
            content_message_chain.remove(0);
        }

        Ok(Context {
            bot,

            is_at_me,

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
        match self.chatroom_type {
            ChatroomType::Friend => match &self.message_chain[0] {
                SingleMessage::Plain { text } => text.as_str().trim().starts_with("/"),
                _ => false,
            },
            ChatroomType::Group => {
                self.is_at_me()
                    // the second `SingleMessage` will be the content
                    && match &self.message_chain[0] {
                        SingleMessage::Plain { text } => text.as_str().trim().starts_with("/"),
                        _ => false,
                    }
            }
        }
    }

    pub fn command_name(&self) -> &str {
        let mut name = "";

        if self.is_command() {
            let command_pattern = Regex::new(r"^\s*/(.+)").unwrap();
            if let SingleMessage::Plain { text } = &self.message_chain[0] {
                if let Some(caps) = command_pattern.captures(text) {
                    name = caps.get(1).map_or("", |m| m.as_str())
                }
            }
        } else {
            println!("{}", Error::new("[Warning] Call the `command_name` method only when the `is_command` return true."))
        }

        name
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
