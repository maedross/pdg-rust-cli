use colored::Colorize;
use std::fmt;

use crate::concepts::{Nationality::Briton, Player::Civitates, UnitClass::{Comitates, Militia}};
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Player {
    Civitates,
    Dux,
    Saxons,
    Scotti,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::Civitates => write!(f, "{}", "Civitates".blue()),
            Player::Dux => write!(f, "{}", "Dux".red()),
            Player::Saxons => write!(f, "{}", "Saxons".black()),
            Player::Scotti => write!(f, "{}", "Scotti".green()),
        }
    }
}

impl fmt::Debug for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::Civitates => write!(f, "{}", "Civitates".blue()),
            Player::Dux => write!(f, "{}", "Dux".red()),
            Player::Saxons => write!(f, "{}", "Saxons".black()),
            Player::Scotti => write!(f, "{}", "Scotti".green()),
        }
    }
}

// Board
#[derive(Clone, Debug)]
pub struct Space<'a> {
    pub name: String,
    pub space_type: SpaceType,
    pub terrain: Option<Terrain>,
    pub adj_spaces: Vec<&'a Space<'a>>,
    pub adj_seas: Vec<&'a Sea<'a>>,
    pub pop: u8,
    pub max_pop: u8,
    pub top_prosp: u8,
    pub bottom_prosp: u8,
    pub stronghold_sites: Vec<StrongholdSite<'a>>,
    pub units: Vec<Unit>,
    pub control: Option<Player>,
}

struct PieceCount {
    units: Vec<Unit>,
    strongholds: Vec<Stronghold>,
}

#[derive(Clone, Debug)]
pub enum SpaceType {
    Region,
    City,
}

#[derive(Clone, Debug)]
pub struct Sea<'a> {
    name: String,
    patrol: bool,
    adj: Vec<&'a Space<'a>>,
}

#[derive(Clone, Debug)]
pub struct StrongholdSite<'a> {
    pub name: String,
    pub site_type: StrongholdSiteType,
    pub stronghold: Option<&'a Stronghold>,
}

#[derive(Clone, Debug)]
pub enum StrongholdSiteType {
    Hillfort,
    Town,
    City,
}

#[derive(Clone, Debug)]
pub enum Terrain {
    Clear,
    Fens,
    Hills,
}

// Components
#[derive(Clone, Debug)]
pub enum Nationality {
    Briton,
    Saxon,
    Scotti,
}

#[derive(Clone, Debug)]
pub struct Stronghold {
    pub controller: Player,
    pub class: StrongholdClass,
    pub nationality: Nationality,
    pub escalade: f32,
    pub garrison: u8,
    pub capacity: u8,
}

impl Stronghold {
    pub fn new(class: StrongholdClass, player: Option<Player>, nation: Option<Nationality>) -> Stronghold {
        match class {
            StrongholdClass::Fort => {
                Stronghold {
                    controller: Player::Dux,
                    class: class,
                    nationality: Nationality::Briton,
                    escalade: 1.,
                    garrison: 1,
                    capacity: 2,
                }
            },
            StrongholdClass::Hillfort => {
                Stronghold {
                    controller: Player::Civitates,
                    class: class,
                    nationality: Nationality::Briton,
                    escalade: 0.5,
                    garrison: 1,
                    capacity: 2,
                }
            },
            StrongholdClass::Town => {
                Stronghold {
                    controller: Player::Civitates,
                    class: class,
                    nationality: Nationality::Briton,
                    escalade: 0.5,
                    garrison: 2,
                    capacity: 4,
                }
            },
            StrongholdClass::Settlement => {
                Stronghold {
                    controller: player.unwrap(),
                    class: class,
                    nationality: nation.unwrap(),
                    escalade: 0.5,
                    garrison: 0,
                    capacity: 2,
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum StrongholdClass {
    Fort,
    Hillfort,
    Town,
    Settlement,
}

#[derive(Clone, Debug)]
pub struct Unit {
    pub designation: UnitClass,
    pub controller: Player,
    pub nationality: Nationality,
    pub plunder: bool,
}

#[derive(Clone, Debug)]
pub enum UnitClass {
    Cavalry,
    Comitates,
    Foederati,
    Militia,
    Raider,
    Warband,
}

impl Unit {
    pub fn con_militia(amt: u8) -> Vec<Unit> {
        let militia: Unit = Unit {
            designation: Militia,
            controller: Civitates,
            nationality: Briton,
            plunder: false,
        };
        let mut ret: Vec<Unit> = vec![];
        for _ in 0..amt {
            ret.push(militia.clone());
        }
        return ret;
    }

    pub fn con_comitates(amt: u8) -> Vec<Unit> {
        let militia: Unit = Unit {
            designation: Comitates,
            controller: Civitates,
            nationality: Briton,
            plunder: false,
        };
        let mut ret: Vec<Unit> = vec![];
        for _ in 0..amt {
            ret.push(militia.clone());
        }
        return ret;
    }
}