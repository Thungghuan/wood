use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::message::{ChatroomType, MessageChain, ReceivedMessage};
use crate::Result;

#[derive(Deserialize, Debug)]
struct BasicResponse {
    code: i32,
    msg: String,
}

#[derive(Clone)]
pub struct Api {
    qq: i64,
    session: String,
    client: reqwest::Client,
    base_url: String,
}

impl Api {
    pub fn new(qq: i64, base_url: &str, session: &str) -> Self {
        let builder = reqwest::Client::builder();
        let client = builder.no_proxy().build().unwrap();

        Api {
            qq,
            session: session.clone().to_string(),
            client,
            base_url: base_url.to_string(),
        }
    }

    fn url(&self, path: &str) -> String {
        self.base_url.clone() + path
    }

    pub async fn link(&self) -> Result<()> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            session_key: String,
            qq: i64,
        }

        let params = Params {
            session_key: self.session.clone(),
            qq: self.qq,
        };

        let resp = self
            .client
            .post(self.url("/bind"))
            .json(&params)
            .send()
            .await?
            .json::<BasicResponse>()
            .await?;

        if resp.code == 0 {
            println!("Bot successfully linked to qq: {}.", self.qq);
            Ok(())
        } else {
            Err(Error::new(resp.msg))
        }
    }

    pub async fn release(&self) -> Result<()> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            session_key: String,
            qq: i64,
        }

        let params = Params {
            session_key: self.session.clone(),
            qq: self.qq,
        };

        let resp = self
            .client
            .post(self.url("/bind"))
            .json(&params)
            .send()
            .await
            .unwrap()
            .json::<BasicResponse>()
            .await
            .unwrap();

        if resp.code == 0 {
            Ok(())
        } else {
            Err(Error::new(resp.msg))
        }
    }

    pub async fn send_message(
        &self,
        chatroom_type: ChatroomType,
        target: i64,
        message_chain: MessageChain,
    ) -> Result<()> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            session_key: String,
            target: String,
            message_chain: MessageChain,
        }

        let params = Params {
            session_key: self.session.clone(),
            target: target.to_string(),
            message_chain,
        };

        let url = self.url(match chatroom_type {
            ChatroomType::Friend => "/sendFriendMessage",
            ChatroomType::Group => "/sendGroupMessage",
        });

        let resp = self
            .client
            .post(url)
            .json(&params)
            .send()
            .await?
            .json::<BasicResponse>()
            .await?;

        if resp.code == 0 {
            Ok(())
        } else {
            Err(Error::new(resp.msg))
        }
    }

    pub async fn fetch_messages(&self) -> Result<Vec<ReceivedMessage>> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            session_key: String,
            count: i8,
        }

        let query = Params {
            session_key: self.session.clone(),
            count: 10,
        };

        #[derive(Deserialize)]
        struct FetchResponse {
            code: i32,
            msg: String,
            data: Vec<ReceivedMessage>,
        }

        let resp = self
            .client
            .get(self.url("/fetchMessage"))
            .query(&query)
            .send()
            .await?
            .json::<FetchResponse>()
            .await?;

        if resp.code == 0 {
            Ok(resp.data)
        } else {
            Err(Error::new(resp.msg))
        }
    }
}
