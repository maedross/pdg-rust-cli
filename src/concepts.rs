use colored::Colorize;
use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
            Player::Civitates => write!(f, "{}", "Civitates"),
            Player::Dux => write!(f, "{}", "Dux"),
            Player::Saxons => write!(f, "{}", "Saxons"),
            Player::Scotti => write!(f, "{}", "Scotti"),
        }
    }
}

// Components
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum Nationality {
    Briton,
    Saxon,
    Scotti,
}

pub enum Force {
    Stronghold(Stronghold),
    Unit(UnitClass, Option<Nationality>),
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct Stronghold {
    pub controller: Player,
    pub class: StrongholdClass,
    pub nationality: Option<Nationality>,
    pub escalade: f32,
    pub garrison: u8,
    pub capacity: u8,
}

impl Stronghold {
    pub fn new(
        class: StrongholdClass,
        player: Option<Player>,
        nation: Option<Nationality>,
    ) -> Stronghold {
        match class {
            StrongholdClass::Fort => Stronghold {
                controller: Player::Dux,
                class: class,
                nationality: Some(Nationality::Briton),
                escalade: 1.,
                garrison: 1,
                capacity: 2,
            },
            StrongholdClass::Hillfort => Stronghold {
                controller: Player::Civitates,
                class: class,
                nationality: Some(Nationality::Briton),
                escalade: 0.5,
                garrison: 1,
                capacity: 2,
            },
            StrongholdClass::Town => Stronghold {
                controller: Player::Civitates,
                class: class,
                nationality: Some(Nationality::Briton),
                escalade: 0.5,
                garrison: 2,
                capacity: 4,
            },
            StrongholdClass::Settlement => Stronghold {
                controller: player.unwrap(),
                class: class,
                nationality: nation,
                escalade: 0.5,
                garrison: 0,
                capacity: 2,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct StrongholdPiece {
    pub controller: Player,
    pub stronghold: Stronghold,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum StrongholdClass {
    Fort,
    Hillfort,
    Town,
    Settlement,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum UnitClass {
    Cavalry,
    Comitates,
    Militia,
    Raider,
    Warband,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Unit {
    pub class: UnitClass,
    pub controller: Player,
    pub nationality: Option<Nationality>,
    pub plunder: bool,
}

impl Unit {
    pub fn with_plunder(self) -> Unit {
        Unit {
            class: self.class,
            controller: self.controller,
            nationality: self.nationality,
            plunder: true,
        }
    }
}

pub const FORT: Stronghold = Stronghold{
    controller: Player::Dux,
    nationality: None,
    class: StrongholdClass::Fort,
    escalade: 1.0,
    garrison: 1,
    capacity: 2,
};
pub const HILLFORT: Stronghold = Stronghold{
    controller: Player::Civitates,
    nationality: Some(Nationality::Briton),
    class: StrongholdClass::Hillfort,
    escalade: 0.5,
    garrison: 1,
    capacity: 2,
};
pub const TOWN: Stronghold = Stronghold {
    controller: Player::Civitates,
    nationality: Some(Nationality::Briton),
    class: StrongholdClass::Town,
    escalade: 0.5,
    garrison: 2,
    capacity: 4,
};
pub const SAXON_SETTLEMENT: Stronghold = Stronghold {
    controller: Player::Saxons,
    nationality: Some(Nationality::Saxon),
    class: StrongholdClass::Settlement,
    escalade: 0.5,
    garrison: 0,
    capacity: 2
};
pub const SCOTTI_SETTLEMENT: Stronghold = Stronghold {
    controller: Player::Scotti,
    nationality: Some(Nationality::Scotti),
    class: StrongholdClass::Settlement,
    escalade: 0.5,
    garrison: 0,
    capacity: 2
};
pub const CIVITATES_SAXON_FOEDERATI_SETTLEMENT: Stronghold = Stronghold {
    controller: Player::Civitates,
    nationality: Some(Nationality::Saxon),
    class: StrongholdClass::Settlement,
    escalade: 0.5,
    garrison: 0,
    capacity: 2
};
pub const CIVITATES_SCOTTI_FOEDERATI_SETTLEMENT: Stronghold = Stronghold {
    controller: Player::Civitates,
    nationality: Some(Nationality::Scotti),
    class: StrongholdClass::Settlement,
    escalade: 0.5,
    garrison: 0,
    capacity: 2
};
pub const DUX_SAXON_FOEDERATI_SETTLEMENT: Stronghold = Stronghold {
    controller: Player::Dux,
    nationality: Some(Nationality::Saxon),
    class: StrongholdClass::Settlement,
    escalade: 0.5,
    garrison: 0,
    capacity: 2
};
pub const DUX_SCOTTI_FOEDERATI_SETTLEMENT: Stronghold = Stronghold {
    controller: Player::Dux,
    nationality: Some(Nationality::Scotti),
    class: StrongholdClass::Settlement,
    escalade: 0.5,
    garrison: 0,
    capacity: 2
};