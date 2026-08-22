use dialoguer::Select;
use std::collections::{HashMap, VecDeque};
use std::fmt;

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
        println!("Checking for end of round...");
        match self.state {
            SequenceOfPlayState::CheckEndRound => {
                if self.current_player > 3
                    || self.available_actions.state == AvailableActionState::End
                {
                    println!("Ending round");
                    self.state = SequenceOfPlayState::ResetEligibility;
                } else {
                    println!("Continuing round");
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
        println!("Checking player state...");
        match self.state {
            SequenceOfPlayState::CheckPlayerStatus => {
                match self
                    .player_eligibilities
                    .get(&self.current_event.eligibility[self.current_player])
                    .unwrap()
                {
                    PlayerState::Eligible => {
                        println!(
                            "{} is eligible",
                            self.current_event.eligibility[self.current_player]
                        );
                        self.state = SequenceOfPlayState::ChoosingAction;
                        return self;
                    }
                    PlayerState::Ineligible => {
                        println!(
                            "{} is ineligible, proceeding to next player",
                            self.current_event.eligibility[self.current_player]
                        );
                        self.current_player += 1;
                        self.state = SequenceOfPlayState::CheckEndRound;
                        return self;
                    }
                    _ => panic!(
                        "While checking player status found a player already at {:?}",
                        self.player_eligibilities
                            .get(&self.current_event.eligibility[self.current_player])
                            .unwrap()
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
        match self.state {
            SequenceOfPlayState::Acting => {
                println!(
                    "{} performing action: {:?}",
                    self.current_event.eligibility[self.current_player],
                    self.selected_action.unwrap()
                );
                match self.selected_action.unwrap() {
                    Action::Pass => {
                        self.player_eligibilities.insert(
                            self.current_event.eligibility[self.current_player],
                            PlayerState::Passed,
                        );
                    }
                    Action::CommandOnly => {
                        println!(
                            "You can Command anything, so long as you're Civitates, the Command is Muster, and you only do it for one round"
                        );
                        let command_options: Vec<String> =
                            get_commands(self.current_event.eligibility[self.current_player]);
                        let selected_command: String = command_options[Select::new()
                            .with_prompt(format!("Select one of the following Commands!"))
                            .items(&command_options)
                            .interact()
                            .unwrap()]
                        .clone();
                        let action_func = validate_command_selection(
                            self.current_event.eligibility[self.current_player],
                            &selected_command,
                        );
                        match action_func {
                            Ok(f) => f(&mut self.board),
                            Err(e) => {
                                println!("{}", e);
                                println!("For now, just marking player as Acted and continuing");
                            }
                        }
                        self.player_eligibilities.insert(
                            self.current_event.eligibility[self.current_player],
                            PlayerState::Acted,
                        );
                    }
                    _ => {
                        self.player_eligibilities.insert(
                            self.current_event.eligibility[self.current_player],
                            PlayerState::Acted,
                        );
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
        println!("Reseting eligibility...");
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
                println!("Eligibilities reset");
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
        println!("Advancing events...");
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
                println!("Events advanced\n\n");
                println!("{}", self);
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

fn get_commands(current_player: Player) -> Vec<String> {
    let commands;
    match current_player {
        Player::Civitates => {
            commands = vec!["Muster", "March", "Trade", "Battle"];
        }
        Player::Dux => {
            commands = vec!["Train", "March", "Intercept", "Battle"];
        }
        Player::Saxons => {
            commands = vec!["Raid", "Return", "March", "Battle"];
        }
        Player::Scotti => {
            commands = vec!["Raid", "Return", "March", "Battle"];
        }
    }
    return commands.iter().map(|s| s.to_string()).collect();
}

fn validate_command_selection(
    current_player: Player,
    selected_command: &str,
) -> Result<fn(&mut Board), &str> {
    match current_player {
        Player::Civitates => match selected_command {
            "Muster" => Ok(commands::muster),
            "March" => Err("Civitates March command not yet implemented"),
            "Trade" => Err("Civitates Trade command not yet implemented"),
            "Battle" => Err("Civitates Battle command not yet implemented"),
            _ => Err("Selected a Command that does not exist"),
        },
        Player::Dux => match selected_command {
            "Train" => Err("Dux Train command not yet implemented"),
            "March" => Err("Dux March command not yet implemented"),
            "Intercept" => Err("Dux Intercept command not yet implemented"),
            "Battle" => Err("Dux Battle command not yet implemented"),
            _ => Err("Selected a Command that does not exist"),
        },
        Player::Saxons => match selected_command {
            "Raid" => Err("Saxons Raid command not yet implemented"),
            "Return" => Err("Saxons Return command not yet implemented"),
            "March" => Err("Saxons March command not yet implemented"),
            "Battle" => Err("Saxons Battle command not yet implemented"),
            _ => Err("Selected a Command that does not exist"),
        },
        Player::Scotti => match selected_command {
            "Raid" => Err("Scotti Raid command not yet implemented"),
            "Return" => Err("Scotti Return command not yet implemented"),
            "March" => Err("Scotti March command not yet implemented"),
            "Battle" => Err("Scotti Battle command not yet implemented"),
            _ => Err("Selected a Command that does not exist"),
        },
    }
}
