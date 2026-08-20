use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestAccountRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountSelection {
    pub granted: bool,
    pub email: Option<String>,
    pub account_type: Option<String>,
    pub reason: Option<String>,
}
