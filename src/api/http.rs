pub struct Http {
    qq: String,
    session: String,
    client: reqwest::Client,
    base_url: String,
}

impl Http {
    pub fn new(qq: &str, base_url: &str, session: &str) -> Self {
        let builder = reqwest::Client::builder();
        let client = builder.no_proxy().build().unwrap();

        Http {
            qq: qq.to_string(),
            base_url: base_url.to_string(),
            client,
            session: String::new(),
        }
    }

    fn url(&self, path: &str) -> String {
        self.base_url.clone() + path
    }

    pub async fn link(&self) -> Result<(), reqwest::Error> {
        Ok(())
    }
}
