use crate::error::Error;
use crate::message::{ChatroomType, MessageChain, Sender, SingleMessage};
use crate::{Bot, Result};

#[allow(dead_code)]
pub struct Context {
    bot: Bot,

    chatroom_type: ChatroomType,
    chatroom_id: i32,
    chatroom_name: String,

    sender_id: i32,
    sender_nickname: String,

    message_id: i32,
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
            SingleMessage::Source { id, time: _ } => id,
            _ => {
                return Err(Error::new(
                    "[Error] Receiving error message type when creating context.".to_string(),
                ))
            }
        };

        Ok(Context {
            bot,

            chatroom_type: sender.chatroom_type(),
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

    pub fn chatroom_id(&self) -> i32 {
        self.chatroom_id
    }

    pub fn chatroom_name(&self) -> String {
        self.chatroom_name.clone()
    }

    pub fn sender_id(&self) -> i32 {
        self.sender_id
    }

    pub fn sender_nickname(&self) -> String {
        self.sender_nickname.clone()
    }

    pub fn message_chain(&self) -> MessageChain {
        let mut message_chain = vec![];
        for single_message in &self.message_chain {
            message_chain.push(single_message.clone())
        }

        message_chain
    }

    pub async fn reply(&self, message_chain: MessageChain) -> Result<()> {
        self.bot
            .send_message(
                self.chatroom_type.clone(),
                &self.chatroom_id.to_string(),
                message_chain,
            )
            .await?;

        Ok(())
    }
}
