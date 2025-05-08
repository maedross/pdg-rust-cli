use dialoguer::Select;
use std::collections::VecDeque;
use std::fmt;
use std::io;

enum Action {
    Pass,
    FirstFactionCommandOnly,
    FirstFactionCommandPlusFeat,
    FirstFactionEvent,
    SecondFactionLimitedCommand,
    SecondFactionEventOrLimitedCommand,
    SecondFactionCommandPlusFeat,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Pass => write!(f, "Pass"),
            Action::FirstFactionCommandOnly => write!(f, "Command Only"),
            Action::FirstFactionCommandPlusFeat => write!(f, "Command + Feat"),
            Action::FirstFactionEvent => write!(f, "Event"),
            Action::SecondFactionLimitedCommand => write!(f, "Limited Command"),
            Action::SecondFactionEventOrLimitedCommand => write!(f, "Event or Limited Command"),
            Action::SecondFactionCommandPlusFeat => write!(f, "Command + Feat"),
        }
    }
}

enum Terrain {
    Clear,
    Fens,
    Hills,
    City,
}
enum Space {
    Land {
        terrain: Terrain,
        control: Control,
        population: u8,
        prosperity: Vec<u8>,
        civitates_pieces: Vec<Forces>,
        dux_pieces: Vec<Forces>,
        saxon_pieces: Vec<Forces>,
        scotti_pieces: Vec<Forces>,
        stronghold_sites: Vec<StrongholdSite>,
    },
    Sea,
}

struct Road {}

struct StrongholdSite {
    name: String,
    stronghold: Option<Stronghold>,
    site_type: StrongholdSiteType,
}

enum StrongholdSiteType {
    Hillfort,
    Town,
}

enum Stronghold {
    None,
    Hillfort,
    Town,
    Fort,
    SaxonSettlement,
    ScottiSettlement,
}

enum Forces {
    Militia,
    Comitates,
    Cavalry,
    SaxonFoederati(Faction),
    ScottiFoederati(Faction),
    SaxonWarband,
    ScottiWarband,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

struct SequenceOfPlay {
    eligible_factions: Vec<Faction>,
    passed_factions: Vec<Faction>,
    acting_factions: Vec<Faction>,
    ineligible_factions: Vec<Faction>,
}

impl SequenceOfPlay {
    fn round_runner(&mut self, card: &Card) {
        println!("\nCurrent Card: {} ({})", card.name, card.number);
        let first_actions = vec![
        Action::Pass,
        Action::FirstFactionCommandOnly,
        Action::FirstFactionCommandPlusFeat,
        Action::FirstFactionEvent,
    ];
        self.take_turn(VecDeque::from(card.eligibility_order.clone()), &first_actions);
    }

    fn take_turn(&mut self, mut eligibility_order: VecDeque<Faction>, actions: &Vec<Action>) {
        match eligibility_order.pop_front() {
            None => self.reset(),
            Some(curr_faction) => {
                if self.eligible_factions.contains(&curr_faction) {
                    println!("{} turn", curr_faction);
                    let selection = Select::new()
                        .with_prompt("What do you choose?")
                        .items(&actions)
                        .interact()
                        .unwrap();

                    self.eligible_factions.retain(|f| f != &curr_faction);
                    match actions[selection] {
                        Action::Pass => {
                            self.passed_factions.push(curr_faction);
                            self.take_turn(eligibility_order, actions);
                        }
                        Action::FirstFactionCommandOnly => {
                            self.acting_factions.push(curr_faction);
                            self.take_turn(
                                eligibility_order,
                                &vec![Action::Pass, Action::SecondFactionLimitedCommand],
                            );
                        }
                        Action::FirstFactionCommandPlusFeat => {
                            self.acting_factions.push(curr_faction);
                            self.take_turn(
                                eligibility_order,
                                &vec![Action::Pass, Action::SecondFactionEventOrLimitedCommand],
                            );
                        }
                        Action::FirstFactionEvent => {
                            self.acting_factions.push(curr_faction);
                            self.take_turn(
                                eligibility_order,
                                &vec![Action::Pass, Action::SecondFactionCommandPlusFeat],
                            );
                        }
                        Action::SecondFactionLimitedCommand
                        | Action::SecondFactionEventOrLimitedCommand
                        | Action::SecondFactionCommandPlusFeat => {
                            self.acting_factions.push(curr_faction);
                            self.reset();
                        }
                    }
                } else {
                    self.take_turn(eligibility_order, actions);
                }
            }
        };
    }

    fn reset(&mut self) {
        self.eligible_factions.append(&mut self.passed_factions);
        self.eligible_factions.append(&mut self.ineligible_factions);
        self.passed_factions = vec![];
        self.ineligible_factions = self.acting_factions.clone();
        self.acting_factions = vec![];
    }
}

impl Default for SequenceOfPlay {
    fn default() -> SequenceOfPlay {
        SequenceOfPlay {
            eligible_factions: vec![
                Faction::Civitates,
                Faction::Dux,
                Faction::Saxons,
                Faction::Scotti,
            ],
            passed_factions: vec![],
            acting_factions: vec![],
            ineligible_factions: vec![],
        }
    }
}

fn main() {
    // Reveal next card
    // If epoch, swap with current and do epoch round
    // Else repeat
    // If no next card, do final scoring
    let mut deck: VecDeque<Card> = VecDeque::from(vec![
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
    ]);

    let corieltauvi = Space::Land {
        terrain: Terrain::Fens,
        control: Control::BritonControl,
        population: 2,
        prosperity: vec![2, 2],
        civitates_pieces: vec![],
        dux_pieces: vec![],
        saxon_pieces: vec![],
        scotti_pieces: vec![],
        stronghold_sites: vec![
            StrongholdSite {
                name: String::from("Lindum"),
                stronghold: Some(Stronghold::Fort),
                site_type: StrongholdSiteType::Town,
            },
            StrongholdSite {
                name: String::from("Ratae"),
                stronghold: Some(Stronghold::Town),
                site_type: StrongholdSiteType::Town,
            },
        ],
    };

    let mut sequence_of_play: SequenceOfPlay = SequenceOfPlay {
        ..Default::default()
    };
    
    let (curr_card, new_card): (Card, Card) = setup(&mut deck);
    sequence_of_play.round_runner(&curr_card);
    sequence_of_play.round_runner(&new_card);
}

fn setup(deck: &mut VecDeque<Card>) -> (Card, Card) {
    let curr_card: Card = deck.pop_front().unwrap();
    let next_card: Card = deck.pop_front().unwrap();
    return (curr_card, next_card);
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

fn Battle(attacker: Faction, defender: Faction, location: Space, fragmentation: bool) {
    /*
       Required Starting Information

       Fragmentation Status
       Attacking player
       Defending player(s)
       Space
       Attacking force/is this from a Raid
       Feats: Retaliate, Shield Wall, Surprise, Reinforce, Ravage
    */

    /*
       Required Tracking
       Are Cavalry fighting?
       Are Barbarians fighting?
       How many pieces does each side lose?
       Did a side survive?
       Was a stronghold Assaulted?
       Were units chosen to Siege a Stronghold?
       Plunder capacity
    */

    // If not Fragmentation or Dux vs Civis, Britons fight together
    // Raid battle only fights with placed Raiders

    /*
       Pre-Battle
       Raiders may Evade in all Terrain or Ambush in Home Terrain
       Warbands, Foederati, Comitates, Militia may Evade or Ambush in Home Terrain
    */

    /*
       PRE-BATTLE

       for attackers and defenders
           if forces include raiders
               if in home terrain evade (3/6) or ambush (2/6)
               if in rough terrain evade (2/6)
               if in clear terrain evade (1/6)
               or don't
           if forces include Warbands, Foederati OR (Comitates or Militia) WITH Cymbrogi
               may ambush (4/6) or evade (2/6)
    */

    /*
       Field Battle
       If all defending units Evaded, skip
       Militia and Raiders are halved
       1. Trap
       2. Defenders Withdraw
       3. Charge/Ambush
       4. Melee
       5. Harass
    */

    /*
       Assault
       If all defenders Evaded, Withdrew, or died
       Target Strongholds 1 by 1
       1. Coup de Main
       2. Escalade
       3. Storm
       4. If all defending units + garrison killed and attackers remain, remove Stronghold
    */

    /*
       Battle Consequences
       * If enemy is not pre-Frag Civis, if Cavalry fought and its side lost fewer pieces than enemy, +1 Prestige.
            If cavalry fought and its side lost more pieces than the enemy, -1 Prestige.
       * Each Fort or if not Frag Town removed, -2 Prestige
       * Each Stronghold destroyed grants 2 plunder, 3 if Town. Cavalry only take plunder if Retaliate.
            If Dux, +1 Prestige. If City, plunder all Prosperity.
       * If a Barbarian lost fewer pieces than the enemy and survived, +1 Renown
       * If Attacker kills units with Plunder, either distribute half rounded down among non-Cavalry attacking units,
            or if Briton may return 1 Prosperity to the space, or if Barbarian in controlled space +1 Renown for each plunder
    */

    /*
       Siege
       If all defending units Evaded, Withdrew, or were removed
       If Stronghold was not Assaulted and attacker has >= Troops as the Stronghold's Capacity,
       defender must remove 1 unit from inside.
       Attackers are counted for only 1 Stronghold each
    */
}
