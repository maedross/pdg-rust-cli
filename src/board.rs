use super::concepts::{Player, Stronghold, Unit};
use crate::{
    board::{
        Dominance::{Civilian, Military},
        Imperium::{Autonomy, Fragmentation, RomanRule},
    },
    concepts::{
        FORT, Force, HILLFORT, Nationality, SAXON_SETTLEMENT, SCOTTI_SETTLEMENT, StrongholdClass,
        TOWN, UnitClass,
    },
};
use dialoguer::Select;
use serde::{Deserialize, Serialize};
use serde_yaml::{self, Value};
use std::{cmp::min, collections::HashMap, fs, str::FromStr, vec};
use tracing::{Level, event, instrument};

#[derive(Clone, Debug)]
pub struct Board {
    pub map: Map,
    pub edge_track: EdgeTrack,
    pub available: Available,
    pub casualties: Casualties,
    pub out_of_play: OutOfPlay,
    pub not_yet_in_play: NotYetInPlay,
    pub niall_noigiallach: NiallNoigiallach,
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
        available: Available,
        casualties: Casualties,
        out_of_play: OutOfPlay,
        not_yet_in_play: NotYetInPlay,
        niall_noigiallach: NiallNoigiallach,
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
            available,
            casualties,
            out_of_play,
            not_yet_in_play,
            niall_noigiallach,
            imperium,
            roads_maintained,
        };
    }

    pub fn filter_spaces_unit(
        &self,
        class: Option<UnitClass>,
        controller: Option<Player>,
        nationality: Option<Nationality>,
        plunder: Option<bool>,
    ) -> Vec<String> {
        // Returns a Vec of names for spaces containing units with the specified class, controller, nationality, and/or plunder value
        self.map
            .land
            .values()
            .filter(|s| s.contains_unit(class, controller, nationality, plunder))
            .map(|s| s.id.clone())
            .collect::<Vec<String>>()
    }

    pub fn filter_spaces_stronghold(
        &self,
        class: Option<StrongholdClass>,
        controller: Option<Player>,
        nationality: Option<Nationality>,
    ) -> Vec<String> {
        self.map
            .land
            .values()
            .filter(|s| s.contains_stronghold(class, controller, nationality))
            .map(|s| s.id.clone())
            .collect::<Vec<String>>()
    }

    pub fn filter_spaces_empty_site(&self, site_type: Option<StrongholdSiteType>) -> Vec<String> {
        self.map.land.values().filter(|s| s.contains_empty_site(site_type)).map(|s| s.id.clone()).collect::<Vec<String>>()
    }

    pub fn filter_spaces_pieces(
        &self,
        controller: Option<Player>,
        nationality: Option<Nationality>,
    ) -> Vec<String> {
        self.map
            .land
            .values()
            .filter(|s| {
                s.contains_unit(None, controller, nationality, None)
                    || s.contains_stronghold(None, controller, nationality)
            })
            .map(|s| s.id.clone())
            .collect::<Vec<String>>()
    }

    fn update_land_space(&mut self, space: Space) {
        self.map.land.insert(space.name.clone(), space);
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
        id: String,
        name: String,
        space_type: SpaceType,
        terrain: Option<Terrain>,
        adj_spaces: Vec<String>,
        adj_road: Vec<String>,
        pop: u8,
        stronghold_sites: HashMap<String, StrongholdSite>,
    ) -> Space {
        Space {
            id: id,
            name: name,
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

    pub fn contains_stronghold(
        &self,
        class: Option<StrongholdClass>,
        controller: Option<Player>,
        nationality: Option<Nationality>,
    ) -> bool {
        let mut result: bool = true;
        result = result
            && match class {
                Some(c) => self.stronghold_sites.values().any(|s| match s.stronghold {
                    Some(stronghold) => stronghold.class == c,
                    None => false,
                }),
                None => false,
            };
        result = result
            && match controller {
                Some(c) => self.stronghold_sites.values().any(|s| match s.stronghold {
                    Some(stronghold) => stronghold.controller == c,
                    None => false,
                }),
                None => false,
            };
        result = result
            && match nationality {
                Some(n) => self.stronghold_sites.values().any(|s| match s.stronghold {
                    Some(stronghold) => stronghold.nationality == Some(n),
                    None => false,
                }),
                None => false,
            };
        return result;
    }

    pub fn contains_empty_site(&self, site_type: Option<StrongholdSiteType>) -> bool {
        self.stronghold_sites.values().any(|s| match s.stronghold {
            Some(_) => false,
            None => match &site_type {
                Some(t) => &s.site_type == t,
                None => true,
            }
        })
    }

    pub fn contains_stronghold_controller(&self, player: Player) -> bool {
        return self
            .stronghold_sites
            .values()
            .any(|site: &StrongholdSite| match site.stronghold {
                Some(s) => s.controller == player,
                _ => false,
            });
    }

    pub fn contains_unit(
        &self,
        class: Option<UnitClass>,
        controller: Option<Player>,
        nationality: Option<Nationality>,
        plunder: Option<bool>,
    ) -> bool {
        let mut result: bool = true;
        result = result
            && match class {
                Some(c) => self.units.iter().any(|u| u.class == c),
                None => true,
            };
        result = result
            && match controller {
                Some(c) => self.units.iter().any(|u| u.controller == c),
                None => true,
            };
        result = result
            && match nationality {
                Some(n) => self.units.iter().any(|u| u.nationality.unwrap() == n),
                None => true,
            };
        result = result
            && match plunder {
                Some(p) => self.units.iter().any(|u| u.plunder == p),
                None => true,
            };
        return result;
    }

    fn amt_units(
        &self,
        class: Option<UnitClass>,
        controller: Option<Player>,
        nationality: Option<Nationality>,
        plunder: Option<bool>,
    ) -> u8 {
        let mut units: Vec<Unit> = self.units.clone();
        match class {
            Some(c) => units = units.into_iter().filter(|u| u.class == c).collect(),
            None => {}
        };
        match controller {
            Some(c) => units = units.into_iter().filter(|u| u.controller == c).collect(),
            None => {}
        };
        match nationality {
            Some(n) => {
                units = units
                    .into_iter()
                    .filter(|u| u.nationality == Some(n))
                    .collect()
            }
            None => {}
        };
        match plunder {
            Some(p) => units = units.into_iter().filter(|u| u.plunder == p).collect(),
            None => {}
        };
        return units.len() as u8;
    }

    pub fn add_units(mut self, unit: Unit, amt: u8, board: &mut Board) -> Space {
        let placeable_amt: u8 = min(
            board
                .available
                .get(Force::Unit(unit.class, unit.nationality)),
            amt,
        );
        board
            .available
            .remove(Force::Unit(unit.class, unit.nationality), placeable_amt);
        for _ in 0..placeable_amt {
            self.units.push(unit);
        }

        let mut yet_to_place = amt - placeable_amt;
        while yet_to_place > 0 {
            println!(
                "Could not place all {:?} from Available; you may choose to Voluntarily Remove some from the board to Available to be placed.\n{} remain to be placed.",
                unit, yet_to_place
            );
            let mut valid_spaces: Vec<String> = board.filter_spaces_unit(
                Some(unit.class),
                Some(unit.controller),
                unit.nationality,
                None,
            );
            valid_spaces.push(String::from("Done"));
            let selection = Select::new()
                .with_prompt("Select a space or Done:")
                .items(&valid_spaces)
                .interact()
                .unwrap();
            if selection == 0 {
                break;
            } else {
                let mut selected_space: Space = board
                    .map
                    .land
                    .get(&valid_spaces[selection])
                    .unwrap()
                    .clone();
                let plunderful: u8 = selected_space.amt_units(
                    Some(unit.class),
                    Some(unit.controller),
                    unit.nationality,
                    Some(true),
                );
                let plunderless: u8 = selected_space.amt_units(
                    Some(unit.class),
                    Some(unit.controller),
                    unit.nationality,
                    Some(false),
                );
                if plunderful == 0 {
                    selected_space = selected_space.remove_unit(unit, board, false);
                } else if plunderless == 0 {
                    selected_space = selected_space.remove_unit(unit.with_plunder(), board, false);
                } else {
                    let plunder_selection: usize = Select::new()
                        .with_prompt("Select whether to remove a unit with or without Plunder:")
                        .items(&vec!["Without Plunder", "With Plunder"])
                        .interact()
                        .unwrap();
                    if plunder_selection == 0 {
                        selected_space = selected_space.remove_unit(unit, board, false);
                    } else {
                        selected_space =
                            selected_space.remove_unit(unit.with_plunder(), board, false);
                    }
                }
                selected_space = selected_space.remove_unit(unit, board, false);
                board.update_land_space(selected_space);
                self.units.push(unit);
                yet_to_place -= 1;
            }
        }
        return self;
    }

    pub fn remove_unit(mut self, unit: Unit, board: &mut Board, casualties: bool) -> Space {
        // Removes one unit from a space to Available or Casualties

        let index = self.units.iter().position(|u| *u == unit);
        match index {
            Some(i) => self.units.swap_remove(i),
            None => panic!(
                "Trying to remove unit that does not exist in the space: {:?}",
                unit
            ),
        };
        if casualties {
            if unit.class == UnitClass::Cavalry {
                board.casualties.cavalry += 1;
            } else {
                panic!(
                    "Attempting to add a piece to Casualties not currently allowed to go in there"
                )
            }
        } else {
            board
                .available
                .add(Force::Unit(unit.class, unit.nationality), 1)
        }
        return self;
    }

    pub fn replace_units(
        self,
        old_force: Unit,
        new_force: Unit,
        amt: u8,
        board: &mut Board,
    ) -> Space {
        let mut res = self.clone();
        for _ in 0..amt {
            res = res.remove_unit(old_force, board, false);
            res = res.add_units(new_force, 1, board);
        }
        return res;
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpaceType {
    Region,
    City,
    Sea,
    OffMapLand,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
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
pub struct Available {
    pub militia: u8,
    pub comitates: u8,
    pub towns: u8,
    pub hillforts: u8,
    pub refugees: u8,
    pub cavalry: u8,
    pub forts: u8,
    pub raiders_saxon: u8,
    pub warbands_saxon: u8,
    pub settlements_saxon: u8,
    pub raiders_scotti: u8,
    pub warbands_scotti: u8,
    pub settlements_scotti: u8,
}

impl Available {
    pub fn get(&self, force: Force) -> u8 {
        match force {
            Force::Stronghold(s) => match s {
                TOWN => self.towns,
                HILLFORT => self.hillforts,
                FORT => self.forts,
                SAXON_SETTLEMENT => self.settlements_saxon,
                SCOTTI_SETTLEMENT => self.settlements_scotti,
                _ => panic!("Invalid stronghold type to lookup in Available: {:?}", s),
            },
            Force::Unit(class, nationality) => match class {
                UnitClass::Comitates => self.comitates,
                UnitClass::Militia => self.militia,
                UnitClass::Cavalry => self.cavalry,
                UnitClass::Raider => match nationality {
                    Some(n) => match n {
                        Nationality::Saxon => self.raiders_saxon,
                        Nationality::Scotti => self.raiders_scotti,
                        Nationality::Briton => {
                            panic!("Trying to check for Available Briton Raiders")
                        }
                    },
                    None => panic!("Need a nationality to check for Available Raiders"),
                },
                UnitClass::Warband => match nationality {
                    Some(n) => match n {
                        Nationality::Saxon => self.warbands_saxon,
                        Nationality::Scotti => self.warbands_scotti,
                        Nationality::Briton => {
                            panic!("Trying to check for Available Briton Warbands")
                        }
                    },
                    None => panic!("Need a nationality to check for Available Warbands"),
                },
            },
        }
    }

    pub fn set(&mut self, force: Force, amt: u8) {
        match force {
            Force::Stronghold(stronghold) => match stronghold {
                TOWN => self.towns = amt,
                HILLFORT => self.hillforts = amt,
                FORT => self.forts = amt,
                SAXON_SETTLEMENT => self.settlements_saxon = amt,
                SCOTTI_SETTLEMENT => self.settlements_scotti = amt,
                _ => panic!("Unrecognized stronghold type {:?}", stronghold),
            },
            Force::Unit(class, nationality) => match class {
                UnitClass::Comitates => self.comitates = amt,
                UnitClass::Militia => self.militia = amt,
                UnitClass::Cavalry => self.cavalry = amt,
                UnitClass::Raider => match nationality {
                    Some(n) => match n {
                        Nationality::Saxon => self.raiders_saxon = amt,
                        Nationality::Scotti => self.raiders_scotti = amt,
                        Nationality::Briton => {
                            panic!("Trying to check for Available Briton Raiders")
                        }
                    },
                    None => panic!("Need a nationality to check for Available Raiders"),
                },
                UnitClass::Warband => match nationality {
                    Some(n) => match n {
                        Nationality::Saxon => self.warbands_saxon = amt,
                        Nationality::Scotti => self.warbands_scotti = amt,
                        Nationality::Briton => {
                            panic!("Trying to check for Available Briton Warbands")
                        }
                    },
                    None => panic!("Need a nationality to check for Available Warbands"),
                },
            },
        }
    }

    pub fn add(&mut self, force: Force, amt: u8) {
        match force {
            Force::Stronghold(stronghold) => match stronghold {
                TOWN => self.towns += amt,
                HILLFORT => self.hillforts += amt,
                FORT => self.forts += amt,
                SAXON_SETTLEMENT => self.settlements_saxon += amt,
                SCOTTI_SETTLEMENT => self.settlements_scotti += amt,
                _ => panic!("Unrecognized stronghold type {:?}", stronghold),
            },
            Force::Unit(class, nationality) => match class {
                UnitClass::Comitates => self.comitates += amt,
                UnitClass::Militia => self.militia += amt,
                UnitClass::Cavalry => self.cavalry += amt,
                UnitClass::Raider => match nationality {
                    Some(n) => match n {
                        Nationality::Saxon => self.raiders_saxon += amt,
                        Nationality::Scotti => self.raiders_scotti += amt,
                        Nationality::Briton => {
                            panic!("Trying to check for Available Briton Raiders")
                        }
                    },
                    None => panic!("Need a nationality to check for Available Raiders"),
                },
                UnitClass::Warband => match nationality {
                    Some(n) => match n {
                        Nationality::Saxon => self.warbands_saxon += amt,
                        Nationality::Scotti => self.warbands_scotti += amt,
                        Nationality::Briton => {
                            panic!("Trying to check for Available Briton Warbands")
                        }
                    },
                    None => panic!("Need a nationality to check for Available Warbands"),
                },
            },
        }
    }

    pub fn remove(&mut self, force: Force, amt: u8) {
        match force {
            Force::Stronghold(stronghold) => match stronghold {
                TOWN => self.towns -= amt,
                HILLFORT => self.hillforts -= amt,
                FORT => self.forts -= amt,
                SAXON_SETTLEMENT => self.settlements_saxon -= amt,
                SCOTTI_SETTLEMENT => self.settlements_scotti -= amt,
                _ => panic!("Unrecognized stronghold type {:?}", stronghold),
            },
            Force::Unit(class, nationality) => match class {
                UnitClass::Comitates => self.comitates -= amt,
                UnitClass::Militia => self.militia -= amt,
                UnitClass::Cavalry => self.cavalry -= amt,
                UnitClass::Raider => match nationality {
                    Some(n) => match n {
                        Nationality::Saxon => self.raiders_saxon -= amt,
                        Nationality::Scotti => self.raiders_scotti -= amt,
                        Nationality::Briton => {
                            panic!("Trying to check for Available Briton Raiders")
                        }
                    },
                    None => panic!("Need a nationality to check for Available Raiders"),
                },
                UnitClass::Warband => match nationality {
                    Some(n) => match n {
                        Nationality::Saxon => self.warbands_saxon -= amt,
                        Nationality::Scotti => self.warbands_scotti -= amt,
                        Nationality::Briton => {
                            panic!("Trying to check for Available Briton Warbands")
                        }
                    },
                    None => panic!("Need a nationality to check for Available Warbands"),
                },
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct Casualties {
    pub cavalry: u8,
}

#[derive(Clone, Debug)]
pub struct OutOfPlay {
    pub cavalry: u8,
}

#[derive(Clone, Debug)]
pub struct NotYetInPlay {
    pub comitates: u8,
}

#[derive(Clone, Debug)]
pub struct NiallNoigiallach {
    pub raiders: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Imperium {
    RomanRule(Dominance),
    Autonomy(Dominance),
    Fragmentation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
                let id = space["id"].as_str().unwrap().to_string();
                let name = space["name"].as_str().unwrap().to_string();
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
                    id.clone(),
                    name,
                    space_type,
                    terrain,
                    adj_spaces,
                    adj_road,
                    pop,
                    stronghold_sites,
                );
                game_map.land.insert(id, space);
            }
            "City" => {
                let id: String = space["id"].as_str().unwrap().to_string();
                let name: String = space["name"].as_str().unwrap().to_string();
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
                    id.clone(),
                    name,
                    space_type,
                    terrain,
                    adj_spaces,
                    adj_road,
                    pop,
                    stronghold_sites,
                );
                game_map.land.insert(id, space);
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
                for (site_name, site_piece) in
                    space["Stronghold Sites"].as_mapping().unwrap().iter()
                {
                    let site_name = site_name.as_str().unwrap();
                    let site_piece = site_piece.as_mapping().unwrap();
                    let site: &mut StrongholdSite =
                        land.stronghold_sites.get_mut(site_name).unwrap();
                    site.stronghold = match site_piece["Type"].as_str().unwrap() {
                        "Fort" => Some(Stronghold::new(
                            StrongholdClass::Fort,
                            Some(Player::Dux),
                            None,
                        )),
                        "Hillfort" => Some(Stronghold::new(
                            StrongholdClass::Hillfort,
                            Some(Player::Civitates),
                            Some(Nationality::Briton),
                        )),
                        "Town" => Some(Stronghold::new(
                            StrongholdClass::Town,
                            Some(Player::Civitates),
                            Some(Nationality::Briton),
                        )),
                        "Saxon Settlement" => Some(Stronghold::new(
                            StrongholdClass::Settlement,
                            Some(Player::Saxons),
                            Some(Nationality::Saxon),
                        )),
                        "Scotti Settlement" => Some(Stronghold::new(
                            StrongholdClass::Settlement,
                            Some(Player::Scotti),
                            Some(Nationality::Scotti),
                        )),
                        _ => panic!("Invalid stronghold type {}", site_name),
                    }
                }

                let unit_list: &serde_yaml::Mapping = space["Units"].as_mapping().unwrap();
                for unit in unit_list.keys() {
                    let unit: &str = unit.as_str().unwrap();
                    match unit {
                        "Cavalry" => {
                            let amt: u8 =
                                unit_list["Cavalry"].as_mapping().unwrap()["Without Plunder"]
                                    .as_u64()
                                    .unwrap() as u8;
                            for _ in 0..amt {
                                land.units.push(Unit {
                                    class: UnitClass::Cavalry,
                                    controller: Player::Dux,
                                    nationality: None,
                                    plunder: false,
                                });
                            }
                        }
                        "Militia" => {
                            let militia: &serde_yaml::Mapping =
                                unit_list["Militia"].as_mapping().unwrap();
                            let amt = militia["Without Plunder"].as_u64().unwrap() as u8;
                            for _ in 0..amt {
                                land.units.push(Unit {
                                    class: UnitClass::Militia,
                                    controller: Player::Civitates,
                                    nationality: Some(Nationality::Briton),
                                    plunder: false,
                                });
                            }
                        }
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
                    }
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

    let available: Available = Available {
        militia: civitates_available_mapping["Militia"].as_u64().unwrap() as u8,
        comitates: civitates_available_mapping
            .get("Comitates")
            .map_or(0, |v| v.as_u64().unwrap() as u8),
        towns: civitates_available_mapping["Towns"].as_u64().unwrap() as u8,
        hillforts: civitates_available_mapping["Hillforts"].as_u64().unwrap() as u8,
        refugees: markers["Refugees"].as_u64().unwrap() as u8,
        cavalry: dux_available_mapping["Cavalry"].as_u64().unwrap() as u8,
        forts: dux_available_mapping["Forts"].as_u64().unwrap() as u8,
        raiders_saxon: saxon_available_mapping["Raiders"].as_u64().unwrap() as u8,
        warbands_saxon: saxon_available_mapping["Warbands"].as_u64().unwrap() as u8,
        settlements_saxon: saxon_available_mapping["Settlements"].as_u64().unwrap() as u8,
        raiders_scotti: scotti_available_mapping["Raiders"].as_u64().unwrap() as u8,
        warbands_scotti: scotti_available_mapping["Warbands"].as_u64().unwrap() as u8,
        settlements_scotti: scotti_available_mapping["Settlements"].as_u64().unwrap() as u8,
    };

    let casualties: Casualties = Casualties {
        cavalry: holding_boxes["Dux"].as_mapping().unwrap()["Casualties"]
            .as_u64()
            .unwrap() as u8,
    };
    let out_of_play: OutOfPlay = OutOfPlay {
        cavalry: holding_boxes["Dux"].as_mapping().unwrap()["Out of play"]
            .as_u64()
            .unwrap() as u8,
    };
    let not_yet_in_play: NotYetInPlay = NotYetInPlay {
        comitates: holding_boxes["Civitates"].as_mapping().unwrap()["Not yet in play"]
            .as_u64()
            .unwrap() as u8,
    };
    let niall_noigiallach: NiallNoigiallach = NiallNoigiallach {
        raiders: holding_boxes["Scotti"].as_mapping().unwrap()["Niall Noigiallach"]
            .as_u64()
            .unwrap() as u8,
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
        available,
        casualties,
        out_of_play,
        not_yet_in_play,
        niall_noigiallach,
        roads_maintained,
    );
    return game_board;
}
