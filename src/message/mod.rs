#![allow(dead_code)]
use serde::{Deserialize, Serialize};

mod sender;
pub use sender::Sender;
use sender::{FriendSender, GroupSender};

mod message_chain;
pub use message_chain::{MessageChain, SingleMessage};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum ReceivedMessage {
    #[serde(rename_all = "camelCase")]
    FriendMessage {
        sender: FriendSender,
        message_chain: MessageChain,
    },

    #[serde(rename_all = "camelCase")]
    GroupMessage {
        sender: GroupSender,
        message_chain: MessageChain,
    },
}

#[test]
fn check_received_friend_message_deserialize_result() {
    let resp = r#"{
        "type":"FriendMessage",
        "messageChain":[{"type":"Source","id":7,"time":20211113},{"type":"Plain","text":"hi"}],
        "sender":{"id":20211113,"nickname":"Thungghuan","remark":"Thungghuan"}
    }"#;

    let mut message_chain = vec![];
    let source_message = SingleMessage::Source {
        id: 7,
        time: 20211113,
    };
    let plain_message = SingleMessage::Plain {
        text: "hi".to_string(),
    };
    message_chain.push(source_message);
    message_chain.push(plain_message);

    let sender = FriendSender {
        id: 20211113,
        nickname: "Thungghuan".to_string(),
        remark: "Thungghuan".to_string(),
    };

    let received_message = ReceivedMessage::FriendMessage {
        sender,
        message_chain,
    };

    assert_eq!(
        serde_json::from_str::<ReceivedMessage>(&resp).unwrap(),
        received_message
    );
}
