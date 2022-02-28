use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::message::MessageChain;
use crate::Result;

#[derive(Deserialize, Debug)]
struct BasicResponse {
    code: i32,
    msg: String,
}

pub struct Api {
    qq: String,
    session: String,
    client: reqwest::Client,
    base_url: String,
}

impl Api {
    pub fn new(qq: &str, base_url: &str, session: &str) -> Self {
        let builder = reqwest::Client::builder();
        let client = builder.no_proxy().build().unwrap();

        Api {
            qq: qq.to_string(),
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
            qq: String,
        }

        let params = Params {
            session_key: self.session.clone(),
            qq: self.qq.clone(),
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
            qq: String,
        }

        let params = Params {
            session_key: self.session.clone(),
            qq: self.qq.clone(),
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

    pub async fn send_friend_message(
        &self,
        target: &str,
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

        let resp = self
            .client
            .post(self.url("/sendFriendMessage"))
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

    pub async fn fetch_message(&self) -> Result<String> {
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

        let resp = self
            .client
            // ATTENTION: use /peekMessage for developing
            .get(self.url("/peekMessage"))
            // .get(self.url("/fetchMessage"))
            .query(&query)
            .send()
            .await?
            .text()
            .await?;

        Ok(resp)
    }
}
