use crate::message::{MessageChain, Sender};

pub enum MessageType {
    Friend,
    Group,
}

pub struct Context {
    message_type: MessageType,
    chatroom: i32,

    sender_id: i32,
    sender_nickname: String,
}

impl Context {
    pub fn new<S>(sender: S, message_chain: MessageChain) -> Self
    where
        S: Sender,
    {
        Context {
            message_type: MessageType::Friend,
            chatroom: 0,
            sender_id: 0,
            sender_nickname: "".to_string(),
        }
    }
}
