pub struct SingleMessage {
    _type: String,
    text: String,
}

impl SingleMessage {
    pub fn new(message: String) -> Self {
        SingleMessage {
            _type: "Plain".to_string(),
            text: message,
        }
    }
}

pub type MessageChain = [SingleMessage];
