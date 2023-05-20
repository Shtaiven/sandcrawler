/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://api.spacetraders.io/v2\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipRefine200ResponseData {
    #[serde(rename = "cargo")]
    pub cargo: Box<crate::models::ShipCargo>,
    #[serde(rename = "cooldown")]
    pub cooldown: Box<crate::models::Cooldown>,
    #[serde(rename = "produced")]
    pub produced: Vec<crate::models::ShipRefine200ResponseDataProducedInner>,
    #[serde(rename = "consumed")]
    pub consumed: Vec<crate::models::ShipRefine200ResponseDataProducedInner>,
}

impl ShipRefine200ResponseData {
    pub fn new(
        cargo: crate::models::ShipCargo,
        cooldown: crate::models::Cooldown,
        produced: Vec<crate::models::ShipRefine200ResponseDataProducedInner>,
        consumed: Vec<crate::models::ShipRefine200ResponseDataProducedInner>,
    ) -> ShipRefine200ResponseData {
        ShipRefine200ResponseData {
            cargo: Box::new(cargo),
            cooldown: Box::new(cooldown),
            produced,
            consumed,
        }
    }
}
