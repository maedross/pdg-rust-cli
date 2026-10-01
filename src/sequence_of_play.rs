use dialoguer::Select;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use tracing::{Level, event};

use super::board::{Board, Imperium, Space, StrongholdSite};
use super::commands::{self, Command, get_faction_commands};
use super::concepts::{Player, StrongholdClass};
use super::concepts::{Unit, UnitClass};
use super::events::{Event, EventType};
use super::feats::{Feat, get_faction_feats};
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
    a: Vec<SequenceOfPlayAction>,
    state: AvailableActionState,
}

impl AvailableActions {
    fn new() -> Self {
        AvailableActions {
            a: vec![
                SequenceOfPlayAction::Pass,
                SequenceOfPlayAction::CommandOnly,
                SequenceOfPlayAction::CommandFeat,
                SequenceOfPlayAction::Event,
            ],
            state: AvailableActionState::Start,
        }
    }

    fn update_available_actions(self, selection: Option<SequenceOfPlayAction>) -> AvailableActions {
        match self.state {
            AvailableActionState::Start => match selection.unwrap() {
                SequenceOfPlayAction::Pass => self,
                SequenceOfPlayAction::CommandOnly => AvailableActions {
                    a: vec![
                        SequenceOfPlayAction::Pass,
                        SequenceOfPlayAction::LimitedCommand,
                    ],
                    state: AvailableActionState::A,
                },
                SequenceOfPlayAction::CommandFeat => AvailableActions {
                    a: vec![
                        SequenceOfPlayAction::Pass,
                        SequenceOfPlayAction::Event,
                        SequenceOfPlayAction::LimitedCommand,
                    ],
                    state: AvailableActionState::B,
                },
                SequenceOfPlayAction::Event => AvailableActions {
                    a: vec![
                        SequenceOfPlayAction::Pass,
                        SequenceOfPlayAction::CommandFeat,
                    ],
                    state: AvailableActionState::C,
                },
                _ => panic!("Invalid selected action for start"),
            },
            AvailableActionState::A => match selection.unwrap() {
                SequenceOfPlayAction::Pass => self,
                SequenceOfPlayAction::LimitedCommand => AvailableActions {
                    a: vec![],
                    state: AvailableActionState::End,
                },
                _ => panic!("Invalid selected action from Command Only"),
            },
            AvailableActionState::B => match selection.unwrap() {
                SequenceOfPlayAction::Pass => self,
                SequenceOfPlayAction::Event => AvailableActions {
                    a: vec![],
                    state: AvailableActionState::End,
                },
                SequenceOfPlayAction::LimitedCommand => AvailableActions {
                    a: vec![],
                    state: AvailableActionState::End,
                },
                _ => panic!("Invalid selected action from Command + Feat"),
            },
            AvailableActionState::C => match selection.unwrap() {
                SequenceOfPlayAction::Pass => self,
                SequenceOfPlayAction::CommandFeat => AvailableActions {
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
enum SequenceOfPlayAction {
    Pass,
    CommandOnly,
    LimitedCommand,
    CommandFeat,
    Event,
}

impl fmt::Display for SequenceOfPlayAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SequenceOfPlayAction::Pass => write!(f, "Pass"),
            SequenceOfPlayAction::CommandOnly => write!(f, "CommandOnly"),
            SequenceOfPlayAction::LimitedCommand => write!(f, "LimitedCommand"),
            SequenceOfPlayAction::CommandFeat => write!(f, "CommandFeat"),
            SequenceOfPlayAction::Event => write!(f, "Event"),
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
    ChoosingSequenceOfPlayAction,
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
    selected_action: Option<SequenceOfPlayAction>,
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
                        self.state = SequenceOfPlayState::ChoosingSequenceOfPlayAction;
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
            SequenceOfPlayState::ChoosingSequenceOfPlayAction => {
                println!("Available actions: {:?}", self.available_actions.a);
                println!(
                    "\nGetting first action from {}",
                    self.current_event.eligibility[self.current_player],
                );
                let selection: SequenceOfPlayAction = self.available_actions.a[Select::new()
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
                    SequenceOfPlayAction::Pass => {
                        self.player_eligibilities
                            .insert(current_player, PlayerState::Passed);
                    }
                    SequenceOfPlayAction::LimitedCommand => {
                        let command = get_command_only_selection(current_player);
                        println!("Selected a limited {}", command);
                        self.player_eligibilities
                            .insert(current_player, PlayerState::Acted);
                    }
                    SequenceOfPlayAction::CommandOnly => {
                        let command = get_command_only_selection(current_player);
                        println!("Selected {} only", command);
                        self.player_eligibilities
                            .insert(current_player, PlayerState::Acted);
                    }
                    SequenceOfPlayAction::CommandFeat => {
                        let (command, feat) = get_command_plus_feat_selection(current_player, self.board.imperium);
                        println!("Selected {} + {}", command, feat);
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

fn get_command_only_selection(current_player: Player) -> Command {
    let commands: Vec<Command> = get_faction_commands(current_player, None);
    let selected_command: usize = Select::new()
        .with_prompt(format!("Select one of the following Commands!"))
        .items(&commands)
        .interact()
        .unwrap();
    return commands[selected_command].clone();
}

fn get_command_plus_feat_selection(current_player: Player, imperium: Imperium) -> (Command, Feat) {
    let selected_command: Command;
    let selected_feat: Feat;

    let commands: Vec<Command> = get_faction_commands(current_player, None);
    let feats: Vec<Feat> = get_faction_feats(current_player, None, imperium);
    let mut initial_options: Vec<String> = commands
        .clone()
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();
    initial_options.append(
        &mut feats
            .clone()
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>(),
    );
    let first_action: usize = Select::new()
        .with_prompt("Select one of the following Commands or Feats!")
        .items(&initial_options)
        .interact()
        .unwrap();
    if first_action < commands.len() {
        selected_command = commands[first_action];
        let feats: Vec<Feat> = get_faction_feats(current_player, Some(selected_command), imperium);
        let second_action = Select::new()
            .with_prompt(format!(
                "Select one of the following Feats to accompany {}!",
                selected_command
            ))
            .items(&feats)
            .interact()
            .unwrap();
        selected_feat = feats[second_action];
    } else {
        selected_feat = feats[first_action - commands.len()];
        let commands = get_faction_commands(current_player, Some(selected_feat));
        let second_action = Select::new()
            .with_prompt(format!(
                "Select one of the following Commands to accompany {}!",
                selected_feat
            ))
            .items(&commands)
            .interact()
            .unwrap();
        selected_command = commands[second_action];
    }
    return (selected_command, selected_feat);
}

fn get_event_selection() {}
