use events::Event;
use sequence_of_play::{SequenceOfPlay, SequenceOfPlayState};
use serde::{Deserialize, Serialize};
use serde_yaml::{Result, Value};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::{collections::VecDeque, println, env};

use crate::board::{build_map_from_yaml, build_scenario_from_yaml};
mod concepts;
mod events;
mod sequence_of_play;
//mod setup;
//mod commands;
mod board;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum Enum {
    Unit,
    Newtype(usize),
    Tuple(usize, usize, usize),
    Struct { x: f64, y: f64 },
}

fn main() {
    build_scenario_from_yaml(
        "C:\\Users\\matth\\Documents\\GitHub\\pdg-rust-cli\\src\\setup\\map.yaml",
        "C:\\Users\\matth\\Documents\\GitHub\\pdg-rust-cli\\src\\setup\\scenario_de_excidio_britanniae.yaml",
    );
}
/*
fn main() {
    let deck: VecDeque<Event> = setup::build_deck();
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
*/
