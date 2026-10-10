use dialoguer::Select;
use std::collections::{HashMap, VecDeque};
use std::fmt::{self};
use tracing::{Level, event};

use crate::commands::issue_command;

use super::board::{Board, Imperium};
use super::commands::{Command, get_faction_commands};
use super::concepts::Player;
use super::events::{Event, EventType};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ActionSelectionState {
    FirstSeeking,
    TookNeither,
    TookFeat,
    TookEvent,
    End,
}

#[derive(Clone, Debug)]
pub enum Action {
    Command {
        command_name: Command,
        space: Option<String>,
        action: fn(&mut Board),
    },
    Feat {
        name: &'static str,
        action: fn(&mut Board),
    },
    Pass {
        action: fn(Player, &mut Board),
    },
    Event {
        name: &'static str,
    },
    Done,
}
impl fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Command { command_name, space, action: _ } => {
                match space {
                    Some(s) => write!(f, "{}", s),
                    None => write!(f, "{}", command_name)
                }
            },
            Action::Feat { name, action: _ } => write!(f, "{}", name),
            Action::Pass { action: _ } => write!(f, "Pass"),
            Action::Event { name } => write!(f, "{}", name),
            Action::Done => write!(f, "Done"),
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
    ResetEligibility,
    AdvanceEvents,
    Epoch,
}

#[derive(Clone)]
pub struct SequenceOfPlay {
    player_eligibilities: HashMap<Player, PlayerState>,
    current_player: usize,
    pub state: SequenceOfPlayState,
    action_selection_state: ActionSelectionState,
    event_deck: VecDeque<Event>,
    current_event: Event,
    event_discard: VecDeque<Event>,
    board: Board,
}

impl fmt::Display for SequenceOfPlay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player eligibilities: {:#?}", self.player_eligibilities)
    }
}

impl SequenceOfPlay {
    pub fn new(mut events: VecDeque<Event>, board: Board) -> Self {
        let mut player_eligibilities: HashMap<Player, PlayerState> = HashMap::new();
        player_eligibilities.insert(Player::Civitates, PlayerState::Eligible);
        player_eligibilities.insert(Player::Dux, PlayerState::Eligible);
        player_eligibilities.insert(Player::Saxons, PlayerState::Eligible);
        player_eligibilities.insert(Player::Scotti, PlayerState::Eligible);

        let curr_event: Event = events.pop_front().unwrap();
        let discard: VecDeque<Event> = VecDeque::new();

        SequenceOfPlay {
            player_eligibilities: player_eligibilities,
            current_player: 0,
            state: SequenceOfPlayState::CheckEndRound,
            action_selection_state: ActionSelectionState::FirstSeeking,
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
                    || self.action_selection_state == ActionSelectionState::End
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
    // TODO: Implement event actions
    pub fn get_action(mut self) -> Self {
        let player: Player = self.current_event.eligibility[self.current_player];
        match self.state {
            SequenceOfPlayState::ChoosingSequenceOfPlayAction => {
                match self.action_selection_state {
                    ActionSelectionState::FirstSeeking => {
                        println!("\nGetting first action from {}", player,)
                    }
                    ActionSelectionState::End => {
                        panic!("Shouldn't be in get_action when action_selection_state is End")
                    }
                    _ => println!("\nGetting second action from {}", player,),
                }
                let mut avaiable_actions: Vec<Action> = vec![PASS];
                avaiable_actions.append(&mut get_faction_commands(player));
                match self.action_selection_state {
                    ActionSelectionState::FirstSeeking => {
                        if self.current_event.unshaded != None {
                            avaiable_actions.push(Action::Event {
                                name: "Unshaded event",
                            });
                        }
                        if self.current_event.shaded != None {
                            avaiable_actions.push(Action::Event {
                                name: "Shaded event",
                            });
                        }
                    }
                    ActionSelectionState::TookFeat => {
                        if self.current_event.unshaded != None {
                            avaiable_actions.push(Action::Event {
                                name: "Unshaded event",
                            });
                        }
                        if self.current_event.shaded != None {
                            avaiable_actions.push(Action::Event {
                                name: "Shaded event",
                            });
                        }
                    }
                    _ => {}
                };
                let selection: Action = avaiable_actions[Select::new()
                    .with_prompt(format!("Select one of the following actions!"))
                    .items(&avaiable_actions)
                    .interact()
                    .unwrap()]
                .clone();
                println!("Selected {}", selection);

                match selection {
                    Action::Command { command_name: _, space: _, action: _ } => match self.action_selection_state {
                        ActionSelectionState::FirstSeeking => {
                            let command_took_feat: bool = issue_command(selection, &mut self.board, false, true);
                            if command_took_feat {
                                self.action_selection_state = ActionSelectionState::TookFeat;
                            } else {
                                self.action_selection_state = ActionSelectionState::TookNeither;
                            }
                            self.player_eligibilities.insert(player, PlayerState::Acted);
                        }
                        ActionSelectionState::TookNeither => {
                            let _ = issue_command(selection, &mut self.board, true, false);
                            self.action_selection_state = ActionSelectionState::End;
                            self.player_eligibilities.insert(player, PlayerState::Acted);
                        }
                        ActionSelectionState::TookFeat => {
                            let _ = issue_command(selection, &mut self.board, true, false);
                            self.action_selection_state = ActionSelectionState::End;
                            self.player_eligibilities.insert(player, PlayerState::Acted);
                        }
                        ActionSelectionState::TookEvent => {
                            let _ = issue_command(selection, &mut self.board, false, true);
                            self.action_selection_state = ActionSelectionState::End;
                            self.player_eligibilities.insert(player, PlayerState::Acted);
                        }
                        ActionSelectionState::End => panic!("Cannot be acting in End state"),
                    },
                    Action::Event { name: _ } => {
                        println!("EVENTS NOT YET IMPLEMENTED");
                        match self.action_selection_state {
                            ActionSelectionState::TookFeat => {
                                self.action_selection_state = ActionSelectionState::End
                            }
                            ActionSelectionState::FirstSeeking => {
                                self.action_selection_state = ActionSelectionState::TookEvent
                            }
                            _ => panic!("Took event from state {:?}", self.action_selection_state),
                        }
                        self.player_eligibilities.insert(player, PlayerState::Acted);
                    }
                    Action::Pass { action } => {
                        (action)(player, &mut self.board);
                        self.player_eligibilities
                            .insert(player, PlayerState::Passed);
                    }
                    _ => panic!(
                        "Somehow chose something other than a Command, Event, or Pass: {}",
                        selection
                    ),
                }
                self.current_player += 1;
                self.state = SequenceOfPlayState::CheckEndRound;
                return self;
            }
            _ => panic!(
                "Can only get action in GettingAction state, currently in {:?}",
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
                self.action_selection_state = ActionSelectionState::FirstSeeking;
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

fn pass(player: Player, board: &mut Board) {
    match player {
        Player::Civitates => board.edge_track.briton_resources += 3,
        Player::Dux => match board.imperium {
            Imperium::Fragmentation => board.edge_track.dux_resources += 3,
            _ => board.edge_track.briton_resources += 3,
        },
        Player::Saxons => board.edge_track.saxon_renown += 1,
        Player::Scotti => board.edge_track.scotti_renown += 1,
    }
}

const PASS: Action = Action::Pass { action: pass };
