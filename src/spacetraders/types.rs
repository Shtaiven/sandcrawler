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
#[serde(rename_all = "camelCase")]
pub struct Chart {
    pub waypoint_symbol: String,
    pub submitted_by: String,
    pub submitted_on: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum SystemType {
    NeutronStar(String),
    RedStar(String),
    OrangeStar(String),
    BlueStar(String),
    YoungStar(String),
    WhiteDwarf(String),
    BlackHole(String),
    Hypergiant(String),
    Nebula(String),
    Unstable(String),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectedSystem {
    pub symbol: String,
    pub sector_symbol: String,
    pub r#type: SystemType,
    pub faction_symbol: String,
    pub x: i32,
    pub y: i32,
    pub distance: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum ContractType {
    Procurement(String),
    Transport(String),
    Shuttle(String),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub id: String,
    pub faction_symbol: String,
    pub r#type: ContractType,
    pub terms: ContractTerms,
    pub accepted: bool,
    pub fulfilled: bool,
    pub expiration: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContractDeliverGood {
    pub trade_symbol: String,
    pub destination_symbol: String,
    pub units_required: i32,
    pub units_fulfilled: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContractPayment {
    pub on_accepted: i32,
    pub on_fulfilled: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContractTerms {
    pub deadline: String,
    pub payment: ContractPayment,
    pub deliver: Vec<ContractDeliverGood>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Cooldown {
    pub ship_symbol: String,
    pub total_seconds: i32,
    pub remaining_seconds: i32,
    pub expiration: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Extraction {
    pub ship_symbol: String,
    pub r#yield: ExtractionYield,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtractionYield {
    symbol: String,
    units: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Faction {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub headquarters: String,
    pub traits: Vec<FactionTrait>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum FactionTraitSymbol {
    Bureaucratic(String),
    Secretive(String),
    Capitalistic(String),
    Industrious(String),
    Peaceful(String),
    Distrustful(String),
    Welcoming(String),
    Smugglers(String),
    Scavengers(String),
    Rebellious(String),
    Exiles(String),
    Pirates(String),
    Raiders(String),
    Clan(String),
    Guild(String),
    Dominion(String),
    Fringe(String),
    Forsaken(String),
    Isolated(String),
    Localized(String),
    Established(String),
    Notable(String),
    Dominant(String),
    Inescapable(String),
    Innovative(String),
    Bold(String),
    Visionary(String),
    Curious(String),
    Daring(String),
    Exploratory(String),
    Resourceful(String),
    Flexible(String),
    Cooperative(String),
    United(String),
    Strategic(String),
    Intelligent(String),
    ResearchFocused(String),
    Collaborative(String),
    Progressive(String),
    Militaristic(String),
    TechnologicallyAdvanced(String),
    Aggressive(String),
    Imperialistic(String),
    TreasureHunters(String),
    Dexterous(String),
    Unpredictable(String),
    Brutal(String),
    Fleeting(String),
    Adaptable(String),
    SelfSufficient(String),
    Defensive(String),
    Proud(String),
    Diverse(String),
    Independent(String),
    SelfInterested(String),
    Fragmented(String),
    Commercial(String),
    FreeMarkets(String),
    Entrepreneurial(String),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FactionTrait {
    pub symbol: FactionTraitSymbol,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JumpGate {
    pub jump_range: i32,
    pub faction_symbol: String,
    pub connected_systems: Vec<ConnectedSystem>,
}

// TODO: Add more types from https://spacetraders.stoplight.io/docs/spacetraders/11f2735b75b02-space-traders-api
