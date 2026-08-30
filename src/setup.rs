use std::collections::VecDeque;
use std::todo;
use tracing::{Level, event, instrument};

use super::concepts::Player::{Civitates, Dux, Saxons, Scotti};
use super::events::{Event, EventType};

#[instrument]
pub fn build_deck() -> VecDeque<Event> {
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
    event!(Level::INFO, "Pushed Calleva Atrebatum to back");
    deck.push_back(ard_ri);
    event!(Level::INFO, "Pushed Ard Ri to back");
    deck.push_back(anderida);
    event!(Level::INFO, "Pushed Anderida to back");
    deck.push_back(classis_britannica);
    event!(Level::INFO, "Pushed Classis Britannica to back");
    deck.push_back(recruits);
    event!(Level::INFO, "Pushed Recruits to back");
    deck.push_back(deira);
    event!(Level::INFO, "Pushed Deira to back");
    deck.push_back(with_the_cross_on_his_shoulders);
    event!(Level::INFO, "Pushed With The Cross On His Shoulders to back");
    deck.push_back(ambrosius_aurelianus);
    event!(Level::INFO, "Pushed Ambrosius Aurelianus to back");
    deck.push_back(celyddon_coed);
    event!(Level::INFO, "Pushed Celyddon Coed to back");
    deck.push_back(fickle_weather);
    event!(Level::INFO, "Pushed Fickle Weather to back");
    deck.push_back(magnus_maximus);
    event!(Level::INFO, "Pushed Magnus Maximus to back");
    return deck;
}
