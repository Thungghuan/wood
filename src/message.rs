#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum SingleMessage {
    Source { id: i32, time: i32 },
    At { target: String, display: String },
    Plain { text: String },
}

pub type MessageChain = Vec<SingleMessage>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Permission {
    OWNER,
    ADMINISTRATOR,
    MEMBER,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub permission: Permission,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FriendSender {
    pub id: i32,
    pub nickname: String,
    pub remark: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GroupSender {
    pub id: i32,
    pub member_name: String,
    pub permission: Permission,
    pub group: Group,
}

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
fn check_message_chain_serialize_result() {
    let source_message = SingleMessage::Source {
        id: 20211113,
        time: 20211113,
    };

    assert_eq!(
        serde_json::to_string(&source_message).unwrap(),
        "{\"type\":\"Source\",\"id\":20211113,\"time\":20211113}"
    );

    let at_message = SingleMessage::At {
        target: "20211113".to_string(),
        display: "土土木木".to_string(),
    };

    assert_eq!(
        serde_json::to_string(&at_message).unwrap(),
        "{\"type\":\"At\",\"target\":\"20211113\",\"display\":\"土土木木\"}"
    );

    let plain_message = SingleMessage::Plain {
        text: "test".to_string(),
    };

    assert_eq!(
        serde_json::to_string(&plain_message).unwrap(),
        "{\"type\":\"Plain\",\"text\":\"test\"}"
    );

    let mut message_chain: MessageChain = vec![];
    message_chain.push(source_message);
    message_chain.push(plain_message);

    assert_eq!(
        serde_json::to_string(&message_chain).unwrap(),
        "[{\"type\":\"Source\",\"id\":20211113,\"time\":20211113},{\"type\":\"Plain\",\"text\":\"test\"}]"
    )
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

#[test]
fn check_group_sender_deserialize_result() {
    let resp = r#"
    {
        "sender": {
            "id":20211113,
            "memberName":"Thungghuan",
            "specialTitle":"",
            "permission":"OWNER",
            "joinTimestamp":20211113,
            "lastSpeakTimestamp":20211113,
            "muteTimeRemaining":0,
            "group": {
                "id":20211113,
                "name":"木木",
                "permission":"ADMINISTRATOR"
            }
        }
    }"#;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct GroupSenderStruct {
        sender: GroupSender,
    }

    let group = Group {
        id: 20211113,
        name: "木木".to_string(),
        permission: Permission::ADMINISTRATOR,
    };

    let group_sender = GroupSender {
        id: 20211113,
        member_name: "Thungghuan".to_string(),
        permission: Permission::OWNER,
        group,
    };

    let group_sender_struct = GroupSenderStruct {
        sender: group_sender,
    };

    assert_eq!(
        serde_json::from_str::<GroupSenderStruct>(&resp).unwrap(),
        group_sender_struct
    );
}
