use serde::{Deserialize, Serialize};

pub struct Http {
    session: String,
    client: reqwest::Client,
    base_url: String,
}

#[derive(Deserialize, Debug)]
struct BasicResponse {
    code: i32,
    msg: String,
}

impl Http {
    pub fn new(base_url: &str, session: &str) -> Self {
        let builder = reqwest::Client::builder();
        let client = builder.no_proxy().build().unwrap();

        Http {
            base_url: base_url.to_string(),
            client,
            session: session.to_string(),
        }
    }

    fn url(&self, path: &str) -> String {
        self.base_url.clone() + path
    }

    pub async fn link(&self, qq: &str) -> Result<(), reqwest::Error> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            session_key: String,
            qq: String,
        }

        let params = Params {
            session_key: self.session.clone(),
            qq: qq.to_string(),
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
            Ok(())
        } else {
            panic!("{}", resp.msg)
        }
    }

    pub async fn release(&self, qq: &str) -> Result<(), reqwest::Error> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            session_key: String,
            qq: String,
        }

        let params = Params {
            session_key: self.session.clone(),
            qq: qq.to_string(),
        };

        let resp = self
            .client
            .post(self.url("/release"))
            .json(&params)
            .send()
            .await?
            .json::<BasicResponse>()
            .await?;

        if resp.code == 0 {
            Ok(())
        } else {
            panic!("{}", resp.msg)
        }
    }
}
