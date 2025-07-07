use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub user_config: UserConfig,
    pub data: Data,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct UserConfig {
    pub box_path: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Data {
    pub target: String,
}
