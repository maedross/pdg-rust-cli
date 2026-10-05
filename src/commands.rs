use crate::board::{Available, Imperium};
use crate::feats::{INVITE_CIVITATTES, RULE};
use crate::sequence_of_play::Action;

use super::board::{Board, Space};
use super::concepts::{Player, StrongholdClass};
use dialoguer::{Input, Select};
use std::fmt;
use tracing::{Level, event, instrument};

//TODO could commands and feats be YAMLs? Would that be worth it or just suffering?

#[derive(Clone, Debug)]
pub struct Command {
    name: String,
    pub action: fn(&mut Board, bool, bool),
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub fn get_faction_commands(player: Player) -> Vec<Action> {
    match player {
        Player::Civitates => vec![MUSTER, MARCH_CIVITATES, TRADE, BATTLE_CIVITATES],
        Player::Dux => vec![TRAIN, MARCH_DUX, INTERCEPT, BATTLE_DUX],
        Player::Saxons => vec![RAID_SAXONS, RETURN_SAXONS, MARCH_SAXONS, BATTLE_SAXONS],
        Player::Scotti => vec![RAID_SCOTTI, RETURN_SCOTTI, MARCH_SCOTTI, BATTLE_SCOTTI],
    }
}

enum EXPCommand {
    Muster,
    March,
    Trade,
    Battle,
    Train,
    Intercept,
    Raid,
    Return,
}

const MUSTER: Action = Action::Command {
    command_name: "Muster",
    action: muster,
};
const MARCH_CIVITATES: Action = Action::Command {
    command_name: "March",
    action: civitates_march,
};
const TRADE: Action = Action::Command {
    command_name: "Trade",
    action: trade,
};
const BATTLE_CIVITATES: Action = Action::Command {
    command_name: "Battle",
    action: civitates_battle,
};

const TRAIN: Action = Action::Command {
    command_name: "Train",
    action: train,
};
const MARCH_DUX: Action = Action::Command {
    command_name: "March",
    action: dux_march,
};
const INTERCEPT: Action = Action::Command {
    command_name: "Intercept",
    action: intercept,
};
const BATTLE_DUX: Action = Action::Command {
    command_name: "Battle",
    action: dux_battle,
};

const RAID_SAXONS: Action = Action::Command {
    command_name: "Raid",
    action: saxon_raid,
};
const RETURN_SAXONS: Action = Action::Command {
    command_name: "Return",
    action: saxon_return,
};
const MARCH_SAXONS: Action = Action::Command {
    command_name: "March",
    action: saxon_march,
};
const BATTLE_SAXONS: Action = Action::Command {
    command_name: "Battle",
    action: saxon_battle,
};

const RAID_SCOTTI: Action = Action::Command {
    command_name: "Raid",
    action: scotti_raid,
};
const RETURN_SCOTTI: Action = Action::Command {
    command_name: "Return",
    action: scotti_return,
};
const MARCH_SCOTTI: Action = Action::Command {
    command_name: "March",
    action: scotti_march,
};
const BATTLE_SCOTTI: Action = Action::Command {
    command_name: "Battle",
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

// TODO: if doing recursion, need to pass in our shrinking list while keeping track of feats as well
// If not recursion, might still need to
// TODO: what would be really nice is to have actions be some sort of type whose name is displayed and have a function stored that can be called,
// so I don't have to do weird index arithmetic and tracking. Feats and Commands both. Feats in general and feats-per-space... sorta like how commands
// currently are. Hm. Do I smell an enum?
// So I need an enum handling Command, Command-per-space, Feat, and Feat-per-space? Should also probably handle Pass and Event
// Do these even need to be different enum values? Could they all be one Action struct? They behave the same way
// They might because they might contain different types of functions.
// Command returns a bool and modifies board state - could modify sequence of play state instead of returning, but would require extra arg (sop)
// Feat modifies board state
// Command per space modifies board state
// Feat per space modifies board state
// Pass modifies board state
// All these board state modifiers could end up returning the board state, especially when we shift to tracking histories
fn muster(board: &mut Board) {
  
}

pub fn issue_command(command: Action, board: &mut Board, limited: bool, feat_allowed: bool) -> bool {
    match command {
        Action::Command { command_name, action } => {
            let mut action_options: Vec<Action>;
            let mut executed_command: bool = false;
            let mut executed_feat: bool = false;

            let prompt: String;
            // Replace with function fetching appropriate feats
            if feat_allowed {
                if board.imperium == Imperium::Fragmentation {
                    action_options = vec![RULE];
                } else {
                    action_options = vec![RULE, INVITE_CIVITATTES];
                }
                prompt = format!("Select a Feat or a space in which to {}", command_name);
            } else {
                action_options = vec![];
                prompt = format!("Select a space in which to {}", command_name);
            }
            for space in board.map.land.keys() {
                action_options.push(Action::Command {
                    space_name: space.clone(),
                    action: muster, // TODO: Rework to action. Do I actually need the Command/CommandSpace distinction?
                });
            }

            loop {
                let selection_ind: usize = Select::new()
                    .with_prompt(prompt)
                    .items(&action_options)
                    .interact()
                    .unwrap();
                let selection: &Action = &action_options[selection_ind];
                match selection {
                    Action::Command { command_name, action } => {
                        println!("{}ing in {}", command_name, space_name);
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
        _ => panic!("Running a Command with a non-Command: {}", command),
    }
}

fn muster_space(space: &str, board: &mut Board) {
    println!("TODO: Execute Muster in space {}", space);
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
