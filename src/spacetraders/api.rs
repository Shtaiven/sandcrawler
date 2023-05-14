use super::types;
use reqwest::{Error, RequestBuilder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
struct RegisterRequest {
    pub faction: String,
    pub symbol: String,
    pub email: String,
}

#[derive(Deserialize, Debug)]
struct RegisterResponse {
    pub data: Registration,
}

#[derive(Deserialize, Debug)]
pub struct Registration {
    pub agent: types::Agent,
    pub contract: types::Contract,
    pub faction: types::Faction,
    pub ship: types::Ship,
    pub token: String,
}

pub async fn register_new_agent(
    symbol: String,
    faction: String,
    email: String,
) -> Result<Registration, Error> {
    let client = reqwest::Client::new();
    let request_url = "https://api.spacetraders.io/v2/register";
    let register_request = RegisterRequest {
        faction,
        symbol,
        email,
    };

    let request_body = serde_json::to_string(&register_request).unwrap();

    let response = client
        .post(request_url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .body(request_body)
        .send()
        .await?;

    let register_response: RegisterResponse = response.json().await?;
    Ok(register_response.data)
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AgentApiResponse {
    pub data: types::Agent,
}

// TODO: Make api methods use the same client and token
// TODO: Ensure the maximum requests per second is respected
pub async fn my_agent_details(token: String) -> Result<types::Agent, Error> {
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
