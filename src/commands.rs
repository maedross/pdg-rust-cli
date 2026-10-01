use crate::board::Available;

use super::board::{Board, Imperium, Space};
use super::concepts::{Player, StrongholdClass};
use super::feats::Feat;
use dialoguer::{Input, MultiSelect, Select};
use std::fmt;
use tracing::{Level, event, instrument};

//TODO could commands and feats be YAMLs? Would that be worth it or just suffering?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    Train,
    March,
    Intercept,
    Battle,
    Muster,
    Trade,
    Raid,
    Return,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Train => write!(f, "Train"),
            Command::March => write!(f, "March"),
            Command::Intercept => write!(f, "Intercept"),
            Command::Battle => write!(f, "Battle"),
            Command::Muster => write!(f, "Muster"),
            Command::Trade => write!(f, "Trade"),
            Command::Raid => write!(f, "Raid"),
            Command::Return => write!(f, "Return"),
        }
    }
}

pub fn get_faction_commands(player: Player, feat: Option<Feat>) -> Vec<Command> {
    match feat {
        None => match player {
            Player::Civitates => vec![
                Command::Muster,
                Command::March,
                Command::Trade,
                Command::Battle,
            ],
            Player::Dux => vec![
                Command::Train,
                Command::March,
                Command::Intercept,
                Command::Battle,
            ],
            Player::Saxons => vec![
                Command::Raid,
                Command::Return,
                Command::March,
                Command::Battle,
            ],
            Player::Scotti => vec![
                Command::Raid,
                Command::Return,
                Command::March,
                Command::Battle,
            ],
        },
        Some(f) => match (f, player) {
            (Feat::Rule, Player::Civitates) => {
                vec![Command::Muster, Command::March, Command::Trade]
            }
            (Feat::Invite, Player::Civitates) => {
                vec![Command::Muster, Command::March, Command::Trade]
            }
            (Feat::Reinforce, Player::Civitates) => vec![Command::Battle],
            (Feat::Pillage, Player::Civitates) => vec![Command::March, Command::Battle],
            (Feat::Build, Player::Dux) => vec![Command::Train, Command::March],
            (Feat::Invite, Player::Dux) => vec![Command::Train, Command::March, Command::Intercept],
            (Feat::Requisition, Player::Dux) => {
                vec![Command::Train, Command::March, Command::Battle]
            }
            (Feat::Retaliate, Player::Dux) => vec![Command::Intercept, Command::Battle],
            (Feat::Settle, Player::Saxons) => vec![Command::Return, Command::March],
            (Feat::Surprise, Player::Saxons) => vec![Command::Raid, Command::Battle],
            (Feat::Ravage, Player::Saxons) => vec![Command::Raid, Command::Battle],
            (Feat::ShieldWall, Player::Saxons) => vec![Command::Battle],
            (Feat::Settle, Player::Scotti) => vec![Command::Return, Command::March],
            (Feat::Surprise, Player::Scotti) => vec![Command::Raid, Command::Battle],
            (Feat::Ransom, Player::Scotti) => vec![Command::Raid, Command::Battle],
            (Feat::Entreat, Player::Scotti) => vec![Command::Return, Command::Battle],
            _ => panic!("Invalid pairing of Feat and Player: {:?}, {:?}", f, player),
        },
    }
}

fn muster(board: &mut Board) {
    todo!()
}

fn civitates_march(board: &mut Board) {
    todo!()
}

fn trade(board: &mut Board) {
    todo!()
}

fn civitates_battle(board: &mut Board) {
    todo!()
}

fn train(board: &mut Board) {
    todo!()
}

fn dux_march(board: &mut Board) {
    todo!()
}

fn intercept(board: &mut Board) {
    todo!()
}

fn dux_battle(board: &mut Board) {
    todo!()
}

fn saxon_raid(board: &mut Board) {
    todo!()
}

fn saxon_return(board: &mut Board) {
    todo!()
}

fn saxon_march(board: &mut Board) {
    todo!()
}

fn saxon_battle(board: &mut Board) {
    todo!()
}

fn scotti_raid(board: &mut Board) {
    todo!()
}

fn scotti_return(board: &mut Board) {
    todo!()
}

fn scotti_march(board: &mut Board) {
    todo!()
}

fn scotti_battle(board: &mut Board) {
    todo!()
}

// TODO: Check available when adding units
fn muster_units(loc: &mut Space, wealth: u8, avail: &mut Available) {
    let mut cubes_to_place = 0;

    for stronghold_site in loc.stronghold_sites.values() {
        match stronghold_site.stronghold {
            Some(s) => match s.class {
                StrongholdClass::Hillfort => cubes_to_place += 1,
                StrongholdClass::Town => cubes_to_place += 1,
                _ => {}
            },
            None => {}
        }
    }

    match loc.control {
        Some(p) => {
            if p == Player::Civitates {
                cubes_to_place += loc.pop;
            }
        }
        None => {}
    }
    // TODO: Make this an option
    println!(
        "Placing {} cubes. Place Comitates instead of Militia?\nEach Comitates costs 1 Wealth.\nCurrent Wealth: {}",
        cubes_to_place, wealth
    );
    let mut num_com: Result<u8, std::num::ParseIntError>;
    loop {
        let comitates_to_place: String = Input::new()
            .allow_empty(true)
            .with_prompt("Enter number of Comitates to place instead of Militia (default: 0)")
            .interact()
            .unwrap();
        if comitates_to_place == "" {
            num_com = Ok(0);
        } else {
            num_com = comitates_to_place.parse::<u8>();
        }
        match num_com {
            Ok(n) => {
                if n > cubes_to_place {
                    println!(
                        "Error: tried placing {} Comitates but there are only {} cubes to place",
                        n, cubes_to_place
                    );
                }
                if n > wealth {
                    println!(
                        "Error: tried placing {} Comitates but may only spend {} Wealth",
                        n, wealth
                    );
                } else {
                    println!("Placed {} Comitates", num_com.unwrap());
                    break;
                }
            }
            _ => println!("Invalid input, must enter a non-negative integer"),
        }
    }

    // resulting_loc
    //     .units
    //     .append(&mut Unit::con_militia(cubes_to_place));
}

// fn muster_strongholds(loc: Space) -> Space {}

/*
#[cfg(test)]
mod tests {
    use super::super::board::{Space, SpaceType, StrongholdSite, StrongholdSiteType, Terrain};
    use super::super::concepts::{Player, Stronghold};

    use super::*;

    #[test]
    fn test_muster() {
        let town: Stronghold = Stronghold::new(StrongholdClass::Town, None, None);
        let aquae_sulis: StrongholdSite = StrongholdSite {
            name: String::from("Aquae Sulis"),
            site_type: StrongholdSiteType::Town,
            stronghold: Some(town),
        };
        let south_cadbury: StrongholdSite = StrongholdSite {
            name: String::from("South Cadbury"),
            site_type: StrongholdSiteType::Hillfort,
            stronghold: None,
        };

        let avail: CivitatesHolding = CivitatesHolding::blank();

        let test_space: Space = Space {
            id: 0,
            name: String::from("Durotriges"),
            space_type: SpaceType::Region,
            terrain: Some(Terrain::Clear),
            adj_spaces: vec![],
            adj_road: vec![],
            adj_seas: vec![],
            pop: 2,
            max_pop: 3,
            top_prosp: 2,
            bottom_prosp: 2,
            stronghold_sites: vec![aquae_sulis, south_cadbury],
            units: vec![],
            control: Some(Player::Civitates),
        };
        let (after, _): (Space<'_>, u8) = muster(test_space, 2, avail);
        assert_eq!(after.units.len(), 3);
    }
}
*/
