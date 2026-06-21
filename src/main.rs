use events::Event;
use sequence_of_play::{SequenceOfPlay, SequenceOfPlayState};
use std::collections::VecDeque;

mod concepts;
mod events;
mod sequence_of_play;
mod setup;

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
