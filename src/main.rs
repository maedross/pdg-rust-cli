use dialoguer::Select;
use std::collections::{HashMap, VecDeque};
use std::fmt;

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

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
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
        civitates_pieces: Vec<Unit>,
        dux_pieces: Vec<Unit>,
        saxon_pieces: Vec<Unit>,
        scotti_pieces: Vec<Unit>,
        stronghold_sites: Vec<StrongholdSite>,
    },
    Sea,
}

struct Rules {
    piece_types: Vec<Unit>,
    stronghold_types: Vec<Stronghold>,
    terrain_interactions: HashMap<(Unit, Terrain), Vec<PreBattle>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Unit {
    name: String,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Faction {
    Civitates,
    Dux,
    Saxons,
    Scotti,
}

#[derive(Debug, Clone, Copy)]
enum PreBattle {
    Ambush(u8),
    Evade(u8),
}

impl fmt::Display for PreBattle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PreBattle::Ambush(_) => write!(f, "Ambush"),
            PreBattle::Evade(_) => write!(f, "Evade"),
        }
    }
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
        self.take_turn(
            VecDeque::from(card.eligibility_order.clone()),
            &first_actions,
        );
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

struct BattleForces {
    attacking_force: Vec<Unit>,
    defending_force: Vec<Unit>,
    evaded_forces: Vec<Unit>,
    trapping_forces: Vec<Unit>,
}

fn main() {
    // Reveal next card
    // If epoch, swap with current and do epoch round
    // Else repeat
    // If no next card, do final scoring

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

    let (curr_card, new_card, deck, rules): (Card, Card, VecDeque<Card>, Rules) = setup();
    sequence_of_play.round_runner(&curr_card);
    sequence_of_play.round_runner(&new_card);
}

fn setup() -> (Card, Card, VecDeque<Card>, Rules) {
    let mut deck: VecDeque<Card> = load_cards();

    let curr_card: Card = deck.pop_front().unwrap();
    let next_card: Card = deck.pop_front().unwrap();

    let (mut units, mut interactions) = load_units();
    let rules: Rules = Rules {
        piece_types: units,
        stronghold_types: vec![],
        terrain_interactions: interactions,
    };
    return (curr_card, next_card, deck, rules);
}

fn load_cards() -> VecDeque<Card> {
    VecDeque::from(vec![
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
    ])
}

fn load_units() -> (Vec<Unit>, HashMap<(Unit, Terrain), Vec<PreBattle>>) {
    let militia: Unit = Unit {
        name: String::from("Militia"),
    };
    let comitates: Unit = Unit {
        name: String::from("Comitates"),
    };
    let cavalry: Unit = Unit {
        name: String::from("Cavalry"),
    };
    let saxon_raider: Unit = Unit {
        name: String::from("Raider"),
    };
    let saxon_warband: Unit = Unit {
        name: String::from("Warband"),
    };
    let scotti_raider: Unit = Unit {
        name: String::from("Raider"),
    };
    let scotti_warband: Unit = Unit {
        name: String::from("Warband"),
    };

    let mut terrain_interactions: HashMap<(Unit, Terrain), Vec<PreBattle>> = HashMap::new();
    terrain_interactions.insert(
        (saxon_raider.clone(), Terrain::Fens),
        vec![PreBattle::Evade(4), PreBattle::Ambush(5)],
    );
    terrain_interactions.insert(
        (saxon_raider.clone(), Terrain::Hills),
        vec![PreBattle::Evade(5)],
    );
    terrain_interactions.insert(
        (saxon_raider.clone(), Terrain::Clear),
        vec![PreBattle::Evade(6)],
    );

    terrain_interactions.insert(
        (saxon_warband.clone(), Terrain::Fens),
        vec![PreBattle::Evade(5), PreBattle::Ambush(3)],
    );

    terrain_interactions.insert(
        (scotti_raider.clone(), Terrain::Hills),
        vec![PreBattle::Evade(4), PreBattle::Ambush(5)],
    );
    terrain_interactions.insert(
        (scotti_raider.clone(), Terrain::Fens),
        vec![PreBattle::Evade(5)],
    );
    terrain_interactions.insert(
        (scotti_raider.clone(), Terrain::Clear),
        vec![PreBattle::Evade(6)],
    );

    terrain_interactions.insert(
        (scotti_warband.clone(), Terrain::Hills),
        vec![PreBattle::Evade(5), PreBattle::Ambush(3)],
    );

    let unit_types: Vec<Unit> = vec![
        militia.clone(),
        comitates.clone(),
        cavalry.clone(),
        saxon_raider.clone(),
        saxon_warband.clone(),
        scotti_raider.clone(),
        scotti_warband.clone(),
    ];

    let rules: Rules = Rules {
        piece_types: unit_types.clone(),
        stronghold_types: vec![],
        terrain_interactions: terrain_interactions.clone(),
    };
    check_evade_ambush(
        &rules,
        &Terrain::Fens,
        &vec![
            militia.clone(),
            saxon_raider.clone(),
            scotti_warband.clone(),
            saxon_warband.clone(),
            cavalry.clone(),
        ],
    );

    (unit_types, terrain_interactions)
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

impl Rules {
    fn check_evade_ambush(rules: &Rules, terrain: &Terrain, forces: &Vec<Unit>) {
        /*
           For each player
               For each group of forces
                   Table lookup? Would be nice. If a global table, could then just update the table when Cymbrogi or one of the nasty events is played
                   Table would need
                       One Piece
                       Many Terrain
                       One Evade
                           One Dice
                       One Withdraw
                           One Dice
                   A piece can be a struct, and a rules struct can instantiate the particulars
        */
        for unit in forces {
            match rules.terrain_interactions.get(&(unit.clone(), *terrain)) {
                Some(tactics) => {
                    println!("{} may:", unit.name);
                    for t in tactics {
                        println!("{}", t);
                    }
                }
                None => {}
            }
        }
    }

    fn battle(
        attacker: Faction,
        defender: Faction,
        space: Space,
        fragmentation: bool,
        rules: &Rules,
    ) {
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

        */
        match space {
            Space::Land {
                terrain,
                control: _,
                population: _,
                prosperity: _,
                civitates_pieces,
                dux_pieces,
                saxon_pieces,
                scotti_pieces,
                stronghold_sites: _,
            } => {
                match attacker {
                    Faction::Civitates => check_evade_ambush(rules, &terrain, &civitates_pieces),
                    Faction::Dux => check_evade_ambush(rules, &terrain, &dux_pieces),
                    Faction::Saxons => check_evade_ambush(rules, &terrain, &saxon_pieces),
                    Faction::Scotti => check_evade_ambush(rules, &terrain, &scotti_pieces),
                }

                match defender {
                    Faction::Civitates => check_evade_ambush(rules, &terrain, &civitates_pieces),
                    Faction::Dux => check_evade_ambush(rules, &terrain, &dux_pieces),
                    Faction::Saxons => check_evade_ambush(rules, &terrain, &saxon_pieces),
                    Faction::Scotti => check_evade_ambush(rules, &terrain, &scotti_pieces),
                }
            }
            _ => panic!(
                "Why are you trying to fight a land in the middle of the sea? Don't be like Caligula"
            ),
        }

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
}
