#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RcoBotConfig {
    pub debug: bool,
    pub bot_config: BotConfig,
    pub ett: EttConfig,
    pub setu: SetuConfig,
    pub apex_api: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BotConfig {
    pub super_admin: Vec<String>,
    pub bot_name: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EttConfig {
    pub uin: String,
    pub pwd: String,
    pub cooldown: Option<i32>,
    pub timeout: Option<i32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SetuConfig {
    pub recall_time: i32,
    pub whether_to_save_locally: bool,
}

impl Default for RcoBotConfig {
    fn default() -> Self {
        let yml_data = std::fs::read_to_string("./resources/config/botconfig.yml")
            .expect("config file not found");
        let config = serde_yaml::from_str::<RcoBotConfig>(&yml_data).unwrap();
        config
    }
}
