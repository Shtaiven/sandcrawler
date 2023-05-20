/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://api.spacetraders.io/v2\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */

/// ScannedWaypoint : A waypoint is a location that ships can travel to such as a Planet, Moon or Space Station.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScannedWaypoint {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "type")]
    pub r#type: crate::models::WaypointType,
    #[serde(rename = "systemSymbol")]
    pub system_symbol: String,
    #[serde(rename = "x")]
    pub x: i32,
    #[serde(rename = "y")]
    pub y: i32,
    #[serde(rename = "orbitals")]
    pub orbitals: Vec<crate::models::WaypointOrbital>,
    #[serde(rename = "faction", skip_serializing_if = "Option::is_none")]
    pub faction: Option<Box<crate::models::WaypointFaction>>,
    /// The traits of the waypoint.
    #[serde(rename = "traits")]
    pub traits: Vec<crate::models::WaypointTrait>,
    #[serde(rename = "chart", skip_serializing_if = "Option::is_none")]
    pub chart: Option<Box<crate::models::Chart>>,
}

impl ScannedWaypoint {
    /// A waypoint is a location that ships can travel to such as a Planet, Moon or Space Station.
    pub fn new(
        symbol: String,
        r#type: crate::models::WaypointType,
        system_symbol: String,
        x: i32,
        y: i32,
        orbitals: Vec<crate::models::WaypointOrbital>,
        traits: Vec<crate::models::WaypointTrait>,
    ) -> ScannedWaypoint {
        ScannedWaypoint {
            symbol,
            r#type,
            system_symbol,
            x,
            y,
            orbitals,
            faction: None,
            traits,
            chart: None,
        }
    }
}
