use colored::Colorize;
use dialoguer::Select;
use std::collections::HashMap;
use std::collections::VecDeque;
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
#[derive(Clone, Debug, PartialEq, Eq)]
enum EventType {
    Standard,
    Epoch,
    Pivotal,
}

#[derive(Clone, Debug)]
struct Event {
    name: String,
    eligibility: Vec<Player>,
    unshaded: Option<u8>,
    shaded: Option<u8>,
    historical_notes: String,
    event_type: EventType,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.eligibility)
    }
}

// Sequence of Play

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
enum PlayerState {
    Eligible,
    Passed,
    Acted,
    Ineligible,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum SequenceOfPlayState {
    CheckEndRound,
    CheckPlayerStatus,
    ChoosingAction,
    Acting,
    ResetEligibility,
    AdvanceEvents,
    Epoch,
}

#[derive(Clone)]
struct SequenceOfPlay {
    player_eligibilities: HashMap<Player, PlayerState>,
    current_player: usize,
    state: SequenceOfPlayState,
    available_actions: AvailableActions,
    selected_action: Option<Action>,
    event_deck: VecDeque<Event>,
    current_event: Event,
    event_discard: VecDeque<Event>,
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

/*
    TODO: Implement player state checking and setting
    When a player selected a non-pass action, they should be set to acting
    When getting action, should be checking player is eligible
    Cleanup should move ineligible and passed players to eligible, acted to ineligible

    Give cards names and print them, round number
*/
impl SequenceOfPlay {
    fn new(mut events: VecDeque<Event>) -> Self {
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
        }
    }

    fn check_end_round(mut self) -> Self {
        println!("Checking for end of round...");
        match self.state {
            SequenceOfPlayState::CheckEndRound => {
                if self.current_player > 3
                    || self.available_actions.state == AvailableActionState::End
                {
                    print!("Ending round");
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

    fn check_player_status(mut self) -> Self {
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

    fn get_action(mut self) -> Self {
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

    fn acting(mut self) -> Self {
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

    fn reset_eligibility(mut self) -> Self {
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

    fn advance_events(mut self) -> Self {
        println!("Advancing events...");
        match self.state {
            SequenceOfPlayState::AdvanceEvents => {
                self.event_discard.push_front(self.current_event);
                self.current_event = self.event_deck.pop_front().unwrap();
                self.current_player = 0;
                match self.event_deck[0].event_type {
                    EventType::Standard => self.state = SequenceOfPlayState::ChoosingAction,
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

    fn epoch(mut self) -> Self {
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

fn build_deck() -> VecDeque<Event> {
    let calleva_atrebatum: Event = Event {
        name: String::from("Calleva Atrebatum"),
        eligibility: vec![Saxons, Scotti, Dux, Civitates],
        unshaded: None,
        shaded: None,
        historical_notes: String::from(""),
        event_type: EventType::Standard,
    };
    let ard_ri: Event = Event {
        name: String::from("Ard Ri"),
        eligibility: vec![Scotti, Dux, Saxons, Civitates],
        unshaded: None,
        shaded: None,
        historical_notes: String::from(""),
        event_type: EventType::Standard,
    };
    let anderida: Event = Event {
        name: String::from("Anderida"),
        eligibility: vec![Saxons, Dux, Scotti, Civitates],
        unshaded: None,
        shaded: None,
        historical_notes: String::from(""),
        event_type: EventType::Standard,
    };
    let classis_britannica: Event = Event {
        name: String::from("Classis Britannica"),
        eligibility: vec![Dux, Saxons, Civitates, Scotti],
        unshaded: None,
        shaded: None,
        historical_notes: String::from(""),
        event_type: EventType::Standard,
    };
    let recruits: Event = Event {
        name: String::from("Recruits"),
        eligibility: vec![Dux, Scotti, Saxons, Civitates],
        unshaded: None,
        shaded: None,
        historical_notes: String::from(""),
        event_type: EventType::Standard,
    };
    let deira: Event = Event {
        name: String::from("Deira"),
        eligibility: vec![Civitates, Saxons, Dux, Scotti],
        unshaded: None,
        shaded: None,
        historical_notes: String::from(""),
        event_type: EventType::Standard,
    };
    let with_the_cross_on_his_shoulders: Event = Event {
        name: String::from("With The Cross On His Shoulders"),
        eligibility: vec![Civitates, Scotti, Dux, Saxons],
        unshaded: None,
        shaded: None,
        historical_notes: String::from(""),
        event_type: EventType::Standard,
    };
    let ambrosius_aurelianus: Event = Event {
        name: String::from("Ambrosius Aurelianus"),
        eligibility: vec![Civitates, Dux, Scotti, Saxons],
        unshaded: None,
        shaded: None,
        historical_notes: String::from(""),
        event_type: EventType::Standard,
    };
    let celyddon_coed: Event = Event {
        name: String::from("Celyddon Coed"),
        eligibility: vec![Scotti, Dux, Civitates, Saxons],
        unshaded: None,
        shaded: None,
        historical_notes: String::from(""),
        event_type: EventType::Standard,
    };
    let fickle_weather: Event = Event {
        name: String::from("Fickle Weather"),
        eligibility: vec![Scotti, Dux, Civitates, Saxons],
        unshaded: None,
        shaded: None,
        historical_notes: String::from(""),
        event_type: EventType::Standard,
    };
    let magnus_maximus: Event = Event {
        name: String::from("Magnus Maximus"),
        eligibility: vec![],
        unshaded: None,
        shaded: None,
        historical_notes: String::from(""),
        event_type: EventType::Epoch,
    };
    let mut deck: VecDeque<Event> = VecDeque::new();
    deck.push_back(calleva_atrebatum);
    deck.push_back(ard_ri);
    deck.push_back(anderida);
    deck.push_back(classis_britannica);
    deck.push_back(recruits);
    deck.push_back(deira);
    deck.push_back(with_the_cross_on_his_shoulders);
    deck.push_back(ambrosius_aurelianus);
    deck.push_back(celyddon_coed);
    deck.push_back(fickle_weather);
    deck.push_back(magnus_maximus);
    return deck;
}

fn main() {
    let deck: VecDeque<Event> = build_deck();
    let mut sop: SequenceOfPlay = SequenceOfPlay::new(deck);
    loop {
        match sop.state {
            SequenceOfPlayState::CheckEndRound => {
                sop = sop.check_end_round();
            }
            SequenceOfPlayState::CheckPlayerStatus => {
                sop = sop.check_player_status();
            }
            SequenceOfPlayState::ChoosingAction => {
                sop = sop.get_action();
            }
            SequenceOfPlayState::Acting => {
                sop = sop.acting();
            }
            SequenceOfPlayState::ResetEligibility => {
                sop = sop.reset_eligibility();
            }
            SequenceOfPlayState::AdvanceEvents => {
                sop = sop.advance_events();
            }
            SequenceOfPlayState::Epoch => {
                sop.epoch();
                println!("Only one Epoch so far, more to come later");
                break;
            }
        };
    }
}
