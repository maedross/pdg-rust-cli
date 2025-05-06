use std::fmt;
use std::io;

enum Space {
    Region,
    City,
    Sea,
}

struct Road {}

struct StrongholdSite {
    name: String,
}

enum Forces {
    Unit,
    Stronghold,
}

enum Faction {
    Civitates,
    Dux,
    Saxons,
    Scotti,
}

enum Control {
    BritonControl,
    DuxControl,
    SaxonControl,
    ScottiControl,
    NoControl,
}

enum CardType {
    Event,
    Epoch,
    Pivotal,
}

struct Card {
    number: u8,
    name: String,
    eligibility_order: Vec<Faction>,
    card_type: CardType,
}

enum LeadAction {
    LimCmd,
    CmdFt,
    Ev,
}

fn main() {
    // Check current card
    // Offer turn to players in eligibility order
    // Allow for cmd, cmd+f, ev, pass
    // Offer turn to second players in eligibility order
    // When none left or turn taken, move current card to discard, move next card to current
    // Reveal next card
    // If epoch, swap with current and do epoch round
    // Else repeat
    // If no next card, do final scoring
    let mut deck: Vec<Card> = vec![
        Card {
            number: 43,
            name: "Omens".to_string(),
            eligibility_order: vec![
                Faction::Saxons,
                Faction::Civitates,
                Faction::Dux,
                Faction::Scotti,
            ],
            card_type: CardType::Event,
        },
        Card {
            number: 44,
            name: "Lindsey".to_string(),
            eligibility_order: vec![
                Faction::Saxons,
                Faction::Civitates,
                Faction::Dux,
                Faction::Scotti,
            ],
            card_type: CardType::Event,
        },
    ];
    let (curr_card, new_card): (Card, Card) = setup(&mut deck);
    let mut factions_to_act: u8 = 2;
    let mut faction_index: usize = 0;
    while factions_to_act > 0 && faction_index < 4 {
        println!("{} turn", curr_card.eligibility_order[faction_index]);
        if factions_to_act == 2 {
            println!("Pass, LimCmd, Cmd+F, Ev");
            let mut action = String::new();
            io::stdin()
                .read_line(&mut action)
                .expect("Failed to read line");
            match action.as_str() {
                "Pass" => faction_index += 1,
                "LimCmd" => {
                    println!("Chose LimCmd");
                    faction_index += 1;
                    factions_to_act -= 1;
                }
                "Cmd+F" => {
                    println!("Chose Cmd+F");
                    faction_index += 1;
                    factions_to_act -= 1;
                }
                "Ev" => {
                    println!("Chose Ev");
                    faction_index += 1;
                    factions_to_act -= 1;
                }
                _ => {
                    panic!("Invalid input")
                }
            }
        } else {
            println!("Pass, LimCmd");
            let mut action = String::new();
            io::stdin()
                .read_line(&mut action)
                .expect("Failed to read line");
            match action.as_str() {
                "Pass" => faction_index += 1,
                "LimCmd" => {
                    println!("Chose LimCmd");
                    faction_index += 1;
                    factions_to_act -= 1;
                }
                _ => {
                    panic!("Invalid input")
                }
            }
        }
    }

}

fn setup(deck: &mut Vec<Card>) -> (Card, Card) {
    let curr_card: Card = deck.pop().unwrap();
    let next_card: Card = deck.pop().unwrap();
    return (curr_card, next_card);
}

fn reset(deck: &mut Vec<Card>) {

}

fn event() {}

fn command(limited: bool) {}

impl fmt::Display for Faction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Faction::Civitates => write!(f, "Civitates"),
            Faction::Dux => write!(f, "Dux"),
            Faction::Saxons => write!(f, "Saxons"),
            Faction::Scotti => write!(f, "Scotti"),
        }
    }
}
