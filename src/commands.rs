use colored::Colorize;


use crate::board::{Board, Imperium};
use crate::concepts::{Player, StrongholdClass};
use crate::feats::get_feats;
use crate::action::Action;

use dialoguer::{MultiSelect, Select};
use std::fmt;
use tracing::{Level, event, instrument};

//TODO could commands and feats be YAMLs? Would that be worth it or just suffering?

pub fn get_faction_commands(player: Player) -> Vec<Command> {
    match player {
        Player::Civitates => vec![
            Command::Muster,
            Command::CivitatesMarch,
            Command::Trade,
            Command::CivitatesBattle,
        ],
        Player::Dux => vec![
            Command::Train,
            Command::DuxMarch,
            Command::Intercept,
            Command::DuxBattle,
        ],
        Player::Saxons => vec![
            Command::SaxonRaid,
            Command::SaxonReturn,
            Command::SaxonMarch,
            Command::SaxonBattle,
        ],
        Player::Scotti => vec![
            Command::ScottiRaid,
            Command::ScottiReturn,
            Command::ScottiMarch,
            Command::ScottiBattle,
        ],
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
            Command::CivitatesMarch
            | Command::DuxMarch
            | Command::SaxonMarch
            | Command::ScottiMarch => write!(f, "March"),
            Command::Trade => write!(f, "Trade"),
            Command::CivitatesBattle
            | Command::DuxBattle
            | Command::SaxonBattle
            | Command::ScottiBattle => write!(f, "Battle"),
            Command::Train => write!(f, "Train"),
            Command::Intercept => write!(f, "Intercept"),
            Command::SaxonRaid | Command::ScottiRaid => write!(f, "Raid"),
            Command::SaxonReturn | Command::ScottiReturn => write!(f, "Return"),
        }
    }
}

pub fn get_command(command: Command) -> Action {
    match command {
        Command::Muster => Action::Command {
            command_name: Command::Muster,
            space: None,
            actions: vec![muster],
            filter: muster_filter,
        },
        Command::CivitatesMarch => Action::Command {
            command_name: Command::CivitatesMarch,
            space: None,
            actions: vec![civitates_march],
            filter: civitates_march_filter,
        },
        Command::Trade => Action::Command {
            command_name: Command::Trade,
            space: None,
            actions: vec![trade],
            filter: trade_filter,
        },
        Command::CivitatesBattle => Action::Command {
            command_name: Command::CivitatesBattle,
            space: None,
            actions: vec![civitates_battle],
            filter: civitates_battle_filter,
        },
        Command::Train => Action::Command {
            command_name: Command::Train,
            space: None,
            actions: vec![train],
            filter: train_filter,
        },
        Command::DuxMarch => Action::Command {
            command_name: Command::DuxMarch,
            space: None,
            actions: vec![dux_march],
            filter: dux_march_filter,
        },
        Command::Intercept => Action::Command {
            command_name: Command::Intercept,
            space: None,
            actions: vec![intercept],
            filter: intercept_filter,
        },
        Command::DuxBattle => Action::Command {
            command_name: Command::DuxBattle,
            space: None,
            actions: vec![dux_battle],
            filter: dux_battle_filter,
        },
        Command::SaxonRaid => Action::Command {
            command_name: Command::SaxonRaid,
            space: None,
            actions: vec![saxon_raid],
            filter: saxon_raid_filter,
        },
        Command::SaxonReturn => Action::Command {
            command_name: Command::SaxonReturn,
            space: None,
            actions: vec![saxon_return],
            filter: saxon_return_filter,
        },
        Command::SaxonMarch => Action::Command {
            command_name: Command::SaxonMarch,
            space: None,
            actions: vec![saxon_march],
            filter: saxon_march_filter,
        },
        Command::SaxonBattle => Action::Command {
            command_name: Command::SaxonBattle,
            space: None,
            actions: vec![saxon_battle],
            filter: saxon_battle_filter,
        },
        Command::ScottiRaid => Action::Command {
            command_name: Command::ScottiRaid,
            space: None,
            actions: vec![scotti_raid],
            filter: scotti_raid_filter,
        },
        Command::ScottiReturn => Action::Command {
            command_name: Command::ScottiReturn,
            space: None,
            actions: vec![scotti_return],
            filter: scotti_return_filter,
        },
        Command::ScottiMarch => Action::Command {
            command_name: Command::ScottiMarch,
            space: None,
            actions: vec![scotti_march],
            filter: scotti_march_filter,
        },
        Command::ScottiBattle => Action::Command {
            command_name: Command::ScottiBattle,
            space: None,
            actions: vec![scotti_battle],
            filter: scotti_battle_filter,
        },
    }
}

fn combine_space_lists(spaces: &mut Vec<Vec<String>>) -> Vec<String> {
    let mut s = vec![];
    loop {
        let new_s = spaces.pop();
        match new_s {
            Some(mut v) => {
                s.append(&mut v);
                s.sort();
                s.dedup();
            }
            None => return s,
        }
    }
}

fn muster_filter(board: &mut Board) -> Vec<String> {
    combine_space_lists(&mut vec![
        board.filter_spaces_pieces(Some(Player::Civitates), None),
        board.filter_spaces_stronghold(None, Some(Player::Civitates), None),
    ])
}

fn civitates_march_filter(board: &mut Board) -> Vec<String> {
    println!("{}", "DEBUG: Filter not yet implemented".yellow());
    board.map.land.clone().into_keys().collect::<Vec<String>>()
}

fn trade_filter(board: &mut Board) -> Vec<String> {
    println!("{}", "DEBUG: Filter not yet implemented".yellow());
    board.map.land.clone().into_keys().collect::<Vec<String>>()
}

fn civitates_battle_filter(board: &mut Board) -> Vec<String> {
    println!("{}", "DEBUG: Filter not yet implemented".yellow());
    board.map.land.clone().into_keys().collect::<Vec<String>>()
}

fn train_filter(board: &mut Board) -> Vec<String> {
    combine_space_lists(&mut vec![
        board.filter_spaces_stronghold(Some(StrongholdClass::Fort), None, None),
        match board.imperium {
            Imperium::Fragmentation => board.filter_spaces_control(Some(Player::Dux)),
            _ => board.filter_spaces_control(Some(Player::Civitates)),
        },
    ])
}

fn dux_march_filter(board: &mut Board) -> Vec<String> {
    println!("{}", "DEBUG: Filter not yet implemented".yellow());
    board.map.land.clone().into_keys().collect::<Vec<String>>()
}

fn intercept_filter(board: &mut Board) -> Vec<String> {
    println!("{}", "DEBUG: Filter not yet implemented".yellow());
    board.map.land.clone().into_keys().collect::<Vec<String>>()
}

fn dux_battle_filter(board: &mut Board) -> Vec<String> {
    println!("{}", "DEBUG: Filter not yet implemented".yellow());
    board.map.land.clone().into_keys().collect::<Vec<String>>()
}

fn saxon_raid_filter(board: &mut Board) -> Vec<String> {
    println!("{}", "DEBUG: Filter not yet implemented".yellow());
    board.map.land.clone().into_keys().collect::<Vec<String>>()
}

fn saxon_return_filter(board: &mut Board) -> Vec<String> {
    println!("{}", "DEBUG: Filter not yet implemented".yellow());
    board.map.land.clone().into_keys().collect::<Vec<String>>()
}

fn saxon_march_filter(board: &mut Board) -> Vec<String> {
    println!("{}", "DEBUG: Filter not yet implemented".yellow());
    board.map.land.clone().into_keys().collect::<Vec<String>>()
}

fn saxon_battle_filter(board: &mut Board) -> Vec<String> {
    println!("{}", "DEBUG: Filter not yet implemented".yellow());
    board.map.land.clone().into_keys().collect::<Vec<String>>()
}

fn scotti_raid_filter(board: &mut Board) -> Vec<String> {
    println!("{}", "DEBUG: Filter not yet implemented".yellow());
    board.map.land.clone().into_keys().collect::<Vec<String>>()
}

fn scotti_return_filter(board: &mut Board) -> Vec<String> {
    println!("{}", "DEBUG: Filter not yet implemented".yellow());
    board.map.land.clone().into_keys().collect::<Vec<String>>()
}

fn scotti_march_filter(board: &mut Board) -> Vec<String> {
    println!("{}", "DEBUG: Filter not yet implemented".yellow());
    board.map.land.clone().into_keys().collect::<Vec<String>>()
}

fn scotti_battle_filter(board: &mut Board) -> Vec<String> {
    println!("{}", "DEBUG: Filter not yet implemented".yellow());
    board.map.land.clone().into_keys().collect::<Vec<String>>()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SelectionOption {
    Done,
    Feat(Action),
    CommandSpace(String),
}

impl fmt::Display for SelectionOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectionOption::Done => write!(f, "Done"),
            SelectionOption::Feat(feat) => write!(f, "{}", feat),
            SelectionOption::CommandSpace(space) => write!(f, "{}", space),
        }
    }
}

// Ohhhhhhhh right, I was separating because of multistage commands like Train, Muster, Raid
// TODO: handle space limits (Raid)
// TODO: Can Raid and other Commands be selected emptily?
// TODO: Should in fact be using filters on spaces
// TODO: seem to be making it limited when shouldn't (first acting)
// TODO: Multiple levels of selection, for Intercept and Raid
// TODO: During execution loop, automatically proceed when Done is only option left
pub fn issue_command(
    command: Action,
    board: &mut Board,
    limited: bool,
    feat_allowed: bool,
) -> bool {
    match command.clone() {
        Action::Command {
            command_name,
            space: _,
            actions,
            filter: _,
        } => {
            let mut executed_command: bool = false;
            let mut executed_feat: bool = false;
            let mut command_spaces: Vec<String> = vec![];
            let mut feat_spaces: Vec<String> = vec![];
            let mut selected_feat: Option<Action> = None;

            let prompt: String;
            if feat_allowed {
                // If no selected feat, select from list of feats and command
                // Else go straight to command
                let mut action_options: Vec<Action> = get_feats(command_name, board.imperium);
                action_options.push(command.clone());

                prompt = format!("Select a Feat or proceed with {}", command_name);
                let selected_action = action_options[Select::new()
                    .with_prompt(&prompt)
                    .items(&action_options)
                    .interact()
                    .unwrap()]
                .clone();
                match selected_action {
                    Action::Command {
                        command_name,
                        space: _,
                        actions: _,
                        filter: _,
                    } => {
                        command_spaces = select_action_spaces(command, limited, board);
                        action_options = get_feats(command_name, board.imperium);
                        action_options.push(Action::Done);
                        let prompt = "Select Feat, or proceed to Command execution:";
                        let mut action_options = get_feats(command_name, board.imperium);
                        action_options.push(Action::Done);
                        let selection = action_options[Select::new()
                            .with_prompt(prompt)
                            .items(&action_options)
                            .interact()
                            .unwrap()]
                        .clone();
                        match selection {
                            Action::Feat { name: _, action: _ } => {
                                selected_feat = Some(selection.clone());
                                println!("DEBUG: selected_feat is {:?} from command first pick", selected_feat);
                                feat_spaces = select_action_spaces(selected_action, limited, board);
                            }
                            Action::Done => {}
                            _ => panic!(),
                        }
                    },
                    Action::Feat { name: _, action: _ } => {
                        selected_feat = Some(selected_action.clone());
                        println!("DEBUG: selected_feat is {:?} from feat first pick", selected_feat);
                        feat_spaces = select_action_spaces(selected_action, limited, board);
                        command_spaces = select_action_spaces(command, limited, board);
                    }
                    _ => panic!(),
                };
            } else {
                command_spaces = select_action_spaces(command, limited, board);
            }
            println!("Moving on to resolving the actions in their chosen spaces...");
            // TODO: if single command space jump straight to action
            for command_step in actions {
                let mut action_options = vec![];
                let prompt: &str;
                if selected_feat != None && !executed_feat {
                    println!("DEBUG: adding {:?} to top of action options", selected_feat);
                    action_options.push(SelectionOption::Feat(selected_feat.clone().unwrap()));
                    prompt =
                        "Select a space to execute your Command, or choose to execute your Feat:";
                } else {
                    prompt = "Select a space to execute your Command:";
                }
                action_options.append(
                    &mut command_spaces
                        .iter()
                        .map(|s| SelectionOption::CommandSpace(s.clone()))
                        .collect(),
                );
                loop {
                    let selection_index: usize = Select::new()
                        .with_prompt(prompt)
                        .items(&action_options)
                        .interact()
                        .unwrap();
                    let selection = &action_options[selection_index];
                    match selection {
                        SelectionOption::CommandSpace(space) => {
                            // execute command in space
                            // mark command as executed
                            // remove space from action options
                            (command_step)(space.clone(), board);
                            if !executed_command {
                                executed_command = true;
                                action_options.push(SelectionOption::Done);
                            }
                            action_options.remove(selection_index);
                        }
                        SelectionOption::Feat(feat) => {
                            // execute feat
                            // mark feat as executed
                            // remove feat from action options
                            println!("FEATS NOT YET IMPLEMENTED");
                            println!("Mock executing {}", feat);
                            if !executed_feat {
                                executed_feat = true;
                            }
                            action_options.remove(selection_index);
                        }
                        SelectionOption::Done => break,
                    }
                }
            }
            return executed_feat;
        }
        _ => panic!("Running a Command with a non-Command: {:?}", command),
    }
}

pub fn select_action_spaces(action: Action, limited: bool, board: &mut Board) -> Vec<String> {
    match action {
        Action::Command {
            command_name,
            space: _,
            actions: _,
            filter,
        } => {
            let mut selected_spaces: Vec<String> = vec![];
            let space_options: Vec<String> = filter(board);
            if !limited {
                let multi_selection: Vec<usize> = MultiSelect::new()
                    .with_prompt(format!("Select unlimited spaces for {} (a to (de)select all, space to de(select) one, enter to submit):", command_name))
                    .items(&space_options)
                    .interact()
                    .unwrap();
                for i in multi_selection {
                    selected_spaces.push(space_options[i].clone());
                }
                selected_spaces.sort();
            } else {
                let selection = Select::new()
                    .with_prompt(format!("Select space for {}:", command_name))
                    .items(&space_options)
                    .interact()
                    .unwrap();
                selected_spaces.push(space_options[selection].clone());
            };
            selected_spaces
        }
        Action::Feat { name, action: _ } => {
            println!("FEATS NOT YET IMPLEMENTED");
            println!("Returning no spaces for {}", name);
            vec![]
        },
        _ => panic!(),
    }
}

fn muster(space: String, board: &mut Board) {
    println!("{}Mustering in {}", "UNIMPLEMENTED: ".yellow(), space);
}

fn civitates_march(space: String, board: &mut Board) {
    println!("{}Civitates Marching in {}", "UNIMPLEMENTED: ".yellow(), space);
}

fn trade(space: String, board: &mut Board) {
    println!("{}Trading in {}", "UNIMPLEMENTED: ".yellow(), space);
}

fn civitates_battle(space: String, board: &mut Board) {
    println!("{}Civitates Battling in {}", "UNIMPLEMENTED: ".yellow(), space);
}

fn train(space: String, board: &mut Board) {
    println!("Training")
}

fn dux_march(space: String, board: &mut Board) {
    println!("{}Dux Marching in {}", "UNIMPLEMENTED: ".yellow(), space);
}

fn intercept(space: String, board: &mut Board) {
    println!("{}Intercepting in {}", "UNIMPLEMENTED: ".yellow(), space);
}

fn dux_battle(space: String, board: &mut Board) {
    println!("{}Dux Battling in {}", "UNIMPLEMENTED: ".yellow(), space);
}

fn saxon_raid(space: String, board: &mut Board) {
    println!("{}Saxons Raiding in {}", "UNIMPLEMENTED: ".yellow(), space);
}

fn saxon_return(space: String, board: &mut Board) {
    println!("{}Saxons Returning in {}", "UNIMPLEMENTED: ".yellow(), space);
}

fn saxon_march(space: String, board: &mut Board) {
    println!("{}Saxons Marching in {}", "UNIMPLEMENTED: ".yellow(), space);
}

fn saxon_battle(space: String, board: &mut Board) {
    println!("{}Saxons Battling in {}", "UNIMPLEMENTED: ".yellow(), space);
}

fn scotti_raid(space: String, board: &mut Board) {
    println!("{}Scotti Raiding in {}", "UNIMPLEMENTED: ".yellow(), space);
}

fn scotti_return(space: String, board: &mut Board) {
    println!("{}Scotti Returning in {}", "UNIMPLEMENTED: ".yellow(), space);
}

fn scotti_march(space: String, board: &mut Board) {
    println!("{}Scotti Marching in {}", "UNIMPLEMENTED: ".yellow(), space);
}

fn scotti_battle(space: String, board: &mut Board) {
    println!("{}Scotti Battling in {}", "UNIMPLEMENTED: ".yellow(), space);
}


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
