mod http;

use http::Http;
use crate::message::MessageChain;

pub struct Api {
    qq: String,
    http: Http,
    session: String,
}

impl Api {
    pub fn new(qq: &str, base_url: &str, session: &str) -> Self {
        Api {
            qq: qq.to_string(),
            session: session.clone().to_string(),
            http: Http::new(base_url, session),
        }
    }

    pub async fn link(&self) {
        match self.http.link(&self.qq).await {
            Ok(()) => println!("Bot successfully linked to qq: {}.", self.qq),
            Err(e) => panic!("{}", e),
        }
    }

    pub async fn release(&self) {
        match self.http.release(&self.qq).await {
            Ok(()) => println!("Session {} successfully released.", self.session),
            Err(e) => panic!("{}", e),
        }
    }

    pub async fn send_friend_message(
        &self,
        target: &str,
        message_chain: MessageChain,
    ) -> Result<(), reqwest::Error> {
        self.http.send_friend_message(target, message_chain).await?;
        Ok(())
    }
}
