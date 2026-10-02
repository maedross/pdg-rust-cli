use dialoguer::Select;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use tracing::{Level, event};

use crate::commands::execute_command;

use super::board::{Board, Imperium};
use super::commands::{Command, get_faction_commands};
use super::concepts::Player;
use super::events::{Event, EventType};
use Player::{Civitates, Dux, Saxons, Scotti};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ActionSelectionState {
    FirstSeeking,
    TookNeither,
    TookFeat,
    TookEvent,
    End,
}

#[derive(Clone, Copy, Debug)]
enum SequenceOfPlayAction {
    Pass,
    Command(Command),
    Event(u8),
}

impl fmt::Display for SequenceOfPlayAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SequenceOfPlayAction::Pass => write!(f, "Pass"),
            SequenceOfPlayAction::Command(c) => write!(f, "Command {}", c),
            SequenceOfPlayAction::Event(e) => write!(f, "Event {:?}", e),
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
    action_selection_state: ActionSelectionState,
    selected_action: Option<SequenceOfPlayAction>,
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
        player_eligibilities.insert(Civitates, PlayerState::Eligible);
        player_eligibilities.insert(Dux, PlayerState::Eligible);
        player_eligibilities.insert(Saxons, PlayerState::Eligible);
        player_eligibilities.insert(Scotti, PlayerState::Eligible);

        let curr_event: Event = events.pop_front().unwrap();
        let discard: VecDeque<Event> = VecDeque::new();

        SequenceOfPlay {
            player_eligibilities: player_eligibilities,
            current_player: 0,
            state: SequenceOfPlayState::CheckEndRound,
            action_selection_state: ActionSelectionState::FirstSeeking,
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
    pub fn get_action(mut self) -> Self {
        match self.state {
            SequenceOfPlayState::ChoosingSequenceOfPlayAction => {
                let player: Player = self.current_event.eligibility[self.current_player];
                match self.action_selection_state {
                    ActionSelectionState::FirstSeeking => {
                        println!("\nGetting first action from {}", player,)
                    }
                    ActionSelectionState::End => {
                        panic!("Shouldn't be in get_action when action_selection_state is End")
                    }
                    _ => println!("\nGetting second action from {}", player,),
                }
                let mut avaiable_actions: Vec<SequenceOfPlayAction> =
                    vec![SequenceOfPlayAction::Pass];
                avaiable_actions.append(
                    &mut get_faction_commands(player)
                        .iter()
                        .map(|c| SequenceOfPlayAction::Command(*c))
                        .collect::<Vec<SequenceOfPlayAction>>(),
                );
                match self.action_selection_state {
                    ActionSelectionState::FirstSeeking => {
                        if self.current_event.unshaded != None {
                            avaiable_actions.push(SequenceOfPlayAction::Event(
                                self.current_event.unshaded.unwrap(),
                            ));
                        }
                        if self.current_event.shaded != None {
                            avaiable_actions.push(SequenceOfPlayAction::Event(
                                self.current_event.shaded.unwrap(),
                            ));
                        }
                    }
                    ActionSelectionState::TookFeat => {
                        if self.current_event.unshaded != None {
                            avaiable_actions.push(SequenceOfPlayAction::Event(
                                self.current_event.unshaded.unwrap(),
                            ));
                        }
                        if self.current_event.shaded != None {
                            avaiable_actions.push(SequenceOfPlayAction::Event(
                                self.current_event.shaded.unwrap(),
                            ));
                        }
                    }
                    _ => {}
                };
                let selection = avaiable_actions[Select::new()
                    .with_prompt(format!("Select one of the following actions!"))
                    .items(&avaiable_actions)
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

    // TODO: This should execute the Pass, Command (possibly limited) (possibly with Feat), or Event
    // TODO: Be sure to adjust action state
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
                        match (current_player, self.board.imperium) {
                            (Player::Civitates, _) => self.board.edge_track.briton_resources += 3,
                            (Player::Dux, Imperium::Fragmentation) => {
                                self.board.edge_track.dux_resources += 3
                            }
                            (Player::Dux, _) => self.board.edge_track.briton_resources += 3,
                            (Player::Saxons, _) => self.board.edge_track.saxon_renown += 1,
                            (Player::Scotti, _) => self.board.edge_track.scotti_renown += 1,
                        }
                        self.player_eligibilities
                            .insert(current_player, PlayerState::Passed);
                    }
                    SequenceOfPlayAction::Command(command) => match self.action_selection_state {
                        ActionSelectionState::FirstSeeking => {
                            if execute_command(
                                command,
                                current_player,
                                false,
                                true,
                                &mut self.board,
                            ) {
                                self.action_selection_state = ActionSelectionState::TookFeat;
                            } else {
                                self.action_selection_state = ActionSelectionState::TookNeither;
                            }
                        },
                        ActionSelectionState::TookEvent => {
                            execute_command(command, current_player, false, true, &mut self.board);
                            self.action_selection_state = ActionSelectionState::End;
                        },
                        ActionSelectionState::TookNeither => {
                            execute_command(command, current_player, true, false, &mut self.board);
                            self.action_selection_state = ActionSelectionState::End;
                        },
                        ActionSelectionState::TookFeat => {
                            execute_command(command, current_player, true, false, &mut self.board);
                            self.action_selection_state = ActionSelectionState::End;
                        },
                        ActionSelectionState::End => {
                            panic!("Really shouldn't be in Acting when ActionSelectionState is End")
                        },
                    },
                    SequenceOfPlayAction::Event(e) => {
                        todo!();
                        self.action_selection_state = ActionSelectionState::TookEvent;
                    }
                };
                self.player_eligibilities
                    .insert(current_player, PlayerState::Acted);
                self.state = SequenceOfPlayState::CheckEndRound;
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
