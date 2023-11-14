use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtraInfo {
    pub portal_url: String,
    pub started_in: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationInfo {
    pub id: Option<String>,
    pub name: String,
    pub package_name: String,
    pub version: String,
    pub authority_url: String,
    pub allow_anonymous_root_access: bool,
    pub extra_info: ExtraInfo,
}
