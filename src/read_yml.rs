use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeSlot {
    pub start: String,
    pub end: String,
    pub class: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub time: Vec<TimeSlot>,
}

pub fn read_yml() -> Result<Config, Box<dyn std::error::Error>> {
    // ホームディレクトリを取得し、YAMLファイルのパスを組み立てる
    let mut path = home_dir().expect("ホームディレクトリが見つかりません");
    path.push(".config/sutime.yml");

    // YAMLファイルの内容を読み込む
    let yaml_content = fs::read_to_string(&path)?;

    // YAMLをパースしてConfig構造体にデシリアライズする
    let config: Config = serde_yaml::from_str(&yaml_content)?;

    Ok(config)
}
