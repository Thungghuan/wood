use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug, PartialEq)]
pub struct BotConfig {
    pub qq: i64,
    pub master_qq: i64,
    pub setting_file: String,
}

#[derive(Debug, PartialEq)]
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

fn load_bot_config(config: Yaml) -> BotConfig {
    let qq = config["qq"].as_i64().unwrap();
    let master_qq = config["masterQQ"].as_i64().unwrap();
    let setting_file = String::from(config["settingFile"].as_str().unwrap());

    BotConfig {
        qq,
        master_qq,
        setting_file,
    }
}

fn load_bot_settings(config: Yaml) -> BotSettings {
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
    let config = load_bot_config(load_yaml_file(path));
    let settings = load_bot_settings(load_yaml_file(&config.setting_file));
    let base_url = String::from("http://") + &settings.host + ":" + &settings.port;
    let session = get_session(&base_url, &settings.verify_key).await;

    (config, session, base_url)
}

#[cfg(test)]
mod test_utils {
    use super::{load_bot_config, load_bot_settings, BotConfig, BotSettings};
    use yaml_rust::YamlLoader;

    #[test]
    fn check_load_bot_config() {
        let config_file_string = r#"
# QQ number of bot
qq: 10000000
# QQ number of the master
masterQQ: 10000000
# Path to the `settings.yml` file for mirai
settingFile: 'config/settings.yml'
        "#;
        let config = YamlLoader::load_from_str(&config_file_string).unwrap()[0].clone();

        let bot_config = BotConfig {
            qq: 10000000,
            master_qq: 10000000,
            setting_file: "config/settings.yml".to_string(),
        };

        assert_eq!(load_bot_config(config), bot_config);
    }

    #[test]
    fn check_load_bot_settings() {
        let settings_file_string = r#"
## comments

## [Ignore]
adapters:
  - http
  - ws

## get VERIFY-KEY
enableVerify: true
verifyKey: verify-key

## get http.host and http.port only
adapterSettings:
  http:
    host: mirai.host
    port: 80
    # cors: [*]
  ws:
    host: mirai.host
    port: 80
    reservedSyncId: -1
        "#;
        let config = YamlLoader::load_from_str(&settings_file_string).unwrap()[0].clone();

        let bot_settings = BotSettings {
            verify_key: "verify-key".to_string(),
            host: "mirai.host".to_string(),
            port: "80".to_string(),
        };

        assert_eq!(load_bot_settings(config), bot_settings);
    }
}
