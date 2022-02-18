mod http;

use http::Http;
use tokio::io;

pub struct Api {
    qq: String,
    http: Http,
    base_url: String,
}

impl Api {
    pub fn new(qq: &str, base_url: &str, session: &str) -> Self {
        Api {
            qq: qq.to_string(),
            http: Http::new(qq, base_url, session),
            base_url: base_url.to_string(),
        }
    }
}
