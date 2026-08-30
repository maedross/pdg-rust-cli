use events::Event;
use sequence_of_play::{SequenceOfPlay, SequenceOfPlayState};
use std::{collections::VecDeque, fs::{File, remove_file}, println, sync::Mutex};
use tracing_subscriber::{fmt};

use crate::board::build_scenario_from_yaml;
mod board;
mod commands;
mod concepts;
mod events;
mod sequence_of_play;
mod setup;

fn main() {
    let _ = remove_file("debug.log");
    let format = fmt::format::format()
        .pretty()
        .with_target(false)
        .with_source_location(false);
    tracing_subscriber::fmt()
        .event_format(format)
        .with_writer(Mutex::new(File::create("debug.log").unwrap()))
        .with_ansi(false)
        .init();

    let board: board::Board = build_scenario_from_yaml(
        "C:\\Users\\matth\\Documents\\GitHub\\pdg-rust-cli\\src\\setup\\map.yaml",
        "C:\\Users\\matth\\Documents\\GitHub\\pdg-rust-cli\\src\\setup\\scenario_de_excidio_britanniae.yaml",
    );
    let deck: VecDeque<Event> = setup::build_deck();
    let mut sop: SequenceOfPlay = SequenceOfPlay::new(deck, board);
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
