use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum SingleMessage {
    Source { id: i32, time: i32 },
    At { target: String, display: String },
    Plain { text: String },
}

pub type MessageChain = Vec<SingleMessage>;

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
