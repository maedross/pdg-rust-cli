use dialoguer::Select;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use tracing::{Level, event, instrument};

use crate::board::{Board, build_scenario_from_yaml};
use crate::commands;

use super::concepts::Player;
use super::events::{Event, EventType};
use Player::{Civitates, Dux, Saxons, Scotti};

use PlayerState::Eligible;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AvailableActionState {
    Start,
    A,
    B,
    C,
    End,
}

#[derive(Clone, Debug)]
struct AvailableActions {
    a: Vec<Action>,
    state: AvailableActionState,
}

impl AvailableActions {
    fn new() -> Self {
        AvailableActions {
            a: vec![
                Action::Pass,
                Action::CommandOnly,
                Action::CommandFeat,
                Action::Event,
            ],
            state: AvailableActionState::Start,
        }
    }

    fn update_available_actions(self, selection: Option<Action>) -> AvailableActions {
        match self.state {
            AvailableActionState::Start => match selection.unwrap() {
                Action::Pass => self,
                Action::CommandOnly => AvailableActions {
                    a: vec![Action::Pass, Action::LimitedCommand],
                    state: AvailableActionState::A,
                },
                Action::CommandFeat => AvailableActions {
                    a: vec![Action::Pass, Action::Event, Action::LimitedCommand],
                    state: AvailableActionState::B,
                },
                Action::Event => AvailableActions {
                    a: vec![Action::Pass, Action::CommandFeat],
                    state: AvailableActionState::C,
                },
                _ => panic!("Invalid selected action for start"),
            },
            AvailableActionState::A => match selection.unwrap() {
                Action::Pass => self,
                Action::LimitedCommand => AvailableActions {
                    a: vec![],
                    state: AvailableActionState::End,
                },
                _ => panic!("Invalid selected action from Command Only"),
            },
            AvailableActionState::B => match selection.unwrap() {
                Action::Pass => self,
                Action::Event => AvailableActions {
                    a: vec![],
                    state: AvailableActionState::End,
                },
                Action::LimitedCommand => AvailableActions {
                    a: vec![],
                    state: AvailableActionState::End,
                },
                _ => panic!("Invalid selected action from Command + Feat"),
            },
            AvailableActionState::C => match selection.unwrap() {
                Action::Pass => self,
                Action::CommandFeat => AvailableActions {
                    a: vec![],
                    state: AvailableActionState::End,
                },
                _ => panic!("Invalid selected action from Event"),
            },
            AvailableActionState::End => {
                panic!("We're finished with the round, just make a new AvailableActions")
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Action {
    Pass,
    CommandOnly,
    LimitedCommand,
    CommandFeat,
    Event,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Pass => write!(f, "Pass"),
            Action::CommandOnly => write!(f, "CommandOnly"),
            Action::LimitedCommand => write!(f, "LimitedCommand"),
            Action::CommandFeat => write!(f, "CommandFeat"),
            Action::Event => write!(f, "Event"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PlayerState {
    Eligible,
    Passed,
    Acted,
    Ineligible,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SequenceOfPlayState {
    CheckEndRound,
    CheckPlayerStatus,
    ChoosingAction,
    Acting,
    ResetEligibility,
    AdvanceEvents,
    Epoch,
}

#[derive(Clone)]
pub struct SequenceOfPlay {
    player_eligibilities: HashMap<Player, PlayerState>,
    current_player: usize,
    pub state: SequenceOfPlayState,
    available_actions: AvailableActions,
    selected_action: Option<Action>,
    event_deck: VecDeque<Event>,
    current_event: Event,
    event_discard: VecDeque<Event>,
    board: Board,
}

impl fmt::Display for SequenceOfPlay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Player eligibilities: {:#?}\nAvailable actions: {:#?}",
            self.player_eligibilities, self.available_actions
        )
    }
}

// TODO: Track players in hashmap and vecs
// TODO: Pretty print
impl SequenceOfPlay {
    pub fn new(mut events: VecDeque<Event>, board: Board) -> Self {
        let mut player_eligibilities: HashMap<Player, PlayerState> = HashMap::new();
        player_eligibilities.insert(Civitates, Eligible);
        player_eligibilities.insert(Dux, Eligible);
        player_eligibilities.insert(Saxons, Eligible);
        player_eligibilities.insert(Scotti, Eligible);

        let curr_event: Event = events.pop_front().unwrap();
        let discard: VecDeque<Event> = VecDeque::new();

        SequenceOfPlay {
            player_eligibilities: player_eligibilities,
            current_player: 0,
            state: SequenceOfPlayState::CheckEndRound,
            available_actions: AvailableActions::new(),
            selected_action: None,
            event_deck: events,
            current_event: curr_event,
            event_discard: discard,
            board,
        }
    }

    pub fn check_end_round(mut self) -> Self {
        event!(Level::INFO, "Checking for end of round...");
        match self.state {
            SequenceOfPlayState::CheckEndRound => {
                if self.current_player > 3
                    || self.available_actions.state == AvailableActionState::End
                {
                    event!(Level::INFO, "Ending round");
                    self.state = SequenceOfPlayState::ResetEligibility;
                } else {
                    event!(Level::INFO, "Continuing round");
                    self.state = SequenceOfPlayState::CheckPlayerStatus;
                }
                return self;
            }
            _ => panic!(
                "Can only check end round in CheckEndRound, currently in {:?}",
                self.state
            ),
        }
    }

    pub fn check_player_status(mut self) -> Self {
        let current_player = self.current_event.eligibility[self.current_player];
        event!(Level::INFO, "Checking player state...");
        match self.state {
            SequenceOfPlayState::CheckPlayerStatus => {
                match self.player_eligibilities.get(&current_player).unwrap() {
                    PlayerState::Eligible => {
                        event!(Level::INFO, "{:?} is eligible", current_player);
                        self.state = SequenceOfPlayState::ChoosingAction;
                        return self;
                    }
                    PlayerState::Ineligible => {
                        event!(
                            Level::INFO,
                            "{:?} is ineligible, proceeding to next player",
                            current_player
                        );
                        self.current_player += 1;
                        self.state = SequenceOfPlayState::CheckEndRound;
                        return self;
                    }
                    _ => panic!(
                        "While checking player status found a player already at {:?}",
                        self.player_eligibilities.get(&current_player).unwrap()
                    ),
                }
            }
            _ => panic!(
                "Can only check player status in CheckPlayerStatus, currently in {:?}",
                self.state
            ),
        }
    }

    // TODO: dependency inject query to handle user input vs bot input (vs automated testing input)?
    pub fn get_action(mut self) -> Self {
        match self.state {
            SequenceOfPlayState::ChoosingAction => {
                println!("Available actions: {:?}", self.available_actions.a);
                println!(
                    "\nGetting first action from {}",
                    self.current_event.eligibility[self.current_player],
                );
                let selection: Action = self.available_actions.a[Select::new()
                    .with_prompt(format!("Select one of the following actions!"))
                    .items(&self.available_actions.a)
                    .interact()
                    .unwrap()];
                println!("Selected {}", selection);
                self.selected_action = Some(selection);
                self.state = SequenceOfPlayState::Acting;
                return self;
            }
            _ => panic!(
                "Can only get action in GettingAction state, currently in {:?}",
                self.state
            ),
        }
    }

    pub fn acting(mut self) -> Self {
        let current_player: Player = self.current_event.eligibility[self.current_player];
        match self.state {
            SequenceOfPlayState::Acting => {
                println!(
                    "{} performing action: {:?}",
                    current_player,
                    self.selected_action.unwrap()
                );
                match self.selected_action.unwrap() {
                    Action::Pass => {
                        self.player_eligibilities
                            .insert(current_player, PlayerState::Passed);
                    }
                    Action::LimitedCommand => {
                        // TODO: add a flag marking the command as being limited
                        println!(
                            "You can Command anything, so long as you're Civitates, the Command is Muster, and you only do it for one round"
                        );
                        let command: Result<fn(&mut Board), String> = get_commands(current_player);
                        match command {
                            Ok(f) => f(&mut self.board),
                            Err(e) => {
                                event!(Level::ERROR, error = e);
                                event!(Level::WARN, "For now, just marking player as Acted and continuing");
                            }
                        }
                        self.player_eligibilities
                            .insert(current_player, PlayerState::Acted);
                    }
                    Action::CommandOnly => {
                        println!(
                            "You can Command anything, so long as you're Civitates, the Command is Muster, and you only do it for one round"
                        );
                        let command: Result<fn(&mut Board), String> = get_commands(current_player);
                        match command {
                            Ok(f) => f(&mut self.board),
                            Err(e) => {
                                event!(Level::ERROR, error = e);
                                event!(Level::WARN, "For now, just marking player as Acted and continuing");
                            }
                        }
                        self.player_eligibilities
                            .insert(current_player, PlayerState::Acted);
                    }
                    Action::CommandFeat => {
                        println!(
                            "You can Command anything, so long as you're Civitates, the Command is Muster, and you only do it for one round"
                        );
                        let command: Result<fn(&mut Board), String> = get_commands(current_player);
                        match command {
                            Ok(f) => f(&mut self.board),
                            Err(e) => {
                                event!(Level::ERROR, error = e);
                                event!(Level::WARN, "For now, just marking player as Acted and continuing");
                            }
                        }
                        self.player_eligibilities
                            .insert(current_player, PlayerState::Acted);
                    }
                    _ => {
                        self.player_eligibilities
                            .insert(current_player, PlayerState::Acted);
                    }
                }
                self.state = SequenceOfPlayState::CheckEndRound;
                self.available_actions = self
                    .available_actions
                    .update_available_actions(self.selected_action);
                self.current_player += 1;
                return self;
            }
            _ => panic!(
                "Can only do action in Acting state, currently in {:?} state",
                self.state
            ),
        }
    }

    pub fn reset_eligibility(mut self) -> Self {
        event!(Level::INFO, "Reseting eligibility...");
        match self.state {
            SequenceOfPlayState::ResetEligibility => {
                let mut new_eligibility: HashMap<Player, PlayerState> = HashMap::new();
                for (elig, p) in &self.player_eligibilities {
                    match *p {
                        PlayerState::Eligible => {
                            new_eligibility.insert(*elig, PlayerState::Eligible)
                        }
                        PlayerState::Ineligible => {
                            new_eligibility.insert(*elig, PlayerState::Eligible)
                        }
                        PlayerState::Passed => new_eligibility.insert(*elig, PlayerState::Eligible),
                        PlayerState::Acted => {
                            new_eligibility.insert(*elig, PlayerState::Ineligible)
                        }
                    };
                }
                self.player_eligibilities = new_eligibility;
                self.available_actions = AvailableActions::new();
                self.state = SequenceOfPlayState::AdvanceEvents;
                event!(Level::INFO, "Eligibilities reset");
                return self;
            }
            _ => {
                panic!(
                    "Can only do cleanup in Reseting state, currently in {:?} state",
                    self.state
                );
            }
        }
    }

    pub fn advance_events(mut self) -> Self {
        event!(Level::INFO, "Advancing events...");
        match self.state {
            SequenceOfPlayState::AdvanceEvents => {
                self.event_discard.push_front(self.current_event);
                self.current_event = self.event_deck.pop_front().unwrap();
                self.current_player = 0;
                match self.event_deck[0].event_type {
                    EventType::Standard => self.state = SequenceOfPlayState::CheckPlayerStatus,
                    EventType::Epoch => {
                        self.state = SequenceOfPlayState::Epoch;
                        let epoch: Event = self.event_deck.pop_front().unwrap();
                        self.event_deck.push_front(self.current_event);
                        self.current_event = epoch;
                    }
                    EventType::Pivotal => {
                        panic!("How did a Pivotal get to be mixed into the deck???")
                    }
                }
                event!(Level::INFO, "Events advanced\n\n");
                event!(Level::INFO, state = %self);
                return self;
            }
            _ => {
                panic!(
                    "Can only advance cards in AdvanceEvents state, currently in {:?} state",
                    self.state
                );
            }
        }
    }

    pub fn epoch(mut self) -> Self {
        println!("Begin Epoch round");
        match self.state {
            SequenceOfPlayState::Epoch => {
                self.state = SequenceOfPlayState::AdvanceEvents;
                return self;
            }
            _ => panic!("Attempting to do epoch round while in {:?}", self.state),
        }
    }
}

/*fn get_actions(current_player: Player, feat: bool, limited: bool) -> Vec<fn(&mut Board)> {

}*/

/*
    1. Choose whether to do limcmd, cmd only, cmd+feat
    2. Retrieve faction commands
    3. Select command
    4. Select and pay for command spaces
    5. If feat
        1. Retrieve faction feats
        2. Select feat
        3. Select feat spaces
    6. Resolve cmd+feat in desired order in selected spaces

    Retrieving faction commands
    Input: faction
    Output: Vec<String>

    Select commands
    Input: Vec<String>
    Output: String

    Select and pay for command spaces
    Input: &mut Board, limcmd flag
    Mutate: resources, wealth, renown
    Output: Vec<String>

    Retrieve faction feats
    Input: faction, command (string)
    Output: Vec<String>

    Select feat
    Input: Vec<String>
    Output: String

    Select feat spaces
    Input: &Board
    Output: Vec<String>

    Resolve cmd+feat
    Input: &mut Board
*/

fn get_commands(current_player: Player) -> Result<fn(&mut Board), String> {
    let commands: Vec<&str>;
    match current_player {
        Player::Civitates => commands = vec!["Muster", "March", "Trade", "Battle"],
        Player::Dux => commands = vec!["Train", "March", "Intercept", "Battle"],
        Player::Saxons => commands = vec!["Raid", "Return", "March", "Battle"],
        Player::Scotti => commands = vec!["Raid", "Return", "March", "Battle"],
    }
    let commands: Vec<String> = commands
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let selected_command: String = commands[Select::new()
        .with_prompt(format!("Select one of the following Commands!"))
        .items(&commands)
        .interact()
        .unwrap()]
    .to_string();
    let action_func: Result<fn(&mut Board), String> =
        validate_command_selection(current_player, &selected_command);
    return action_func;
}

fn get_feats(current_player: Player, command: &str) -> Vec<String> {
    let feats: Vec<&str>;
    match command {
        "Muster" => feats = vec!["Rule", "Invite"],
        "March" => match current_player {
            Player::Civitates => feats = vec!["Rule", "Invite", "Pillage"],
            Player::Dux => feats = vec!["Build", "Invite", "Requisition"],
            Player::Saxons => feats = vec!["Settle"],
            Player::Scotti => feats = vec!["Settle", "Entreat"],
        },
        "Trade" => feats = vec!["Rule", "Invite"],
        "Battle" => match current_player {
            Player::Civitates => feats = vec!["Reinforce", "Pillage"],
            Player::Dux => feats = vec!["Requisition", "Retaliate"],
            Player::Saxons => feats = vec!["Surprise", "Ravage", "Shield Wall"],
            Player::Scotti => feats = vec!["Surprise", "Ransom", "Entreat"],
        },
        "Train" => feats = vec!["Build", "Invite", "Requisition"],
        "Intercept" => feats = vec!["Invite", "Retaliate"],
        "Raid" => match current_player {
            Player::Saxons => feats = vec!["Surprise", "Ravage"],
            Player::Scotti => feats = vec!["Surprise", "Ransom"],
            _ => panic!("{} do not have a Feat for {}", current_player, command),
        },
        "Return" => match current_player {
            Player::Saxons => feats = vec!["Settle"],
            Player::Scotti => feats = vec!["Settle", "Entreat"],
            _ => panic!("{} do not have a Feat for {}", current_player, command),
        },
        _ => panic!("Passed in invalid Command {}", command),
    }
    return feats.iter().map(|s| s.to_string()).collect();
}

fn validate_command_selection(
    current_player: Player,
    selected_command: &str,
) -> Result<fn(&mut Board), String> {
    match current_player {
        Player::Civitates => match selected_command {
            "Muster" => Ok(commands::muster),
            "March" => Err("Civitates March command not yet implemented".to_string()),
            "Trade" => Err("Civitates Trade command not yet implemented".to_string()),
            "Battle" => Err("Civitates Battle command not yet implemented".to_string()),
            _ => Err("Selected a Command that does not exist".to_string()),
        },
        Player::Dux => match selected_command {
            "Train" => Err("Dux Train command not yet implemented".to_string()),
            "March" => Err("Dux March command not yet implemented".to_string()),
            "Intercept" => Err("Dux Intercept command not yet implemented".to_string()),
            "Battle" => Err("Dux Battle command not yet implemented".to_string()),
            _ => Err("Selected a Command that does not exist".to_string()),
        },
        Player::Saxons => match selected_command {
            "Raid" => Err("Saxons Raid command not yet implemented".to_string()),
            "Return" => Err("Saxons Return command not yet implemented".to_string()),
            "March" => Err("Saxons March command not yet implemented".to_string()),
            "Battle" => Err("Saxons Battle command not yet implemented".to_string()),
            _ => Err("Selected a Command that does not exist".to_string()),
        },
        Player::Scotti => match selected_command {
            "Raid" => Err("Scotti Raid command not yet implemented".to_string()),
            "Return" => Err("Scotti Return command not yet implemented".to_string()),
            "March" => Err("Scotti March command not yet implemented".to_string()),
            "Battle" => Err("Scotti Battle command not yet implemented".to_string()),
            _ => Err("Selected a Command that does not exist".to_string()),
        },
    }
}

fn validate_feat_selection(
    current_player: Player,
    selected_feat: &str,
) -> Result<fn(&mut Board), &str> {
    match current_player {
        Player::Civitates => match selected_feat {
            "Rule" => Err("Civitates Rule feat not yet implemented"),
            "Invite" => Err("Civitates Invite feat not yet implemented"),
            "Reinforce" => Err("Civitates Reinforce feat not yet implemented"),
            "Pillage" => Err("Civitates Pillage feat not yet implemented"),
            _ => Err("Selected a Feat that does not exist"),
        },
        Player::Dux => match selected_feat {
            "Build" => Err("Dux Build feat not yet implemented"),
            "Invite" => Err("Dux Invite feat not yet implemented"),
            "Requisition" => Err("Dux Requisition feat not yet implemented"),
            "Retaliate" => Err("Dux Retaliate feat not yet implemented"),
            _ => Err("Selected a Feat that does not exist"),
        },
        Player::Saxons => match selected_feat {
            "Settle" => Err("Saxons Settle feat not yet implemented"),
            "Surprise" => Err("Saxons Surprise feat not yet implemented"),
            "Ravage" => Err("Saxons Ravage feat not yet implemented"),
            "Shield Wall" => Err("Saxons Shield Wall feat not yet implemented"),
            _ => Err("Selected a Feat that does not exist"),
        },
        Player::Scotti => match selected_feat {
            "Settle" => Err("Scotti Settle feat not yet implemented"),
            "Surprise" => Err("Scotti Surprise feat not yet implemented"),
            "Ransom" => Err("Scotti Ransom feat not yet implemented"),
            "Entreat" => Err("Scotti Entreat feat not yet implemented"),
            _ => Err("Selected a Feat that does not exist"),
        },
    }
}
