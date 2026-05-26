use colored::Colorize;
use dialoguer::Select;
use std::collections::HashSet;
use std::fmt;
use std::vec;

use crate::Player::Civitates;
use crate::Player::Dux;
use crate::Player::Saxons;
use crate::Player::Scotti;
use crate::PlayerState::Eligible;
// State

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Player {
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
            Player::Civitates => write!(f, "{}", "Civitates".blue()),
            Player::Dux => write!(f, "{}", "Dux".red()),
            Player::Saxons => write!(f, "{}", "Saxons".black()),
            Player::Scotti => write!(f, "{}", "Scotti".green()),
        }
    }
}


// Cards
struct Event {
    eligibility: Vec<Player>,
    unshaded: Option<u8>,
    shaded: Option<u8>,
    historical_notes: String,
}

struct CardManager<'a> {
    deck: &'a mut Vec<Event>,
    current_event: &'a Event,
    upcoming_event: &'a Event,
    discard: &'a mut Vec<Event>,
}

// Sequence of Play

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum SequenceOfPlayState {
    GettingFirstAction,
    FirstAction,
    GettingSecondAction,
    SecondAction,
    Passing,
    EndOfRound,
}

#[derive(Clone, Copy, Debug)]
enum AvailableActionState {
    Pass,
    CommandOnly,
    LimitedCommand,
    CommandFeat,
    Event,
}

impl fmt::Display for AvailableActionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AvailableActionState::Pass => write!(f, "Pass"),
            AvailableActionState::CommandOnly => write!(f, "CommandOnly"),
            AvailableActionState::LimitedCommand => write!(f, "LimitedCommand"),
            AvailableActionState::CommandFeat => write!(f, "CommandFeat"),
            AvailableActionState::Event => write!(f, "Event"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum PlayerState {
    Eligible,
    Passed,
    Acted,
    Ineligible,
}

#[derive(Clone, Copy, Debug)]
struct PlayerEligibility {
    faction: Player,
    state: PlayerState,
}

#[derive(Clone)]
struct SequenceOfPlay {
    player_eligibilities: Vec<PlayerEligibility>,
    state: SequenceOfPlayState,
    available_actions: Vec<AvailableActionState>,
    current_player: Option<Player>,
}

impl fmt::Display for SequenceOfPlay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Current state: {:?}\nPlayer eligibilities: {:#?}\nAvailable actions: {:#?}",
            self.state, self.player_eligibilities, self.available_actions
        )
    }
}

impl SequenceOfPlay {
    fn new() -> Self {
        SequenceOfPlay {
            player_eligibilities: vec![
                PlayerEligibility {
                    faction: Civitates,
                    state: Eligible,
                },
                PlayerEligibility {
                    faction: Dux,
                    state: Eligible,
                },
                PlayerEligibility {
                    faction: Saxons,
                    state: Eligible,
                },
                PlayerEligibility {
                    faction: Scotti,
                    state: Eligible,
                },
            ],
            state: SequenceOfPlayState::GettingFirstAction,
            available_actions: vec![
                AvailableActionState::Pass,
                AvailableActionState::CommandOnly,
                AvailableActionState::CommandFeat,
                AvailableActionState::Event,
            ],
            current_player: None,
        }
    }

    fn get_first_action(mut self) -> Result<Self, &'static str> {
        match self.state {
            SequenceOfPlayState::GettingFirstAction => {
                println!("Getting first action");
                let selection: usize = Select::new()
                    .with_prompt(format!("Select one of the following actions!"))
                    .items(&self.available_actions)
                    .interact()
                    .unwrap();
                println!("Selected {}", self.available_actions[selection]);
                match self.available_actions[selection] {
                    AvailableActionState::Pass => return Ok(self),
                    AvailableActionState::CommandOnly => {
                        self.available_actions = vec![
                            AvailableActionState::Pass,
                            AvailableActionState::LimitedCommand,
                        ]
                    }
                    AvailableActionState::CommandFeat => {
                        self.available_actions = vec![
                            AvailableActionState::Pass,
                            AvailableActionState::LimitedCommand,
                            AvailableActionState::Event,
                        ]
                    }
                    AvailableActionState::Event => {
                        self.available_actions = vec![
                            AvailableActionState::Pass,
                            AvailableActionState::CommandFeat,
                        ]
                    }
                    _ => return Err("Selected invalid action"),
                }
                self.state = SequenceOfPlayState::FirstAction;
                return Ok(self);
            }
            _ => Err("Can only get first action in GettingFirstAction state"),
        }
    }

    fn first_action(mut self) -> Result<Self, &'static str> {
        match self.state {
            SequenceOfPlayState::FirstAction => {
                println!("Performing first action: {:?}", self.state);
                self.state = SequenceOfPlayState::GettingSecondAction;
                return Ok(self);
            }
            _ => Err("Can only do first action in FirstAction state"),
        }
    }

    fn get_second_action(mut self) -> Result<Self, &'static str> {
        match self.state {
            SequenceOfPlayState::GettingSecondAction => {
                println!("Getting second action");
                let selection: usize = Select::new()
                    .with_prompt(format!("Select one of the following actions!"))
                    .items(&self.available_actions)
                    .interact()
                    .unwrap();
                println!("Selected {}", self.available_actions[selection]);
                match self.available_actions[selection] {
                    AvailableActionState::Pass => return Ok(self),
                    AvailableActionState::LimitedCommand => {},
                    AvailableActionState::Event => {},
                    AvailableActionState::CommandFeat => {},
                    _ => return Err("Selected invalid action"),
                }
                self.available_actions = vec![
                    AvailableActionState::Pass,
                    AvailableActionState::CommandOnly,
                    AvailableActionState::CommandFeat,
                    AvailableActionState::Event,
                ];
                self.state = SequenceOfPlayState::SecondAction;
                return Ok(self);
            }
            _ => Err("Can only get second action in GettingSecondAction state"),
        }
    }

    fn second_action(mut self) -> Result<Self, &'static str> {
        match self.state {
            SequenceOfPlayState::SecondAction => {
                println!("Performing second action");
                self.state = SequenceOfPlayState::EndOfRound;
                return Ok(self);
            }
            _ => Err("Can only do second action in SecondAction state"),
        }
    }

    fn cleanup(mut self) -> Result<Self, &'static str> {
        match self.state {
            SequenceOfPlayState::EndOfRound => {
                println!("Reseting for the next round");
                self.state = SequenceOfPlayState::GettingFirstAction;
                return Ok(self);
            }
            _ => Err("Can only do cleanup in EndOfRound state"),
        }
    }
}


fn main() {
    /*
       1. Build game
           1. Create edge track with initial values
           2. Build map
           3. Create unit types?
           4. Put stuff on map
           5. Generate deck
       2. Begin sequence of play
       3. Loop
    */
    println!(
        "{} {} {} {}",
        Player::Civitates,
        Player::Dux,
        Player::Saxons,
        Player::Scotti
    );

    let sop: SequenceOfPlay = SequenceOfPlay::new();

    loop {
        match sop.state.clone() {
            SequenceOfPlayState::Passing => {},
            SequenceOfPlayState::GettingFirstAction => {
                let sop: SequenceOfPlay = sop.clone().get_first_action().unwrap();
            },
            SequenceOfPlayState::FirstAction => {
                let sop: SequenceOfPlay = sop.clone().first_action().unwrap();
            },
            SequenceOfPlayState::GettingSecondAction => {
                let sop: SequenceOfPlay = sop.clone().get_second_action().unwrap();
            },
            SequenceOfPlayState::SecondAction => {
                let sop: SequenceOfPlay = sop.clone().second_action().unwrap();
            },
            SequenceOfPlayState::EndOfRound => {
                let sop: SequenceOfPlay = sop.clone().cleanup().unwrap();
            },
        }
    }
    
    let sop: SequenceOfPlay = sop.get_first_action().unwrap();
    let sop: SequenceOfPlay = sop.first_action().unwrap();
    let sop: SequenceOfPlay = sop.get_second_action().unwrap();
    let sop: SequenceOfPlay = sop.second_action().unwrap();
    let sop: SequenceOfPlay = sop.cleanup().unwrap();

    let mut test_card_0: Event = Event {
        eligibility: vec![
            Player::Civitates,
            Player::Dux,
            Player::Saxons,
            Player::Scotti,
        ],
        unshaded: None,
        shaded: None,
        historical_notes: String::from(""),
    };
    let mut test_card_1: Event = Event {
        eligibility: vec![
            Player::Scotti,
            Player::Saxons,
            Player::Dux,
            Player::Civitates,
        ],
        unshaded: None,
        shaded: None,
        historical_notes: String::from(""),
    };
    let mut test_card_view: CardManager = CardManager {
        deck: &mut vec![],
        current_event: &test_card_0,
        upcoming_event: &test_card_1,
        discard: &mut vec![],
    };
}
