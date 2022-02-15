use std::{fs::File, io::Read};
use yaml_rust::{Yaml, YamlLoader};

use crate::bot::{BotConfig, BotSettings, BotSettingsAdapter, BotSettingsAdapterHttp};

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

pub fn load_bot_settings(path: &str) -> Result<BotSettings, ()> {
    let config = load_yaml_file(path)?;

    let verify_key = config["verifyKey"].as_str().unwrap();
    let host = config["adapterSettings"]["http"]["host"].as_str().unwrap();
    let port = config["adapterSettings"]["http"]["port"].as_i64().unwrap();
    let port = u32::try_from(port).unwrap();

    let adapter_settings = BotSettingsAdapter {
        http: BotSettingsAdapterHttp {
            host: String::from(host),
            port,
        },
    };

    // Ok(BotSettings::new(String::from(verify_key), adapt_settings))
    Ok(BotSettings {
        verify_key: verify_key.to_string(),
        adapter_settings,
    })
}
