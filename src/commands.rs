use crate::board::{Available, Imperium};
use crate::feats::{get_feats};
use crate::sequence_of_play::Action;

use super::board::{Board, Space};
use super::concepts::{Player, StrongholdClass};
use dialoguer::{Input, Select};
use std::fmt;
use tracing::{Level, event, instrument};

//TODO could commands and feats be YAMLs? Would that be worth it or just suffering?

pub fn get_faction_commands(player: Player) -> Vec<Action> {
    match player {
        Player::Civitates => vec![MUSTER, MARCH_CIVITATES, TRADE, BATTLE_CIVITATES],
        Player::Dux => vec![TRAIN, MARCH_DUX, INTERCEPT, BATTLE_DUX],
        Player::Saxons => vec![RAID_SAXONS, RETURN_SAXONS, MARCH_SAXONS, BATTLE_SAXONS],
        Player::Scotti => vec![RAID_SCOTTI, RETURN_SCOTTI, MARCH_SCOTTI, BATTLE_SCOTTI],
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    Muster,
    CivitatesMarch,
    Trade,
    CivitatesBattle,
    Train,
    DuxMarch,
    Intercept,
    DuxBattle,
    SaxonRaid,
    SaxonReturn,
    SaxonMarch,
    SaxonBattle,
    ScottiRaid,
    ScottiReturn,
    ScottiMarch,
    ScottiBattle,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Muster => write!(f, "Muster"),
            Command::CivitatesMarch | Command::DuxMarch | Command::SaxonMarch | Command::ScottiMarch => write!(f, "March"),
            Command::Trade => write!(f, "Trade"),
            Command::CivitatesBattle | Command::DuxBattle | Command::SaxonBattle | Command::ScottiBattle => write!(f, "Battle"),
            Command::Train => write!(f, "Train"),
            Command::Intercept => write!(f, "Intercept"),
            Command::SaxonRaid | Command::ScottiRaid => write!(f, "Raid"),
            Command::SaxonReturn | Command::ScottiReturn => write!(f, "Return"),
        }
    }
}

const MUSTER: Action = Action::Command {
    command_name: Command::Muster,
    space: None,
    action: muster,
};
const MARCH_CIVITATES: Action = Action::Command {
    command_name: Command::CivitatesMarch,
    space: None,
    action: civitates_march,
};
const TRADE: Action = Action::Command {
    command_name: Command::Trade,
    space: None,
    action: trade,
};
const BATTLE_CIVITATES: Action = Action::Command {
    command_name: Command::CivitatesBattle,
    space: None,
    action: civitates_battle,
};

const TRAIN: Action = Action::Command {
    command_name: Command::Train,
    space: None,
    action: train,
};
const MARCH_DUX: Action = Action::Command {
    command_name: Command::DuxMarch,
    space: None,
    action: dux_march,
};
const INTERCEPT: Action = Action::Command {
    command_name: Command::Intercept,
    space: None,
    action: intercept,
};
const BATTLE_DUX: Action = Action::Command {
    command_name: Command::DuxBattle,
    space: None,
    action: dux_battle,
};

const RAID_SAXONS: Action = Action::Command {
    command_name: Command::SaxonRaid,
    space: None,
    action: saxon_raid,
};
const RETURN_SAXONS: Action = Action::Command {
    command_name: Command::SaxonReturn,
    space: None,
    action: saxon_return,
};
const MARCH_SAXONS: Action = Action::Command {
    command_name: Command::SaxonMarch,
    space: None,
    action: saxon_march,
};
const BATTLE_SAXONS: Action = Action::Command {
    command_name: Command::SaxonBattle,
    space: None,
    action: saxon_battle,
};

const RAID_SCOTTI: Action = Action::Command {
    command_name: Command::ScottiRaid,
    space: None,
    action: scotti_raid,
};
const RETURN_SCOTTI: Action = Action::Command {
    command_name: Command::ScottiReturn,
    space: None,
    action: scotti_return,
};
const MARCH_SCOTTI: Action = Action::Command {
    command_name: Command::ScottiMarch,
    space: None,
    action: scotti_march,
};
const BATTLE_SCOTTI: Action = Action::Command {
    command_name: Command::ScottiBattle,
    space: None,
    action: scotti_battle,
};
/*
    HOW COMMAND EXECUTION WORKS

    Commands have a default Option<u8> number of spaces - None means unlimited
    Limited sets that max to 1

    Display all the spaces we can select to do a Command
    These are spaces meeting target requirements that have not already been met

    IF we are allowed a Feat
        IF we have not already chosen one
            Also display a list of all spaces we can do all Feats
        Else we have already chosen one
            If there are spaces remaining to perform it
                Display those

    Need to know:
    * Where have we already executed Commands
    * Where have we already executed Feats
    * What Feat did we execute

    Haskell patterning seems the way to go?

    Input: board, limited, selected_command_spaces: Option<Vec>

    Output: whether a feat was taken

    Side effects: board transformation

    For now can use Haskell architecture with function runner calling real function, because I am not immediately sure if selected
    spaces will always be the same type. Can reduce later.
*/

fn muster(board: &mut Board) {
    println!("Mustering");
}

// TODO: handle space limits (Raid)
// TODO: Can Raid and other Commands be selected emptily?
// TODO: Should in fact be using filters on spaces
pub fn issue_command(command: Action, board: &mut Board, limited: bool, feat_allowed: bool) -> bool {
    match command {
        Action::Command { command_name, space: _, action } => {
            let mut action_options: Vec<Action>;
            let mut executed_command: bool = false;
            let mut executed_feat: bool = false;

            let prompt: String;
            // Replace with function fetching appropriate feats
            if feat_allowed {
                action_options = get_feats(command_name, board.imperium);
                prompt = format!("Select a Feat or a space in which to {}", command_name);
            } else {
                action_options = vec![];
                prompt = format!("Select a space in which to {}", command_name);
            }
            for space in board.map.land.keys() {
                action_options.push(Action::Command {
                    command_name,
                    space: Some(space.clone()),
                    action: action,
                });
            }

            loop {
                let selection_ind: usize = Select::new()
                    .with_prompt(&prompt)
                    .items(&action_options)
                    .interact()
                    .unwrap();
                let selection: &Action = &action_options[selection_ind];
                match selection {
                    Action::Command { command_name, space, action } => {
                        println!("{}ing in {}", command_name, space.as_ref().unwrap());
                        (action)(board);
                        action_options.remove(selection_ind);
                        if !executed_command {
                            let mut new_action_options: Vec<Action> = vec![Action::Done];
                            new_action_options.append(&mut action_options);
                            action_options = new_action_options;
                            executed_command = true;
                        }
                        if limited {
                            break;
                        }
                    }
                    Action::Feat { name, action } => {
                        println!("Performing Feat {}", name);
                        action_options.retain(|a| match a {
                            Action::Feat { name: _, action: _ } => false,
                            _ => true,
                        });
                        executed_feat = true;
                    }
                    Action::Done => break,
                    _ => panic!("Illegal selection {}", selection),
                }
            }
            return executed_feat;
        }
        _ => panic!("Running a Command with a non-Command: {:?}", command),
    }
}

fn civitates_march(board: &mut Board) {
    println!("Civitates Marching");
}

fn trade(board: &mut Board) {
    println!("Trading");
}

fn civitates_battle(board: &mut Board) {
    println!("Civitates Battling");
}

fn train(board: &mut Board) {
    println!("Training")
}

fn dux_march(board: &mut Board) {
    println!("Dux Marching");
}

fn intercept(board: &mut Board) {
    println!("Intercepting")
}

fn dux_battle(board: &mut Board) {
    println!("Dux Battling");
}

fn saxon_raid(board: &mut Board) {
    println!("Saxons Raiding");
}

fn saxon_return(board: &mut Board) {
    println!("Saxons Returning");
}

fn saxon_march(board: &mut Board) {
    println!("Saxons Marching");
}

fn saxon_battle(board: &mut Board) {
    println!("Saxons Battling");
}

fn scotti_raid(board: &mut Board) {
    println!("Scotti Raiding");
}

fn scotti_return(board: &mut Board) {
    println!("Scotti Returning");
}

fn scotti_march(board: &mut Board) {
    println!("Scotti Marching");
}

fn scotti_battle(board: &mut Board) {
    println!("Scotti Battling");
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
