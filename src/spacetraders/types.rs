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
    pub waypoint_symbol: String,
    pub submitted_by: String,
    pub submitted_on: String,
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
#[serde(rename_all = "UPPERCASE")]
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
#[serde(rename_all = "UPPERCASE")]
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
    pub transactions: Vec<MarketTransaction>,
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
#[serde(rename_all = "UPPERCASE")]
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
    pub frame: ShipFrame,
    pub reactor: ShipReactor,
    pub engine: ShipEngine,
    pub mounts: Vec<ShipMount>,
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
    pub faction: WaypointFaction,
    pub traits: Vec<WaypointTrait>,
    pub chart: Vec<Chart>,
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
#[serde(rename_all = "UPPERCASE")]
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
#[serde(rename_all = "UPPERCASE")]
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
    pub condition: i32,
    pub module_slots: i32,
    pub mounting_points: i32,
    pub fuel_capacity: i32,
    pub requirements: ShipRequirements,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
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
    pub consumed: ShipFuelConsumed,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipFuelConsumed {
    pub amount: i32,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipModule {
    pub symbol: ShipModuleSymbol,
    pub capacity: i32,
    pub range: i32,
    pub name: String,
    pub description: String,
    pub requirements: ShipRequirements,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
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
    pub description: String,
    pub strength: i32,
    pub deposits: Vec<ShipMountDeposit>,
    pub requirements: ShipRequirements,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
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
#[serde(rename_all = "UPPERCASE")]
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
#[serde(rename_all = "UPPERCASE")]
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
#[serde(rename_all = "UPPERCASE")]
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
    pub condition: ShipCondition,
    pub power_output: i32,
    pub requirements: ShipRequirements,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
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
    pub power: i32,
    pub crew: i32,
    pub slots: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
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
#[serde(rename_all = "UPPERCASE")]
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
}
impl fmt::Display for ShipType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Shipyard {
    pub symbol: String,
    pub ship_types: Vec<ShipType>,
    pub transactions: Vec<ShipyardTransaction>,
    pub ships: Vec<ShipyardShip>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipyardShip {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipyardTransaction {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Survey {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SurveyDeposit {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct System {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SystemFaction {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
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
pub struct SystemWaypoint {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradeGood {
    pub symbol: TradeSymbol,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
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
pub struct Waypoint {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WaypointFaction {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WaypointOrbital {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WaypointTrait {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
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
