use crate::message::{ChatroomType, MessageChain, Sender};
use crate::{Bot, Result};

pub struct Context<'ctx> {
    bot: &'ctx Bot,

    chatroom_type: ChatroomType,
    chatroom_id: i32,
    chatroom_name: String,

    sender_id: i32,
    sender_nickname: String,

    message_chain: MessageChain,
}

impl<'ctx> Context<'ctx> {
    pub fn new<S>(bot: &'ctx Bot, sender: S, message_chain: MessageChain) -> Self
    where
        S: Sender,
    {
        println!("{:#?}", message_chain);

        Context {
            bot,

            chatroom_type: sender.chatroom_type(),
            chatroom_id: sender.chatroom_id(),
            chatroom_name: sender.chatroom_name(),

            sender_id: sender.sender_id(),
            sender_nickname: sender.sender_nickname(),

            message_chain,
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
