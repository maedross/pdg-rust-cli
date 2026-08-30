use crate::{board::{
    Dominance::{Civilian, Military},
    Imperium::{Autonomy, Fragmentation, RomanRule},
}, concepts::{Nationality, StrongholdClass, UnitClass}};
use tracing::{Level, event, instrument};
use super::concepts::{Player, Stronghold, Unit};
use serde::{Deserialize, Serialize};
use serde_yaml::{self, Value};
use std::{collections::HashMap, fs, str::FromStr, vec};

#[derive(Clone, Debug)]
pub struct Board {
    pub map: Map,
    pub edge_track: EdgeTrack,
    pub civitates_available: CivitatesAvailable,
    pub civitates_not_yet_in_play: CivitatesNotYetInPlay,
    pub dux_available: DuxAvailable,
    pub dux_casualties: DuxCasualties,
    pub dux_out_of_play: DuxOutOfPlay,
    pub saxons_available: SaxonsAvailable,
    pub scotti_available: ScottiAvailable,
    pub scotti_niall_noigiallach: ScottiNiallNoigiallach,
    pub imperium: Imperium,
    pub roads_maintained: bool,
}

impl Board {
    fn new(
        game_map: Map,
        imperium: Imperium,
        briton_resources: u8,
        wealth: u8,
        dux_resources: u8,
        prestige: u8,
        saxon_renown: u8,
        scotti_renown: u8,
        civitates_available: CivitatesAvailable,
        civitates_not_yet_in_play: CivitatesNotYetInPlay,
        dux_available: DuxAvailable,
        dux_casualties: DuxCasualties,
        dux_out_of_play: DuxOutOfPlay,
        saxons_available: SaxonsAvailable,
        scotti_available: ScottiAvailable,
        scotti_niall_noigiallach: ScottiNiallNoigiallach,
        roads_maintained: bool,
    ) -> Board {
        let mut briton_control: u8 = game_map
            .land
            .clone()
            .into_values()
            .filter(|x| x.control == Some(Player::Civitates))
            .map(|x| x.pop)
            .sum();
        let mut dux_control: u8 = game_map
            .land
            .clone()
            .into_values()
            .filter(|x| x.control == Some(Player::Dux))
            .map(|x| x.pop)
            .sum();
        let mut saxon_control: u8 = game_map
            .land
            .clone()
            .into_values()
            .filter(|x| x.control == Some(Player::Saxons))
            .map(|x| x.pop)
            .sum();
        let mut total_prosperity: u8 = game_map
            .land
            .clone()
            .into_values()
            .filter(|x| x.control == Some(Player::Saxons))
            .map(|x| x.top_prosp + x.bottom_prosp)
            .sum();

        let mut briton_control_threshold: u8;
        let mut dux_threshold: u8;
        let mut saxon_renown_threshold: Option<u8>;
        match imperium {
            Imperium::RomanRule(dominance) => {
                briton_control_threshold = 36;
                dux_threshold = 75;
                saxon_renown_threshold = Some(30);
            }
            Imperium::Autonomy(dominance) => {
                briton_control_threshold = 27;
                dux_threshold = 60;
                saxon_renown_threshold = Some(30);
            }
            Imperium::Fragmentation => {
                briton_control_threshold = 16;
                dux_threshold = 17;
                saxon_renown_threshold = None;
            }
        }
        let mut edge_track = EdgeTrack {
            briton_resources,
            wealth,
            dux_resources,
            briton_control,
            dux_control,
            prestige,
            total_prosperity,
            saxon_renown,
            saxon_control,
            scotti_renown,
            briton_control_threshold,
            dux_threshold,
            saxon_control_threshold: 10,
            saxon_renown_threshold,
            scotti_renown_threshold: 45,
        };

        return Board {
            map: game_map,
            edge_track,
            civitates_available,
            civitates_not_yet_in_play,
            dux_available,
            dux_casualties,
            dux_out_of_play,
            saxons_available,
            scotti_available,
            scotti_niall_noigiallach,
            imperium,
            roads_maintained,
        };
    }
}

#[derive(Clone, Debug)]
pub struct Map {
    pub land: HashMap<String, Space>,
    pub off_map_land: HashMap<String, OffMapLandSpace>,
    pub seas: HashMap<String, Sea>,
}

impl Map {
    fn new() -> Map {
        return Map {
            land: HashMap::new(),
            off_map_land: HashMap::new(),
            seas: HashMap::new(),
        };
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Terrain {
    Clear,
    Fens,
    Hills,
}

impl FromStr for Terrain {
    type Err = MapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MapParseError as E;
        match s {
            "Clear" => Ok(Terrain::Clear),
            "Fens" => Ok(Terrain::Fens),
            "Hills" => Ok(Terrain::Hills),
            _ => {
                return Err(E {
                    err: format!("Invalid terrain type: {}", s),
                });
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Space {
    pub id: String,
    pub name: String,
    pub space_type: SpaceType,
    pub terrain: Option<Terrain>,
    pub adj_spaces: Vec<String>,
    pub adj_road: Vec<String>,
    pub pop: u8,
    #[serde(default)]
    pub max_pop: u8,
    #[serde(default)]
    pub top_prosp: u8,
    #[serde(default)]
    pub bottom_prosp: u8,
    pub stronghold_sites: HashMap<String, StrongholdSite>,
    #[serde(default)]
    pub units: Vec<Unit>,
    #[serde(default)]
    pub control: Option<Player>,
}

impl<'a> Space {
    fn new(
        id: &str,
        name: &str,
        space_type: SpaceType,
        terrain: Option<Terrain>,
        adj_spaces: Vec<String>,
        adj_road: Vec<String>,
        pop: u8,
        stronghold_sites: HashMap<String, StrongholdSite>,
    ) -> Space {
        Space {
            id: id.to_string(),
            name: name.to_string(),
            space_type,
            terrain,
            adj_spaces,
            adj_road,
            pop,
            max_pop: pop + 1,
            top_prosp: 0,
            bottom_prosp: 0,
            stronghold_sites,
            units: vec![],
            control: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpaceType {
    Region,
    City,
    Sea,
    OffMapLand,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StrongholdSiteType {
    Hillfort,
    Town,
    City,
}

#[derive(Debug)]
pub struct MapParseError {
    err: String,
}

impl FromStr for StrongholdSiteType {
    type Err = MapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MapParseError as E;
        match s {
            "Hillfort" => Ok(StrongholdSiteType::Hillfort),
            "Town" => Ok(StrongholdSiteType::Town),
            "City" => Ok(StrongholdSiteType::City),
            _ => {
                return Err(E {
                    err: format!("Invalid stronghold site type: {}", s),
                });
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StrongholdSite {
    pub name: String,
    pub site_type: StrongholdSiteType,
    pub stronghold: Option<Stronghold>,
}

impl<'a> StrongholdSite {
    fn new(name: &str, site_type: StrongholdSiteType) -> StrongholdSite {
        StrongholdSite {
            name: name.to_string(),
            site_type,
            stronghold: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct OffMapLandSpace {
    id: String,
    name: String,
    patrol_spaces: Vec<u8>,
    adj: Vec<u8>,
}

impl OffMapLandSpace {
    fn new(id: &str, name: &str) -> OffMapLandSpace {
        OffMapLandSpace {
            id: id.to_string(),
            name: name.to_string(),
            patrol_spaces: vec![],
            adj: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Sea {
    id: String,
    name: String,
    patrol: bool,
    adj: Vec<u8>,
}

impl Sea {
    fn new(id: &str, name: &str) -> Sea {
        Sea {
            id: id.to_string(),
            name: name.to_string(),
            patrol: false,
            adj: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub struct EdgeTrack {
    pub briton_resources: u8,
    pub wealth: u8,
    pub dux_resources: u8,
    briton_control: u8,
    dux_control: u8,
    pub prestige: u8,
    total_prosperity: u8,
    pub saxon_renown: u8,
    saxon_control: u8,
    pub scotti_renown: u8,
    briton_control_threshold: u8,
    dux_threshold: u8,
    saxon_control_threshold: u8,
    saxon_renown_threshold: Option<u8>,
    scotti_renown_threshold: u8,
}

#[derive(Clone, Debug)]
pub struct CivitatesAvailable {
    pub militia: u8,
    pub comitates: u8,
    pub towns: u8,
    pub hillforts: u8,
    pub refugees: u8,
}

#[derive(Clone, Debug)]
pub struct CivitatesNotYetInPlay {
    pub comitates: u8,
}

#[derive(Clone, Debug)]
pub struct ScottiAvailable {
    pub raiders: u8,
    pub warbands: u8,
    pub settlements: u8,
    pub max_settlements: u8,
}

#[derive(Clone, Debug)]
pub struct ScottiNiallNoigiallach {
    pub raiders: u8,
}

#[derive(Clone, Debug)]
pub struct SaxonsAvailable {
    pub raiders: u8,
    pub warbands: u8,
    pub settlements: u8,
    pub max_settlements: u8,
}

#[derive(Clone, Debug)]
pub struct DuxAvailable {
    pub cavalry: u8,
    pub forts: u8,
}

#[derive(Clone, Debug)]
pub struct DuxCasualties {
    pub cavalry: u8,
}

#[derive(Clone, Debug)]
pub struct DuxOutOfPlay {
    pub cavalry: u8,
}

#[derive(Clone, Debug)]
pub enum Imperium {
    RomanRule(Dominance),
    Autonomy(Dominance),
    Fragmentation,
}

#[derive(Clone, Copy, Debug)]
pub enum Dominance {
    Military,
    Civilian,
}

#[instrument]
pub fn build_map_from_yaml(file_path: &str) -> Map {
    let contents = fs::read_to_string(file_path).unwrap();
    let values: Vec<Value> = serde_yaml::from_str(&contents).unwrap();
    let mut game_map: Map = Map::new();
    for v in values {
        let space: &serde_yaml::Mapping = v.as_mapping().unwrap();
        let space_type: &str = space["space_type"].as_str().unwrap();
        match space_type {
            "Region" => {
                let id: &str = space["id"].as_str().unwrap();
                let name: &str = space["name"].as_str().unwrap();
                let space_type: SpaceType = SpaceType::Region;
                let terrain: Option<Terrain> =
                    Some(Terrain::from_str(space["Terrain"].as_str().unwrap()).unwrap());
                let pop: u8 = space["pop"].as_u64().unwrap() as u8;
                let mut stronghold_sites: HashMap<String, StrongholdSite> = HashMap::new();
                for s in space["stronghold_sites"].as_sequence().unwrap() {
                    let name: &str = s["name"].as_str().unwrap();
                    let site_type: StrongholdSiteType =
                        StrongholdSiteType::from_str(s["site_type"].as_str().unwrap()).unwrap();
                    let site: StrongholdSite = StrongholdSite::new(name, site_type);
                    stronghold_sites.insert(name.to_string(), site);
                }
                let adj_spaces: Vec<String> = space["adj_spaces"]
                    .as_sequence()
                    .unwrap()
                    .into_iter()
                    .map(|i| i.as_str().unwrap().to_string())
                    .collect();
                let adj_road: Vec<String> = space["adj_road"]
                    .as_sequence()
                    .unwrap()
                    .into_iter()
                    .map(|i| i.as_str().unwrap().to_string())
                    .collect();
                let space: Space = Space::new(
                    id,
                    name,
                    space_type,
                    terrain,
                    adj_spaces,
                    adj_road,
                    pop,
                    stronghold_sites,
                );
                game_map.land.insert(id.to_string(), space);
            }
            "City" => {
                let id: &str = space["id"].as_str().unwrap();
                let name: &str = space["name"].as_str().unwrap();
                let space_type: SpaceType = SpaceType::City;
                let terrain: Option<Terrain> = None;
                let pop: u8 = space["pop"].as_u64().unwrap() as u8;
                let mut stronghold_sites: HashMap<String, StrongholdSite> = HashMap::new();
                for s in space["stronghold_sites"].as_sequence().unwrap() {
                    let name: &str = s["name"].as_str().unwrap();
                    let site_type: StrongholdSiteType =
                        StrongholdSiteType::from_str(s["site_type"].as_str().unwrap()).unwrap();
                    let site: StrongholdSite = StrongholdSite::new(name, site_type);
                    stronghold_sites.insert(name.to_string(), site);
                }
                let adj_spaces: Vec<String> = space["adj_spaces"]
                    .as_sequence()
                    .unwrap()
                    .into_iter()
                    .map(|i| i.as_str().unwrap().to_string())
                    .collect();
                let adj_road: Vec<String> = space["adj_road"]
                    .as_sequence()
                    .unwrap()
                    .into_iter()
                    .map(|i| i.as_str().unwrap().to_string())
                    .collect();
                let space: Space = Space::new(
                    id,
                    name,
                    space_type,
                    terrain,
                    adj_spaces,
                    adj_road,
                    pop,
                    stronghold_sites,
                );
                game_map.land.insert(id.to_string(), space);
            }
            "Sea" => {
                let id: &str = space["id"].as_str().unwrap();
                let name: &str = space["name"].as_str().unwrap();
                let sea: Sea = Sea::new(id, name);
                game_map.seas.insert(id.to_string(), sea);
            }
            "Off map land" => {
                let id: &str = space["id"].as_str().unwrap();
                let name: &str = space["name"].as_str().unwrap();
                let off_map_land = OffMapLandSpace::new(id, name);
                game_map.off_map_land.insert(id.to_string(), off_map_land);
            }
            _ => panic!("Invalid space type: {}", space_type),
        }
    }
    return game_map;
}

#[instrument]
pub fn build_scenario_from_yaml(map_file_path: &str, scenario_file_path: &str) -> Board {
    let mut game_map: Map = build_map_from_yaml(map_file_path);
    let contents: String = fs::read_to_string(scenario_file_path).unwrap();
    let values: Value = serde_yaml::from_str(&contents).unwrap();

    let res: &serde_yaml::Mapping = values["Resources/Renown"].as_mapping().unwrap();
    let markers: &serde_yaml::Mapping = values["Markers"].as_mapping().unwrap();
    let spaces: &serde_yaml::Mapping = values["Spaces"].as_mapping().unwrap();
    let holding_boxes: &serde_yaml::Mapping = values["Holding Boxes"].as_mapping().unwrap();

    //Resources
    let briton_resources: u8 = res["Briton"].as_u64().unwrap() as u8;
    let dux_resources: u8 = res["Dux"].as_u64().unwrap() as u8;
    let saxon_renown: u8 = res["Saxon"].as_u64().unwrap() as u8;
    let scotti_renown: u8 = res["Scotti"].as_u64().unwrap() as u8;

    //Markers
    let wealth: u8 = markers["Wealth"].as_u64().unwrap() as u8;
    let prestige: u8 = markers["Prestige"].as_u64().unwrap() as u8;
    let roads_maintained: bool = markers["Roads"].as_bool().unwrap();

    let imperium_dominance: Option<&str> =
        markers["Imperium"].as_mapping().unwrap()["Dominance"].as_str();
    let imperium: Imperium = match markers["Imperium"].as_mapping().unwrap()["Level"]
        .as_str()
        .unwrap()
    {
        "Roman Rule" => match imperium_dominance {
            Some(s) => match s {
                "Military" => RomanRule(Military),
                "Civilian" => RomanRule(Civilian),
                _ => panic!("Invalid dominance"),
            },
            None => panic!("Require Dominance at Roman Rule!"),
        },
        "Autonomy" => match imperium_dominance {
            Some(s) => match s {
                "Military" => Autonomy(Military),
                "Civilian" => Autonomy(Civilian),
                _ => panic!("Invalid dominance"),
            },
            None => panic!("Require Dominance at Autonomy!"),
        },
        "Fragmentation" => Fragmentation,
        _ => panic!("Invalid imperium level!"),
    };

    //Spaces
    event!(Level::INFO, ?spaces);
    for space_key in spaces.clone().into_keys() {
        let space_id: &str = space_key.as_str().unwrap();
        let space: &Value = spaces.get(space_id).unwrap();
        let space_mapping: &serde_yaml::Mapping = space.as_mapping().unwrap();
        event!(Level::INFO, space_id, ?space);
        let x: Option<&mut Space> = game_map.land.get_mut(space_id);
        match x {
            Some(land) => {
                match space_mapping["Control"].as_str().unwrap() {
                    "Briton" => land.control = Some(Player::Civitates),
                    "Dux" => land.control = Some(Player::Dux),
                    "Saxon" => land.control = Some(Player::Saxons),
                    "Scotti" => land.control = Some(Player::Scotti),
                    "None" => land.control = None,
                    _ => panic!("Invalid control for {}", space_id),
                }

                //TODO: Handle altered population

                if land.space_type == SpaceType::Region {
                    event!(Level::INFO, ?land.space_type);
                    land.top_prosp = space_mapping["Prosperity"].as_mapping().unwrap()["Top"]
                        .as_u64()
                        .unwrap() as u8;
                    land.bottom_prosp = space_mapping["Prosperity"].as_mapping().unwrap()["Bottom"]
                        .as_u64()
                        .unwrap() as u8;
                } else if land.space_type == SpaceType::City {
                    land.bottom_prosp = space_mapping["Prosperity"].as_u64().unwrap() as u8;
                }

                event!(Level::INFO, stronghold_sites  = ?space["Stronghold Sites"].as_mapping().unwrap());
                for (site_name, site_piece) in space["Stronghold Sites"].as_mapping().unwrap().iter() {
                    let site_name = site_name.as_str().unwrap();
                    let site_piece = site_piece.as_mapping().unwrap();
                    let site: &mut StrongholdSite = land.stronghold_sites.get_mut(site_name).unwrap();
                    site.stronghold = match site_piece["Type"].as_str().unwrap() {
                        "Fort" => Some(Stronghold::new(StrongholdClass::Fort, Some(Player::Dux), None)),
                        "Hillfort" => Some(Stronghold::new(StrongholdClass::Hillfort, Some(Player::Civitates), Some(Nationality::Briton))),
                        "Town" => Some(Stronghold::new(StrongholdClass::Town, Some(Player::Civitates), Some(Nationality::Briton))),
                        "Saxon Settlement" => Some(Stronghold::new(StrongholdClass::Settlement, Some(Player::Saxons), Some(Nationality::Saxon))),
                        "Scotti Settlement" => Some(Stronghold::new(StrongholdClass::Settlement, Some(Player::Scotti), Some(Nationality::Scotti))),
                        _ => panic!("Invalid stronghold type {}", site_name),
                    }
                }

                let unit_list: &serde_yaml::Mapping = space["Units"].as_mapping().unwrap();
                for unit in unit_list.keys() {
                    let unit: &str = unit.as_str().unwrap();
                    match unit {
                        "Cavalry" => {
                            let designation: UnitClass = UnitClass::Cavalry;
                            let controller: Player = Player::Dux;
                            let nationality: Nationality = Nationality::Briton;
                            let plunder: bool = unit_list.contains_key("Without Plunder");
                            let amt = unit_list["Cavalry"].as_mapping().unwrap()["Without Plunder"].as_u64().unwrap();
                            for _ in 0..amt {
                                land.units.push(Unit { designation, controller, nationality, plunder });
                            }
                        },
                        "Militia" => {
                            let militia: &serde_yaml::Mapping = unit_list["Militia"].as_mapping().unwrap();
                            let designation: UnitClass = UnitClass::Militia;
                            let controller: Player = Player::Civitates;
                            let nationality: Nationality = Nationality::Briton;
                            let plunder: bool = unit_list.contains_key("Without Plunder");
                            let amt = militia["Without Plunder"].as_u64().unwrap();
                            for _ in 0..amt {
                                land.units.push(Unit { designation, controller, nationality, plunder });
                            }
                        },
                        _ => panic!("Invalid unit type {}", unit),
                    }
                }
            }
            None => {
                let y: Option<&mut Sea> = game_map.seas.get_mut(space_id);
                match y {
                    Some(sea) => {
                        let patrol = space_mapping["Patrolled"].as_bool().unwrap();
                        sea.patrol = patrol;
                    },
                    None => panic!("Unrecognized space ID {}", space_id),
                }
            }
        }
    }

    //Holding Boxes
    let civitates_available_mapping: &serde_yaml::Mapping =
        holding_boxes["Civitates"].as_mapping().unwrap()["Available"]
            .as_mapping()
            .unwrap();
    let dux_available_mapping: &serde_yaml::Mapping =
        holding_boxes["Dux"].as_mapping().unwrap()["Available"]
            .as_mapping()
            .unwrap();
    let saxon_available_mapping: &serde_yaml::Mapping =
        holding_boxes["Saxons"].as_mapping().unwrap()["Available"]
            .as_mapping()
            .unwrap();
    let scotti_available_mapping: &serde_yaml::Mapping =
        holding_boxes["Scotti"].as_mapping().unwrap()["Available"]
            .as_mapping()
            .unwrap();

    let civitates_available: CivitatesAvailable = CivitatesAvailable {
        militia: civitates_available_mapping["Militia"].as_u64().unwrap() as u8,
        comitates: civitates_available_mapping
            .get("Comitates")
            .map_or(0, |v| v.as_u64().unwrap() as u8),
        towns: civitates_available_mapping["Towns"].as_u64().unwrap() as u8,
        hillforts: civitates_available_mapping["Hillforts"].as_u64().unwrap() as u8,
        refugees: markers["Refugees"].as_u64().unwrap() as u8,
    };
    let civitates_not_yet_in_play: CivitatesNotYetInPlay = CivitatesNotYetInPlay {
        comitates: holding_boxes["Civitates"].as_mapping().unwrap()["Not yet in play"]
            .as_u64()
            .unwrap() as u8,
    };
    let dux_out_of_play: DuxOutOfPlay = DuxOutOfPlay {
        cavalry: holding_boxes["Dux"].as_mapping().unwrap()["Out of play"]
            .as_u64()
            .unwrap() as u8,
    };
    let dux_casualties: DuxCasualties = DuxCasualties {
        cavalry: holding_boxes["Dux"].as_mapping().unwrap()["Casualties"]
            .as_u64()
            .unwrap() as u8,
    };
    let scotti_niall_noigiallach: ScottiNiallNoigiallach = ScottiNiallNoigiallach {
        raiders: holding_boxes["Scotti"].as_mapping().unwrap()["Niall Noigiallach"]
            .as_u64()
            .unwrap() as u8,
    };

    let dux_available: DuxAvailable = DuxAvailable {
        cavalry: dux_available_mapping["Cavalry"].as_u64().unwrap() as u8,
        forts: dux_available_mapping["Forts"].as_u64().unwrap() as u8,
    };
    let saxons_available: SaxonsAvailable = SaxonsAvailable {
        raiders: saxon_available_mapping["Raiders"].as_u64().unwrap() as u8,
        warbands: saxon_available_mapping["Warbands"].as_u64().unwrap() as u8,
        settlements: saxon_available_mapping["Settlements"].as_u64().unwrap() as u8,
        max_settlements: 12,
    };
    let scotti_available: ScottiAvailable = ScottiAvailable {
        raiders: scotti_available_mapping["Raiders"].as_u64().unwrap() as u8,
        warbands: scotti_available_mapping["Warbands"].as_u64().unwrap() as u8,
        settlements: scotti_available_mapping["Settlements"].as_u64().unwrap() as u8,
        max_settlements: 6,
    };

    let game_board = Board::new(
        game_map,
        imperium,
        briton_resources,
        wealth,
        dux_resources,
        prestige,
        saxon_renown,
        scotti_renown,
        civitates_available,
        civitates_not_yet_in_play,
        dux_available,
        dux_casualties,
        dux_out_of_play,
        saxons_available,
        scotti_available,
        scotti_niall_noigiallach,
        roads_maintained,
    );
    return game_board;
}
