mod http;

use http::Http;

pub struct Api {
    qq: String,
    http: Http,
}

impl Api {
    pub fn new(qq: &str, base_url: &str, session: &str) -> Self {
        Api {
            qq: qq.to_string(),
            http: Http::new(base_url, session),
        }
    }

    pub async fn link(&self) {
        match self.http.link(&self.qq).await {
            Ok(()) => (),
            Err(e) => panic!("{}", e),
        }
    }
}
