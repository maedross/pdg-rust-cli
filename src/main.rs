use colored::Colorize;
use dialoguer::Select;
use std::collections::HashMap;
use std::fmt;

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
#[derive(Clone, Debug)]
struct Event {
    eligibility: Vec<Player>,
    unshaded: Option<u8>,
    shaded: Option<u8>,
    historical_notes: String,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.eligibility)
    }
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
    GettingAction,
    Acting,
    Reseting,
}

#[derive(Clone, Copy, Debug)]
enum AvailableActionState {
    Start,
    A,
    B,
    C,
    None,
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
                    state: AvailableActionState::None,
                },
                _ => panic!("Invalid selected action from Command Only"),
            },
            AvailableActionState::B => match selection.unwrap() {
                Action::Pass => self,
                Action::Event => AvailableActions {
                    a: vec![],
                    state: AvailableActionState::None,
                },
                Action::LimitedCommand => AvailableActions {
                    a: vec![],
                    state: AvailableActionState::None,
                },
                _ => panic!("Invalid selected action from Command + Feat"),
            },
            AvailableActionState::C => match selection.unwrap() {
                Action::Pass => self,
                Action::CommandFeat => AvailableActions {
                    a: vec![],
                    state: AvailableActionState::None,
                },
                _ => panic!("Invalid selected action from Event"),
            },
            AvailableActionState::None => AvailableActions {
                a: vec![
                    Action::Pass,
                    Action::CommandOnly,
                    Action::CommandFeat,
                    Action::Event,
                ],
                state: AvailableActionState::Start,
            },
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
    eligibility_order: Vec<Player>,
    state: SequenceOfPlayState,
    available_actions: AvailableActions,
    selected_action: Option<Action>,
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
    fn new(e: Vec<Player>) -> Self {
        let mut player_eligibilities: HashMap<Player, PlayerState> = HashMap::new();
        player_eligibilities.insert(Civitates, Eligible);
        player_eligibilities.insert(Dux, Eligible);
        player_eligibilities.insert(Saxons, Eligible);
        player_eligibilities.insert(Scotti, Eligible);
        SequenceOfPlay {
            player_eligibilities: player_eligibilities,
            state: SequenceOfPlayState::GettingAction,
            available_actions: AvailableActions::new(),
            eligibility_order: e,
            selected_action: None,
        }
    }

    fn get_action(mut self, curr_player: Player) -> Result<Self, &'static str> {
        match self.state {
            SequenceOfPlayState::GettingAction => {
                println!("\n\nGetting first action from {}", curr_player,);
                let selection: Action = self.available_actions.a[Select::new()
                    .with_prompt(format!("Select one of the following actions!"))
                    .items(&self.available_actions.a)
                    .interact()
                    .unwrap()];
                println!("Selected {}", selection);
                self.selected_action = Some(selection);
                self.state = SequenceOfPlayState::Acting;
                return Ok(self);
            }
            _ => Err("Can only get action in GettingAction state"),
        }
    }

    fn acting(mut self, curr_player: Player) -> Result<Self, &'static str> {
        match self.state {
            SequenceOfPlayState::Acting => {
                println!("{} performing action: {:?}", curr_player, self.selected_action.unwrap());
                self.state = SequenceOfPlayState::GettingAction;
                self.available_actions = self.available_actions.update_available_actions(self.selected_action);
                return Ok(self);
            }
            _ => Err("Can only do action in Acting state"),
        }
    }

    fn cleanup(mut self) -> Result<Self, &'static str> {
        match self.state {
            SequenceOfPlayState::Reseting => {
                println!("Reseting for the next round");
                self.state = SequenceOfPlayState::GettingAction;
                self.available_actions = self.available_actions.update_available_actions(None);
                return Ok(self);
            }
            _ => Err("Can only do cleanup in Reseting state"),
        }
    }
}

fn build_deck() -> Vec<Event> {
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
    let test_card_2: Event = Event {
        eligibility: vec![
            Player::Dux,
            Player::Saxons,
            Player::Civitates,
            Player::Scotti,
        ],
        unshaded: None,
        shaded: None,
        historical_notes: String::from(""),
    };
    return vec![test_card_0, test_card_1, test_card_2];
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


    // TODO: when 2 players act and there are players left on the card, tries to go into the next round and crashes
    let deck: Vec<Event> = build_deck();
    let mut cards: std::vec::IntoIter<Event> = deck.into_iter();
    let new_card: Event = cards.next().unwrap();
    let mut sop: SequenceOfPlay = SequenceOfPlay::new(new_card.eligibility.clone());
    let mut curr_order = new_card.eligibility.into_iter();
    let mut cp: Option<Player> = curr_order.next();
    loop {
        match cp {
            Some(p) => match sop.state {
                SequenceOfPlayState::GettingAction => {
                    sop = sop.get_action(p).unwrap();
                }
                SequenceOfPlayState::Acting => {
                    sop = sop.acting(p).unwrap();
                    cp = curr_order.next();
                }
                SequenceOfPlayState::Reseting => {
                    sop = sop.cleanup().unwrap();
                    break;
                },
            },
            None => break,
        }
    }
    // TODO: Should not be advancing player every state transition, only after actions taken
    // Also worth considering if event deck will be part of SequenceOfPlay

    // Get our card
    // Go through the factions in card order and get actions
    // When 2 factions have acted or we have gone through the whole card, adjust eligibility and draw new card and new round
}
