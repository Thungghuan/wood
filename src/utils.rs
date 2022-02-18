use serde::Deserialize;
use std::{collections::HashMap, fs::File, io::Read};
use yaml_rust::{Yaml, YamlLoader};

use crate::bot::BotConfig;

pub struct BotSettings {
    pub verify_key: String,
    pub host: String,
    pub port: String,
}

pub fn load_yaml_file(path: &str) -> Result<Yaml, ()> {
    let mut file;
    match File::open(path) {
        Ok(f) => file = f,
        Err(e) => panic!("{}", e),
    }

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        panic!("{}", e)
    }

    let config = YamlLoader::load_from_str(&contents).unwrap()[0].clone();
    Ok(config)
}

pub fn load_bot_config(path: &str) -> Result<BotConfig, ()> {
    let config = load_yaml_file(path)?;

    let qq = String::from(config["qq"].as_str().unwrap());
    let master_qq = String::from(config["masterQQ"].as_str().unwrap());
    let setting_file = String::from(config["settingFile"].as_str().unwrap());

    Ok(BotConfig::new(qq, master_qq, setting_file))
}

pub fn load_bot_settings(path: &str) -> BotSettings {
    let config = load_yaml_file(path).unwrap();

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

pub async fn get_session(base_url: &str, verify_key: &str) -> String {
    #[derive(Deserialize, Debug)]
    struct VerifyResponse {
        session: String,
    }

    let client = reqwest::Client::new();
    let mut data = HashMap::new();
    data.insert("verifyKey", verify_key);

    let resp = client
        .post(String::from(base_url) + "/verify")
        .json(&data)
        .send()
        .await
        .unwrap()
        .json::<VerifyResponse>()
        .await
        .unwrap();

    resp.session
}
