use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    pub account_id: String,
    pub symbol: String,
    pub headquarters: String,
    pub credits: i32,
}

#[derive(Deserialize, Debug)]
pub struct AgentApiResponse {
    pub data: Agent,
}
