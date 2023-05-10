use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct AgentApiResponse {
    pub data: super::types::Agent,
}

pub async fn get_agent(token: String) -> Result<super::types::Agent, Error> {
    let client = reqwest::Client::new();
    let request_url = "https://api.spacetraders.io/v2/my/agent";

    let response = client
        .get(request_url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    let agent: AgentApiResponse = response.json().await?;
    Ok(agent.data)
}
