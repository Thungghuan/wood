use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};
use yaml_rust::{Yaml, YamlLoader};

use crate::bot::BotConfig;

pub async fn load_yaml_file(path: &str) -> io::Result<Yaml> {
    let mut file;
    match File::open(path).await {
        Ok(f) => file = f,
        Err(e) => panic!("{}", e),
    }

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents).await {
        panic!("{}", e)
    }

    let config = YamlLoader::load_from_str(&contents).unwrap()[0].clone();
    Ok(config)
}

pub async fn load_bot_config(path: &str) -> io::Result<BotConfig> {
    let config = load_yaml_file(path).await?;

    let qq = String::from(config["qq"].as_str().unwrap());
    let master_qq = String::from(config["master_qq"].as_str().unwrap());
    let setting_file = String::from(config["setting_file"].as_str().unwrap());

    Ok(BotConfig::new(qq, master_qq, setting_file))
}
