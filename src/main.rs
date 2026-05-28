use colored::Colorize;
use dialoguer::Select;
use std::collections::HashMap;
use std::fmt;

use crate::Player::Civitates;
use crate::Player::Dux;
use crate::Player::Saxons;
use crate::Player::Scotti;
use crate::PlayerState::Acted;
use crate::PlayerState::Eligible;
use crate::PlayerState::Ineligible;
use crate::PlayerState::Passed;
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
#[derive(Clone, Debug)]
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

// TODO: State history?
#[derive(Clone)]
struct SequenceOfPlay {
    player_eligibilities: HashMap<Player, PlayerState>,
    state: SequenceOfPlayState,
    available_actions: Vec<AvailableActionState>,
    current_player: Option<Player>,
    old_state: Option<SequenceOfPlayState>,
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

/*
    TODO: Implement player state checking and setting
    When a player selected a non-pass action, they should be set to acting
    When getting action, should be checking player is eligible
    Cleanup should move ineligible and passed players to eligible, acted to ineligible

    Give cards names and print them, round number
*/
impl SequenceOfPlay {
    fn new() -> Self {
        let mut player_eligibilities: HashMap<Player, PlayerState> = HashMap::new();
        player_eligibilities.insert(Civitates, Eligible);
        player_eligibilities.insert(Dux, Eligible);
        player_eligibilities.insert(Saxons, Eligible);
        player_eligibilities.insert(Scotti, Eligible);
        SequenceOfPlay {
            player_eligibilities: player_eligibilities,
            state: SequenceOfPlayState::GettingFirstAction,
            available_actions: vec![
                AvailableActionState::Pass,
                AvailableActionState::CommandOnly,
                AvailableActionState::CommandFeat,
                AvailableActionState::Event,
            ],
            current_player: None,
            old_state: None,
        }
    }

    fn pass(mut self) -> Result<Self, &'static str> {
        match self.state {
            SequenceOfPlayState::Passing => {
                println!("{} chose to pass", self.current_player.unwrap());
                match self.old_state {
                    Some(s) => {
                        self.state = s;
                        print!(
                            "{} has been set from {:?} to ",
                            self.current_player.unwrap(),
                            self.player_eligibilities
                                .get(&self.current_player.unwrap())
                                .unwrap()
                        );
                        self.player_eligibilities
                            .insert(self.current_player.unwrap(), Passed);
                        print!(
                            "{:?}\n\n",
                            self.player_eligibilities
                                .get(&self.current_player.unwrap())
                                .unwrap()
                        );
                    }
                    None => panic!("Passed when there was no old state stored!"),
                }

                return Ok(self);
            }
            _ => Err("Can only do pass in Passing state"),
        }
    }

    fn get_first_action(mut self) -> Result<Self, &'static str> {
        match self.state {
            SequenceOfPlayState::GettingFirstAction => {
                println!("\n\nGetting first action from {}", self.current_player.unwrap());
                let selection: usize = Select::new()
                    .with_prompt(format!("Select one of the following actions!"))
                    .items(&self.available_actions)
                    .interact()
                    .unwrap();
                println!("Selected {}", self.available_actions[selection]);
                match self.available_actions[selection] {
                    AvailableActionState::Pass => {
                        self.old_state = Some(self.state);
                        self.state = SequenceOfPlayState::Passing;
                        return Ok(self);
                    }
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
                println!(
                    "{} performing first action: {:?}",
                    self.current_player.unwrap(),
                    self.state
                );
                self.state = SequenceOfPlayState::GettingSecondAction;
                return Ok(self);
            }
            _ => Err("Can only do first action in FirstAction state"),
        }
    }

    fn get_second_action(mut self) -> Result<Self, &'static str> {
        match self.state {
            SequenceOfPlayState::GettingSecondAction => {
                println!(
                    "\nGetting second action from {}",
                    self.current_player.unwrap()
                );
                let selection: usize = Select::new()
                    .with_prompt(format!("Select one of the following actions!"))
                    .items(&self.available_actions)
                    .interact()
                    .unwrap();
                println!("Selected {}", self.available_actions[selection]);
                match self.available_actions[selection] {
                    AvailableActionState::Pass => {
                        self.old_state = Some(self.state);
                        self.state = SequenceOfPlayState::Passing;
                        return Ok(self);
                    }
                    AvailableActionState::LimitedCommand => {}
                    AvailableActionState::Event => {}
                    AvailableActionState::CommandFeat => {}
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
                println!("{} performing second action", self.current_player.unwrap());
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

    let mut sop: SequenceOfPlay = SequenceOfPlay::new();

    // TODO: Should not be advancing player every state transition, only after actions taken
    // Iterable instead of vec?
    // Also worth considering if event deck will be part of SequenceOfPlay

    let test_card_0: Event = Event {
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
    let test_card_1: Event = Event {
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

    let mut cards: std::vec::IntoIter<Event> = vec![test_card_0, test_card_1].into_iter();
    let mut curr_card: Option<Event> = cards.next();
    loop {
        match curr_card {
            Some(ref e) => {
                let mut curr_eligibility_order: std::vec::IntoIter<Player> =
                    e.eligibility.clone().into_iter();
                let mut curr_player: Option<Player> = curr_eligibility_order.next();
                loop {
                    match curr_player {
                        Some(p) => {
                            match sop.player_eligibilities.get(&p).unwrap() {
                                Eligible => sop.current_player = Some(p),
                                Ineligible => {
                                    println!("{} is ineligible. Continuing...", p);
                                    continue;
                                }
                                Passed => panic!("{} is passed, which is invalid", p),
                                Acted => panic!("{} has acted, which is invalid", p),
                            }
                            sop.current_player = Some(p);
                            match sop.state {
                                SequenceOfPlayState::Passing => {
                                    sop = sop.pass().unwrap();
                                    curr_player = curr_eligibility_order.next();
                                }
                                SequenceOfPlayState::GettingFirstAction => {
                                    sop = sop.get_first_action().unwrap();
                                }
                                SequenceOfPlayState::FirstAction => {
                                    sop = sop.first_action().unwrap();
                                    curr_player = curr_eligibility_order.next();
                                }
                                SequenceOfPlayState::GettingSecondAction => {
                                    sop = sop.get_second_action().unwrap();
                                }
                                SequenceOfPlayState::SecondAction => {
                                    sop = sop.second_action().unwrap();
                                    curr_player = curr_eligibility_order.next();
                                }
                                SequenceOfPlayState::EndOfRound => {
                                    sop = sop.cleanup().unwrap();
                                }
                            }
                        }
                        None => {
                            println!("Game end");
                            break;
                        }
                    }
                }
                curr_card = cards.next();
            }
            None => break,
        }
    }
}
