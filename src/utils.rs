use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};
use yaml_rust::{Yaml, YamlLoader};

use crate::bot::BotConfig;

struct BotSettings {
    verify_key: String,
    host: String,
    port: String,
}

fn load_yaml_file(path: &str) -> Yaml {
    let mut file;
    match File::open(path) {
        Ok(f) => file = f,
        Err(e) => panic!("{}", e),
    }

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        panic!("{}", e)
    }

    // When error occurred executing `unwrap()`, it will panic.
    YamlLoader::load_from_str(&contents).unwrap()[0].clone()
}

fn load_bot_config(path: &str) -> BotConfig {
    let config = load_yaml_file(path);

    let qq = String::from(config["qq"].as_str().unwrap());
    let master_qq = String::from(config["masterQQ"].as_str().unwrap());
    let setting_file = String::from(config["settingFile"].as_str().unwrap());

    BotConfig::new(qq, master_qq, setting_file)
}

fn load_bot_settings(path: &str) -> BotSettings {
    let config = load_yaml_file(path);

    let verify_key = config["verifyKey"].as_str().unwrap().to_string();
    let host = config["adapterSettings"]["http"]["host"]
        .as_str()
        .unwrap()
        .to_string();
    let port = config["adapterSettings"]["http"]["port"]
        .as_i64()
        .unwrap()
        .to_string();

    BotSettings {
        verify_key,
        host,
        port,
    }
}

async fn get_session(base_url: &str, verify_key: &str) -> String {
    #[derive(Deserialize, Debug)]
    struct VerifyResponse {
        session: String,
    }

    let client = reqwest::Client::new();

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Params {
        verify_key: String,
    }

    let params = Params {
        verify_key: verify_key.to_string(),
    };

    let resp = client
        .post(String::from(base_url) + "/verify")
        .json(&params)
        .send()
        .await
        .unwrap()
        .json::<VerifyResponse>()
        .await
        .unwrap();

    resp.session
}

pub async fn init(path: &str) -> (BotConfig, String, String) {
    let config = load_bot_config(path);
    let settings = load_bot_settings(&config.setting_file);
    let base_url = String::from("http://") + &settings.host + ":" + &settings.port;
    let session = get_session(&base_url, &settings.verify_key).await;

    (config, session, base_url)
}
