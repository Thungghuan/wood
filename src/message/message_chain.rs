use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "type")]
pub enum SingleMessage {
    Source {
        id: i64,
        time: i64,
    },

    At {
        target: i64,
        display: String,
    },

    Plain {
        text: String,
    },

    #[serde(rename_all = "camelCase")]
    Face {
        face_id: i64,
        name: String,
    },

    #[serde(rename_all = "camelCase")]
    Image {
        image_id: Option<String>,
        url: Option<String>,
        path: Option<String>,
        base64: Option<String>,
    },
}

pub type MessageChain = Vec<SingleMessage>;

pub fn create_plain_message(text: String) -> SingleMessage {
    SingleMessage::Plain { text }
}

pub fn create_plain_message_chain(text: String) -> MessageChain {
    let mut message_chain = vec![];

    message_chain.push(create_plain_message(text));

    message_chain
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
        target: 20211113,
        display: "土土木木".to_string(),
    };

    assert_eq!(
        serde_json::to_string(&at_message).unwrap(),
        "{\"type\":\"At\",\"target\":20211113,\"display\":\"土土木木\"}"
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
