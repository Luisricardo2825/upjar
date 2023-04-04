use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuilderConfig {
    pub url: String,
    pub user: String,
    pub password: String,
    pub jar_file_path: String,
    pub resource_id: String,
    pub resource_desc: String,
}
