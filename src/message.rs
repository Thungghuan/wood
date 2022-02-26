use serde::Serialize;

// TODO: customize enum serialization
// #[derive(Serialize)]
// pub enum SingleMessage {
//     Plain {
//         #[serde(rename = "type")]
//         _type: String,
//         text: String,
//     },
// }

// pub type MessageChain = Vec<SingleMessage>;

// Use Plain type message first
#[derive(Serialize)]
pub struct PlainMessage {
    #[serde(rename = "type")]
    pub _type: String,
    pub text: String,
}

pub type MessageChain = Vec<PlainMessage>;
