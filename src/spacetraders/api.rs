use reqwest::Error;

pub async fn get_agent(token: String) -> Result<super::types::Agent, Error> {
    let client = reqwest::Client::new();
    let request_url = "https://api.spacetraders.io/v2/my/agent";

    let response = client
        .get(request_url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    let agent: super::types::AgentApiResponse = response.json().await?;
    Ok(agent.data)
}
