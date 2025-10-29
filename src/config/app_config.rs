use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AppConfig {
    #[serde(default)]
    pub user_config: UserConfig,
    #[serde(default)]
    pub data: Data,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct UserConfig {
    pub box_path: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Data {
    #[serde(default)]
    pub targets: Vec<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
