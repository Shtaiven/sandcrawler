use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    pub account_id: String,
    pub symbol: String,
    pub headquarters: String,
    pub credits: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    #[serde(default)]
    pub waypoint_symbol: String,

    #[serde(default)]
    pub submitted_by: String,

    #[serde(default)]
    pub submitted_on: String,
}
impl Default for Chart {
    fn default() -> Chart {
        Chart {
            waypoint_symbol: "".to_string(),
            submitted_by: "".to_string(),
            submitted_on: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectedSystem {
    pub symbol: String,
    pub sector_symbol: String,
    pub r#type: SystemType,

    #[serde(default)]
    pub faction_symbol: String,
    pub x: i32,
    pub y: i32,
    pub distance: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractType {
    Procurement,
    Transport,
    Shuttle,
}
impl fmt::Display for ContractType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContractDeliverGood {
    pub trade_symbol: String,
    pub destination_symbol: String,
    pub units_required: i32,
    pub units_fulfilled: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContractPayment {
    pub on_accepted: i32,
    pub on_fulfilled: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContractTerms {
    pub deadline: String,
    pub payment: ContractPayment,

    #[serde(default)]
    pub deliver: Vec<ContractDeliverGood>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Cooldown {
    pub ship_symbol: String,
    pub total_seconds: i32,
    pub remaining_seconds: i32,
    pub expiration: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Extraction {
    pub ship_symbol: String,
    pub r#yield: ExtractionYield,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtractionYield {
    symbol: String,
    units: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Faction {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub headquarters: String,
    pub traits: Vec<FactionTrait>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FactionTraitSymbol {
    Bureaucratic,
    Secretive,
    Capitalistic,
    Industrious,
    Peaceful,
    Distrustful,
    Welcoming,
    Smugglers,
    Scavengers,
    Rebellious,
    Exiles,
    Pirates,
    Raiders,
    Clan,
    Guild,
    Dominion,
    Fringe,
    Forsaken,
    Isolated,
    Localized,
    Established,
    Notable,
    Dominant,
    Inescapable,
    Innovative,
    Bold,
    Visionary,
    Curious,
    Daring,
    Exploratory,
    Resourceful,
    Flexible,
    Cooperative,
    United,
    Strategic,
    Intelligent,
    ResearchFocused,
    Collaborative,
    Progressive,
    Militaristic,
    TechnologicallyAdvanced,
    Aggressive,
    Imperialistic,
    TreasureHunters,
    Dexterous,
    Unpredictable,
    Brutal,
    Fleeting,
    Adaptable,
    SelfSufficient,
    Defensive,
    Proud,
    Diverse,
    Independent,
    SelfInterested,
    Fragmented,
    Commercial,
    FreeMarkets,
    Entrepreneurial,
}
impl fmt::Display for FactionTraitSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FactionTrait {
    pub symbol: FactionTraitSymbol,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JumpGate {
    pub jump_range: i32,

    #[serde(default)]
    pub faction_symbol: String,
    pub connected_systems: Vec<ConnectedSystem>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub symbol: String,
    pub exports: Vec<TradeGood>,
    pub imports: Vec<TradeGood>,
    pub exchange: Vec<TradeGood>,

    #[serde(default)]
    pub transactions: Vec<MarketTransaction>,

    #[serde(default)]
    pub trade_goods: Vec<MarketTradeGood>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarketTradeGood {
    pub symbol: String,
    pub trade_volume: i32,
    pub supply: MarketTradeGoodSupply,
    pub purchase_price: i32,
    pub sell_price: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarketTradeGoodSupply {
    Scarce,
    Limited,
    Moderate,
    Abundant,
}
impl fmt::Display for MarketTradeGoodSupply {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarketTransaction {
    pub waypoint_symbol: String,
    pub ship_symbol: String,
    pub trade_symbol: String,
    pub r#type: String,
    pub units: i32,
    pub price_per_unit: i32,
    pub total_price: i32,
    pub timestanp: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub total: String,
    pub page: String,
    pub limit: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScannedShip {
    pub symbol: String,
    pub registration: ShipRegistration,
    pub nav: ShipNav,

    #[serde(default)]
    pub frame: ScannedShipFrame,

    #[serde(default)]
    pub reactor: ScannedShipReactor,
    pub engine: ShipEngine,

    #[serde(default)]
    pub mounts: Vec<ShipMount>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScannedShipFrame {
    pub symbol: String,
}
impl Default for ScannedShipFrame {
    fn default() -> ScannedShipFrame {
        ScannedShipFrame {
            symbol: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScannedShipReactor {
    pub symbol: String,
}
impl Default for ScannedShipReactor {
    fn default() -> ScannedShipReactor {
        ScannedShipReactor {
            symbol: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScannedSystem {
    pub symbol: String,
    pub sector_symbol: String,
    pub r#type: SystemType,
    pub x: i32,
    pub y: i32,
    pub distance: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScannedWaypoint {
    pub symbol: String,
    pub r#type: WaypointType,
    pub system_symbol: String,
    pub x: i32,
    pub y: i32,
    pub orbitals: Vec<WaypointOrbital>,

    #[serde(default)]
    pub faction: WaypointFaction,
    pub traits: Vec<WaypointTrait>,

    #[serde(default)]
    pub chart: Chart,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ship {
    pub symbol: String,
    pub registration: ShipRegistration,
    pub nav: ShipNav,
    pub crew: ShipCrew,
    pub frame: ShipFrame,
    pub reactor: ShipReactor,
    pub engine: ShipEngine,
    pub modules: Vec<ShipModule>,
    pub mounts: Vec<ShipMount>,
    pub cargo: ShipCargo,
    pub fuel: ShipFuel,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipCargo {
    pub capacity: i32,
    pub units: i32,
    pub inventory: Vec<ShipCargoItem>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipCargoItem {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub units: i32,
}

type ShipCondition = i32;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipCrew {
    pub current: i32,
    pub required: i32,
    pub capacity: i32,

    #[serde(default)]
    pub rotation: ShipCrewRotation,
    pub morale: ShipCondition,
    pub wages: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipCrewRotation {
    Strict,
    Relaxed,
}
impl Default for ShipCrewRotation {
    fn default() -> ShipCrewRotation {
        ShipCrewRotation::Strict
    }
}
impl fmt::Display for ShipCrewRotation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipEngine {
    pub symbol: ShipEngineSymbol,
    pub name: String,
    pub description: String,
    pub condition: ShipCondition,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipEngineSymbol {
    EngineImpulseDriveI,
    EngineIonDriveI,
    EngineIonDriveIi,
    EngineHyperDriveI,
}
impl fmt::Display for ShipEngineSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipFrame {
    pub symbol: ShipFrameSymbol,
    pub name: String,
    pub description: String,

    #[serde(default)]
    pub condition: i32,
    pub module_slots: i32,
    pub mounting_points: i32,
    pub fuel_capacity: i32,
    pub requirements: ShipRequirements,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipFrameSymbol {
    FrameProbe,
    FrameDrone,
    FrameInterceptor,
    FrameRacer,
    FrameFighter,
    FrameFrigate,
    FrameShuttle,
    FrameExplorer,
    FrameMiner,
    FrameLightFreighter,
    FrameHeavyFreighter,
    FrameTransport,
    FrameDestroyer,
    FrameCruiser,
    FrameCarrier,
}
impl fmt::Display for ShipFrameSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipFuel {
    pub current: i32,
    pub capacity: i32,

    #[serde(default)]
    pub consumed: ShipFuelConsumed,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipFuelConsumed {
    pub amount: i32,
    pub timestamp: String,
}
impl Default for ShipFuelConsumed {
    fn default() -> ShipFuelConsumed {
        ShipFuelConsumed {
            amount: 0,
            timestamp: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipModule {
    pub symbol: ShipModuleSymbol,

    #[serde(default)]
    pub capacity: i32,

    #[serde(default)]
    pub range: i32,
    pub name: String,

    #[serde(default)]
    pub description: String,
    pub requirements: ShipRequirements,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipModuleSymbol {
    ModuleMineralProcessorI,
    ModuleCargoHoldI,
    ModuleCrewQuartersI,
    ModuleEnvoyQuartersI,
    ModulePassengerCabinI,
    ModuleMicroRefineryI,
    ModuleOreRefineryI,
    ModuleFuelRefineryI,
    ModuleScienceLabI,
    ModuleJumpDriveI,
    ModuleJumpDriveIi,
    ModuleJumpDriveIii,
    ModuleWarpDriveI,
    ModuleWarpDriveIi,
    ModuleWarpDriveIii,
    ModuleShieldGeneratorI,
    ModuleShieldGeneratorIi,
}
impl fmt::Display for ShipModuleSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipMount {
    pub symbol: ShipMountSymbol,
    pub name: String,

    #[serde(default)]
    pub description: String,

    #[serde(default)]
    pub strength: i32,

    #[serde(default)]
    pub deposits: Vec<ShipMountDeposit>,
    pub requirements: ShipRequirements,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipMountDeposit {
    QuartzSand,
    SiliconCrystals,
    PreciousStones,
    IceWater,
    AmmoniaIce,
    IronOre,
    CopperOre,
    SilverOre,
    AluminumOre,
    GoldOre,
    PlatinumOre,
    Diamonds,
    UraniteOre,
    MeritiumOre,
}
impl fmt::Display for ShipMountDeposit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipMountSymbol {
    MountGasSiphonI,
    MountGasSiphonIi,
    MountGasSiphonIii,
    MountSurveyorI,
    MountSurveyorIi,
    MountSurveyorIii,
    MountSensorArrayI,
    MountSensorArrayIi,
    MountSensorArrayIii,
    MountMiningLaserI,
    MountMiningLaserIi,
    MountMiningLaserIii,
    MountLaserCannonI,
    MountMissileLauncherI,
    MountTurretI,
}
impl fmt::Display for ShipMountSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipNav {
    pub system_symbol: String,
    pub waypoint_symbol: String,
    pub route: ShipNavRoute,
    pub status: ShipNavStatus,

    #[serde(default)]
    pub flight_mode: ShipNavFlightMode,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipNavFlightMode {
    Drift,
    Stealth,
    Cruise,
    Burn,
}
impl Default for ShipNavFlightMode {
    fn default() -> ShipNavFlightMode {
        ShipNavFlightMode::Cruise
    }
}
impl fmt::Display for ShipNavFlightMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipNavRoute {
    pub destination: ShipNavRouteWaypoint,
    pub departure: ShipNavRouteWaypoint,
    pub departure_time: String,
    pub arrival: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipNavRouteWaypoint {
    pub symbol: String,
    pub r#type: WaypointType,
    pub system_symbol: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipNavStatus {
    InTransit,
    InOrbit,
    Docked,
}
impl fmt::Display for ShipNavStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipReactor {
    pub symbol: ShipReactorSymbol,
    pub name: String,
    pub description: String,

    #[serde(default)]
    pub condition: ShipCondition,
    pub power_output: i32,
    pub requirements: ShipRequirements,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipReactorSymbol {
    ReactorSolarI,
    ReactorFusionI,
    ReactorFissionI,
    ReactorChemicalI,
    ReactorAntimatterI,
}
impl fmt::Display for ShipReactorSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipRegistration {
    pub name: String,
    pub faction_symbol: String,
    pub role: ShipRole,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipRequirements {
    #[serde(default)]
    pub power: i32,

    #[serde(default)]
    pub crew: i32,

    #[serde(default)]
    pub slots: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipRole {
    Fabricator,
    Harvester,
    Hauler,
    Interceptor,
    Excavator,
    Transport,
    Repair,
    Surveyor,
    Command,
    Carrier,
    Patrol,
    Satellite,
    Explorer,
    Refinery,
}
impl fmt::Display for ShipRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipType {
    ShipProbe,
    ShipMiningDrone,
    ShipInterceptor,
    ShipLightHauler,
    ShipCommandFrigate,
    ShipExplorer,
    ShipHeavyFreighter,
    ShipLightShuttle,
    ShipOreHound,
    ShipRefiningFreighter,
    Unknown,
}
impl fmt::Display for ShipType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Default for ShipType {
    fn default() -> ShipType {
        ShipType::Unknown
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Shipyard {
    pub symbol: String,
    pub ship_types: Vec<ShipType>,

    #[serde(default)]
    pub transactions: Vec<ShipyardTransaction>,

    #[serde(default)]
    pub ships: Vec<ShipyardShip>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipyardShip {
    #[serde(default)]
    pub r#type: ShipType,
    pub name: String,
    pub description: String,
    pub purchase_price: i32,
    pub frame: ShipFrame,
    pub reactor: ShipReactor,
    pub engine: ShipEngine,
    pub modules: Vec<ShipModule>,
    pub mounts: Vec<ShipMount>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipyardTransaction {
    pub waypoint_symbol: String,
    pub ship_symbol: String,
    pub price: i32,
    pub agent_symbol: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Survey {
    pub signature: String,
    pub symbol: String,
    pub deposits: Vec<SurveyDeposit>,
    pub expiration: String,
    pub size: SurveySize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SurveySize {
    Small,
    Moderate,
    Large,
}
impl fmt::Display for SurveySize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SurveyDeposit {
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct System {
    pub symbol: String,
    pub sector_symbol: String,
    pub r#type: SystemType,
    pub x: i32,
    pub y: i32,
    pub waypoints: Vec<SystemWaypoint>,
    pub factions: Vec<SystemFaction>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SystemFaction {
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SystemType {
    NeutronStar,
    RedStar,
    OrangeStar,
    BlueStar,
    YoungStar,
    WhiteDwarf,
    BlackHole,
    Hypergiant,
    Nebula,
    Unstable,
}
impl fmt::Display for SystemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SystemWaypoint {
    pub symbol: String,
    pub r#type: WaypointType,
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradeGood {
    pub symbol: TradeSymbol,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TradeSymbol {
    PreciousStones,
    QuartzSand,
    SiliconCrystals,
    AmmoniaIce,
    LiquidHydrogen,
    LiquidNitrogen,
    IceWater,
    ExoticMatter,
    AdvancedCircuitry,
    GravitonEmitters,
    Iron,
    IronOre,
    Copper,
    CopperOre,
    Aluminum,
    AluminumOre,
    Silver,
    SilverOre,
    Gold,
    GoldOre,
    Platinum,
    PlatinumOre,
    Diamonds,
    Uranite,
    UraniteOre,
    Meritium,
    MeritiumOre,
    Hydrocarbon,
    Antimatter,
    Fertilizers,
    Fabrics,
    Food,
    Jewelry,
    Machinery,
    Firearms,
    AssaultRifles,
    MilitaryEquipment,
    Explosives,
    LabInstruments,
    Ammunition,
    Electronics,
    ShipPlating,
    Equipment,
    Fuel,
    Medicine,
    Drugs,
    Clothing,
    Microprocessors,
    Plastics,
    Polynucleotides,
    Biocomposites,
    Nanobots,
    AiMainframes,
    QuantumDrives,
    RoboticDrones,
    CyberImplants,
    GeneTherapeutics,
    NeuralChips,
    MoodRegulators,
    ViralAgents,
    MicroFusionGenerators,
    Supergrains,
    LaserRifles,
    Holographics,
    ShipSalvage,
    RelicTech,
    NovelLifeforms,
    BotanicalSpecimens,
    CulturalArtifacts,
    ReactorSolarI,
    ReactorFusionI,
    ReactorFissionI,
    ReactorChemicalI,
    ReactorAntimatterI,
    EngineImpulseDriveI,
    EngineIonDriveI,
    EngineIonDriveIi,
    EngineHyperDriveI,
    ModuleMineralProcessorI,
    ModuleCargoHoldI,
    ModuleCrewQuartersI,
    ModuleEnvoyQuartersI,
    ModulePassengerCabinI,
    ModuleMicroRefineryI,
    ModuleOreRefineryI,
    ModuleFuelRefineryI,
    ModuleScienceLabI,
    ModuleJumpDriveI,
    ModuleJumpDriveIi,
    ModuleJumpDriveIii,
    ModuleWarpDriveI,
    ModuleWarpDriveIi,
    ModuleWarpDriveIii,
    ModuleShieldGeneratorI,
    ModuleShieldGeneratorIi,
    MountGasSiphonI,
    MountGasSiphonIi,
    MountGasSiphonIii,
    MountSurveyorI,
    MountSurveyorIi,
    MountSurveyorIii,
    MountSensorArrayI,
    MountSensorArrayIi,
    MountSensorArrayIii,
    MountMiningLaserI,
    MountMiningLaserIi,
    MountMiningLaserIii,
    MountLaserCannonI,
    MountMissileLauncherI,
    MountTurretI,
}
impl fmt::Display for TradeSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Waypoint {
    pub symbol: String,
    pub r#type: WaypointType,
    pub system_symbol: String,
    pub x: i32,
    pub y: i32,
    pub orbitals: Vec<WaypointOrbital>,

    #[serde(default)]
    pub faction: WaypointFaction,
    pub traits: Vec<WaypointTrait>,

    #[serde(default)]
    pub chart: Chart,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WaypointFaction {
    pub symbol: String,
}
impl Default for WaypointFaction {
    fn default() -> WaypointFaction {
        WaypointFaction {
            symbol: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WaypointOrbital {
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WaypointTrait {
    pub symbol: WaypointTraitSymbol,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WaypointTraitSymbol {
    Uncharted,
    Marketplace,
    Shipyard,
    Outpost,
    ScatteredSettlements,
    SprawlingCities,
    MegaStructures,
    Overcrowded,
    HighTech,
    Corrupt,
    Bureaucratic,
    TradingHub,
    Industrial,
    BlackMarket,
    ResearchFacility,
    MilitaryBase,
    SurveillanceOutpost,
    ExplorationOutpost,
    MineralDeposits,
    CommonMetalDeposits,
    PreciousMetalDeposits,
    RareMetalDeposits,
    MethanePools,
    IceCrystals,
    ExplosiveGases,
    StrongMagnetosphere,
    VibrantAuroras,
    SaltFlats,
    Canyons,
    PerpetualDaylight,
    PerpetualOvercast,
    DrySeabeds,
    MagmaSeas,
    Supervolcanoes,
    AshClouds,
    VastRuins,
    MutatedFlora,
    Terraformed,
    ExtremeTemperatures,
    ExtremePressure,
    DiverseLife,
    ScarceLife,
    Fossils,
    WeakGravity,
    StrongGravity,
    CrushingGravity,
    ToxicAtmosphere,
    CorrosiveAtmosphere,
    BreathableAtmosphere,
    Jovian,
    Rocky,
    Volcanic,
    Frozen,
    Swamp,
    Barren,
    Temperate,
    Jungle,
    Ocean,
    Stripped,
}
impl fmt::Display for WaypointTraitSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WaypointType {
    Planet,
    GasGiant,
    Moon,
    OrbitalStation,
    JumpGate,
    AsteroidField,
    Nebula,
    DebrisField,
    GravityWell,
}
impl fmt::Display for WaypointType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
