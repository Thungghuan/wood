use serde::Serialize;

#[allow(dead_code)]
#[derive(Serialize)]
#[serde(tag = "type")]
pub enum SingleMessage {
    Source { id: String, time: String },
    At { target: String, display: String },
    Plain { text: String },
}

pub type MessageChain = Vec<SingleMessage>;

#[test]
fn check_message_chain_serialize_result() {
    let source_message = SingleMessage::Source {
        id: "20211113".to_string(),
        time: "20211113".to_string(),
    };

    assert_eq!(
        serde_json::to_string(&source_message).unwrap(),
        "{\"type\":\"Source\",\"id\":\"20211113\",\"time\":\"20211113\"}"
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
}
