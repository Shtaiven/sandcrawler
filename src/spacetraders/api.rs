use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AgentApiResponse {
    pub data: super::types::Agent,
}

// TODO: Make api methods use the same client and token
// TODO: Ensure the maximum requests per second is respected
pub async fn get_agent(token: String) -> Result<super::types::Agent, Error> {
    let client = reqwest::Client::new();
    let request_url = "https://api.spacetraders.io/v2/my/agent";

    let response = client
        .get(request_url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token.trim()))
        .send()
        .await?;

    let agent: AgentApiResponse = response.json().await?;
    Ok(agent.data)
}

// TODO: Add more api methods from https://spacetraders.stoplight.io/docs/spacetraders/11f2735b75b02-space-traders-api
