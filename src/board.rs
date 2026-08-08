use super::concepts::{Player, Stronghold, Unit};
use serde::{Deserialize, Serialize};
use serde_yaml::{self, Value};
use std::{collections::HashMap, fs, str::FromStr, vec};

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

pub struct Map {
    land: HashMap<u8, Space>,
    off_map_land: HashMap<u8, OffMapLandSpace>,
    seas: HashMap<u8, Sea>,
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
    pub id: u8,
    pub name: String,
    pub space_type: SpaceType,
    pub terrain: Option<Terrain>,
    pub adj_spaces: Vec<u8>,
    pub adj_road: Vec<u8>,
    pub pop: u8,
    #[serde(default)]
    pub max_pop: u8,
    #[serde(default)]
    pub top_prosp: u8,
    #[serde(default)]
    pub bottom_prosp: u8,
    pub stronghold_sites: Vec<StrongholdSite>,
    #[serde(default)]
    pub units: Vec<Unit>,
    #[serde(default)]
    pub control: Option<Player>,
}

impl<'a> Space {
    fn new(
        id: u8,
        name: &str,
        space_type: SpaceType,
        terrain: Option<Terrain>,
        adj_spaces: Vec<u8>,
        adj_road: Vec<u8>,
        pop: u8,
        stronghold_sites: Vec<StrongholdSite>,
    ) -> Space {
        Space {
            id,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    id: u8,
    name: String,
    patrol_spaces: Vec<u8>,
    adj: Vec<u8>,
}

impl OffMapLandSpace {
    fn new(id: u8, name: &str) -> OffMapLandSpace {
        OffMapLandSpace {
            id,
            name: name.to_string(),
            patrol_spaces: vec![],
            adj: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Sea {
    id: u8,
    name: String,
    patrol: bool,
    adj: Vec<u8>,
}

impl Sea {
    fn new(id: u8, name: &str) -> Sea {
        Sea {
            id,
            name: name.to_string(),
            patrol: false,
            adj: vec![],
        }
    }
}

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
pub struct CivitatesAvailable {
    pub militia: u8,
    pub comitates: u8,
    pub towns: u8,
    pub hillforts: u8,
    pub refugees: u8,
}

pub struct CivitatesNotYetInPlay {
    pub comitates: u8,
}

pub struct ScottiAvailable {
    pub raiders: u8,
    pub warbands: u8,
    pub settlements: u8,
    pub max_settlements: u8,
}

pub struct ScottiNiallNoigiallach {
    pub raiders: u8,
}

pub struct SaxonsAvailable {
    pub raiders: u8,
    pub warbands: u8,
    pub settlements: u8,
    pub max_settlements: u8,
}

pub struct DuxAvailable {
    pub cavalry: u8,
    pub forts: u8,
}

pub struct DuxCasualties {
    pub cavalry: u8,
}

pub struct DuxOutOfPlay {
    pub cavalry: u8,
}

pub enum Imperium {
    RomanRule(Dominance),
    Autonomy(Dominance),
    Fragmentation,
}

#[derive(Clone, Copy, Debug)]
pub enum Dominance {
    Military,
    Civilian,
    None,
}

pub fn build_map_from_yaml(file_path: &str) {
    let contents = fs::read_to_string(file_path).unwrap();
    let values: Vec<Value> = serde_yaml::from_str(&contents).unwrap();
    let mut game_map: Map = Map::new();
    for v in values {
        let space: &serde_yaml::Mapping = v.as_mapping().unwrap();
        let space_type: &str = space["space_type"].as_str().unwrap();
        match space_type {
            "Region" => {
                let id: u8 = space["id"].as_u64().unwrap() as u8;
                let name: &str = space["name"].as_str().unwrap();
                let space_type: SpaceType = SpaceType::Region;
                let terrain: Option<Terrain> = Some(Terrain::from_str(space["Terrain"].as_str().unwrap()).unwrap());
                let pop: u8 = space["pop"].as_u64().unwrap() as u8;
                let mut stronghold_sites: Vec<StrongholdSite> = vec![];
                for s in space["stronghold_sites"].as_sequence().unwrap() {
                    let name: &str = s["name"].as_str().unwrap();
                    let site_type: StrongholdSiteType = StrongholdSiteType::from_str(s["site_type"].as_str().unwrap()).unwrap();
                    let site: StrongholdSite = StrongholdSite::new(name, site_type);
                    stronghold_sites.push(site);
                }
                let adj_spaces: Vec<u8> = space["adj_spaces"].as_sequence().unwrap().into_iter().map(|i| i.as_u64().unwrap() as u8).collect();
                let adj_road: Vec<u8> = space["adj_road"].as_sequence().unwrap().into_iter().map(|i| i.as_u64().unwrap() as u8).collect();
                let space: Space = Space::new(id, name, space_type, terrain, adj_spaces, adj_road, pop, stronghold_sites);
                game_map.land.insert(id, space);
            }
            "City" => {
                let id: u8 = space["id"].as_u64().unwrap() as u8;
                let name: &str = space["name"].as_str().unwrap();
                let space_type: SpaceType = SpaceType::Region;
                let terrain: Option<Terrain> = None;
                let pop: u8 = space["pop"].as_u64().unwrap() as u8;
                let mut stronghold_sites: Vec<StrongholdSite> = vec![];
                for s in space["stronghold_sites"].as_sequence().unwrap() {
                    let name: &str = s["name"].as_str().unwrap();
                    let site_type: StrongholdSiteType = StrongholdSiteType::from_str(s["site_type"].as_str().unwrap()).unwrap();
                    let site: StrongholdSite = StrongholdSite::new(name, site_type);
                    stronghold_sites.push(site);
                }
                let adj_spaces: Vec<u8> = space["adj_spaces"].as_sequence().unwrap().into_iter().map(|i| i.as_u64().unwrap() as u8).collect();
                let adj_road: Vec<u8> = space["adj_road"].as_sequence().unwrap().into_iter().map(|i| i.as_u64().unwrap() as u8).collect();
                let space: Space = Space::new(id, name, space_type, terrain, adj_spaces, adj_road, pop, stronghold_sites);
                game_map.land.insert(id, space);
            }
            "Sea" => {
                let id: u8 = space["id"].as_u64().unwrap() as u8;
                let name: &str = space["name"].as_str().unwrap();
                let sea: Sea = Sea::new(id, name);
                game_map.seas.insert(id, sea);
            }
            "Off map land" => {
                let id: u8 = space["id"].as_u64().unwrap() as u8;
                let name: &str = space["name"].as_str().unwrap();
                let off_map_land = OffMapLandSpace::new(id, name);
                game_map.off_map_land.insert(id, off_map_land);
            }
            _ => panic!("Invalid space type: {}", space_type),
        }
    }
}
