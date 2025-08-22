use dialoguer::Select;
use rand::prelude::*;
use std::collections::VecDeque;
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Faction {
    Civitates,
    Dux,
    Saxons,
    Scotti,
    None,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
enum Terrain {
    Clear,
    Fens,
    Hills,
    City,
}

#[derive(Debug, Clone)]
struct Space {
    terrain: Terrain,
    control: Faction,
    population: u8,
    prosperity: Vec<u8>,
    comitates: u8,
    civitates_saxon_foederati: u8,
    civitates_scotti_foederati: u8,
    militia: u8,
    cavalry: u8,
    dux_saxon_foederati: u8,
    dux_scotti_foederati: u8,
    saxon_raider: u8,
    saxon_warband: u8,
    scotti_raider: u8,
    scotti_warband: u8,
    stronghold_sites: Vec<StrongholdSite>,
}

impl Default for Space {
    fn default() -> Space {
        Space {
            comitates: 0,
            civitates_saxon_foederati: 0,
            civitates_scotti_foederati: 0,
            militia: 0,
            cavalry: 0,
            dux_saxon_foederati: 0,
            dux_scotti_foederati: 0,
            saxon_raider: 0,
            saxon_warband: 0,
            scotti_raider: 0,
            scotti_warband: 0,
            terrain: Terrain::Clear,
            control: Faction::None,
            population: 0,
            prosperity: vec![0, 0],
            stronghold_sites: vec![],
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Unit {
    Comitates,
    CivitatesSaxonFoederati,
    CivitatesScottiFoederati,
    Militia,
    Cavalry,
    DuxSaxonFoederati,
    DuxScottiFoederati,
    SaxonRaider,
    SaxonWarband,
    ScottiRaider,
    ScottiWarband,
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unit::Comitates => write!(f, "Comitates"),
            Unit::CivitatesSaxonFoederati => write!(f, "Civitates Saxon Foederati"),
            Unit::CivitatesScottiFoederati => write!(f, "Civitates Scotti Foederati"),
            Unit::Militia => write!(f, "Militia"),
            Unit::Cavalry => write!(f, "Cavalry"),
            Unit::DuxSaxonFoederati => write!(f, "Dux Saxon Foederati"),
            Unit::DuxScottiFoederati => write!(f, "Dux Scotti Foederati"),
            Unit::SaxonRaider => write!(f, "Saxon Raider"),
            Unit::SaxonWarband => write!(f, "Saxon Warband"),
            Unit::ScottiRaider => write!(f, "Scotti Raider"),
            Unit::ScottiWarband => write!(f, "Scotti Warband"),
        }
    }
}

struct Road {}
enum Stronghold {
    None,
    Hillfort,
    Town,
    Fort,
    SaxonSettlement,
    ScottiSettlement,
}

#[derive(Debug, Clone, Copy)]
enum PreBattle {
    Ambush,
    Evade,
    None,
}

impl fmt::Display for PreBattle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PreBattle::Ambush => write!(f, "Ambush"),
            PreBattle::Evade => write!(f, "Evade"),
            PreBattle::None => write!(f, "None"),
        }
    }
}

#[derive(Debug, Clone)]
struct StrongholdSite {
    name: String,
    stronghold: Option<Stronghold>,
    town_allowed: bool,
}

struct Card {
    number: u8,
    name: String,
    eligibility_order: Vec<Faction>,
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

    let corieltauvi = Space {
        terrain: Terrain::Fens,
        control: Faction::Civitates,
        population: 2,
        prosperity: vec![2, 2],
        stronghold_sites: vec![
            StrongholdSite {
                name: String::from("Lindum"),
                stronghold: Some(Stronghold::Fort),
                town_allowed: true,
            },
            StrongholdSite {
                name: String::from("Ratae"),
                stronghold: Some(Stronghold::Town),
                town_allowed: true,
            },
        ],
        ..Default::default()
    };

    let mut sequence_of_play: SequenceOfPlay = SequenceOfPlay {
        ..Default::default()
    };

    let (curr_card, new_card, deck): (Card, Card, VecDeque<Card>) = setup();
    sequence_of_play.round_runner(&curr_card);
    sequence_of_play.round_runner(&new_card);
}

fn setup() -> (Card, Card, VecDeque<Card>) {
    let mut deck: VecDeque<Card> = load_cards();

    let curr_card: Card = deck.pop_front().unwrap();
    let next_card: Card = deck.pop_front().unwrap();

    return (curr_card, next_card, deck);
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
        },
    ])
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

fn get_evade(unit: Unit, terrain: Terrain) -> Option<u8> {
    match (unit, terrain) {
        (Unit::Comitates, Terrain::Hills) => Some(5),
        (Unit::Comitates, _) => None,
        (Unit::CivitatesSaxonFoederati, Terrain::Fens) => Some(5),
        (Unit::CivitatesSaxonFoederati, _) => None,
        (Unit::CivitatesScottiFoederati, Terrain::Hills) => Some(5),
        (Unit::CivitatesScottiFoederati, _) => None,
        (Unit::Militia, Terrain::Hills) => Some(5),
        (Unit::Militia, _) => None,
        (Unit::Cavalry, _) => None,
        (Unit::DuxSaxonFoederati, Terrain::Fens) => Some(5),
        (Unit::DuxSaxonFoederati, _) => None,
        (Unit::DuxScottiFoederati, Terrain::Hills) => Some(5),
        (Unit::DuxScottiFoederati, _) => None,
        (Unit::SaxonRaider, Terrain::Fens) => Some(4),
        (Unit::SaxonRaider, Terrain::Hills) => Some(5),
        (Unit::SaxonRaider, Terrain::Clear) => Some(6),
        (Unit::SaxonWarband, Terrain::Fens) => Some(5),
        (Unit::SaxonWarband, _) => None,
        (Unit::ScottiRaider, Terrain::Hills) => Some(4),
        (Unit::ScottiRaider, Terrain::Fens) => Some(5),
        (Unit::ScottiRaider, Terrain::Clear) => Some(6),
        (Unit::ScottiWarband, Terrain::Hills) => Some(5),
        (Unit::ScottiWarband, _) => None,
        (_, _) => None,
    }
}

fn get_ambush(unit: Unit, terrain: Terrain) -> Option<u8> {
    match (unit, terrain) {
        (Unit::Comitates, Terrain::Hills) => Some(3),
        (Unit::Comitates, _) => None,
        (Unit::CivitatesSaxonFoederati, Terrain::Fens) => Some(3),
        (Unit::CivitatesSaxonFoederati, _) => None,
        (Unit::CivitatesScottiFoederati, Terrain::Hills) => Some(3),
        (Unit::CivitatesScottiFoederati, _) => None,
        (Unit::Militia, Terrain::Hills) => Some(3),
        (Unit::Militia, _) => None,
        (Unit::Cavalry, _) => None,
        (Unit::DuxSaxonFoederati, Terrain::Fens) => Some(3),
        (Unit::DuxSaxonFoederati, _) => None,
        (Unit::DuxScottiFoederati, Terrain::Hills) => Some(3),
        (Unit::DuxScottiFoederati, _) => None,
        (Unit::SaxonRaider, Terrain::Fens) => Some(5),
        (Unit::SaxonWarband, Terrain::Fens) => Some(3),
        (Unit::SaxonWarband, _) => None,
        (Unit::ScottiRaider, Terrain::Hills) => Some(5),
        (Unit::ScottiWarband, Terrain::Hills) => Some(3),
        (Unit::ScottiWarband, _) => None,
        (_, _) => None,
    }
}

fn prebattle_assign(
    player: Faction,
    space: Space,
) -> (Vec<Unit>, Vec<Unit>, Vec<Unit>, Vec<Unit>, Vec<Unit>) {
    let prebattle: Vec<PreBattle> = vec![PreBattle::Evade, PreBattle::Ambush, PreBattle::None];
    // For raiders not in home terrain
    let raider_prebattle: Vec<PreBattle> = vec![PreBattle::Evade, PreBattle::None];

    //let mut trap: Vec<Unit> = vec![];
    let mut charge_or_ambush: Vec<Unit> = vec![];
    let mut melee: Vec<Unit> = vec![];
    let mut harass: Vec<Unit> = vec![];
    let mut roll_evade: Vec<Unit> = vec![];
    let mut roll_ambush: Vec<Unit> = vec![];

    match player {
        Faction::Civitates => {
            if space.militia > 0 {
                if space.terrain == Terrain::Hills {
                    let selection = Select::new()
                        .with_prompt("What do Militia choose?")
                        .items(&prebattle)
                        .interact()
                        .unwrap();

                    match prebattle[selection] {
                        PreBattle::Ambush => roll_ambush.push(Unit::Militia),
                        PreBattle::Evade => roll_evade.push(Unit::Militia),
                        PreBattle::None => melee.push(Unit::Militia),
                    }
                } else {
                    melee.push(Unit::Militia);
                }
            }
            if space.comitates > 0 {
                if space.terrain == Terrain::Hills {
                    let selection = Select::new()
                        .with_prompt("What do Comitates choose?")
                        .items(&prebattle)
                        .interact()
                        .unwrap();

                    match prebattle[selection] {
                        PreBattle::Ambush => roll_ambush.push(Unit::Comitates),
                        PreBattle::Evade => roll_evade.push(Unit::Comitates),
                        PreBattle::None => melee.push(Unit::Comitates),
                    }
                } else {
                    melee.push(Unit::Comitates);
                }
            }
            if space.civitates_saxon_foederati > 0 {
                if space.terrain == Terrain::Fens {
                    let selection = Select::new()
                        .with_prompt("What do Civitates Saxon Foederati choose?")
                        .items(&prebattle)
                        .interact()
                        .unwrap();

                    match prebattle[selection] {
                        PreBattle::Ambush => roll_ambush.push(Unit::CivitatesSaxonFoederati),
                        PreBattle::Evade => roll_evade.push(Unit::CivitatesSaxonFoederati),
                        PreBattle::None => melee.push(Unit::CivitatesSaxonFoederati),
                    }
                } else {
                    melee.push(Unit::CivitatesSaxonFoederati);
                }
            }
            if space.civitates_scotti_foederati > 0 {
                if space.terrain == Terrain::Hills {
                    let selection = Select::new()
                        .with_prompt("What do Civitates Scotti Foederati choose?")
                        .items(&prebattle)
                        .interact()
                        .unwrap();

                    match prebattle[selection] {
                        PreBattle::Ambush => roll_ambush.push(Unit::CivitatesScottiFoederati),
                        PreBattle::Evade => roll_evade.push(Unit::CivitatesScottiFoederati),
                        PreBattle::None => melee.push(Unit::CivitatesScottiFoederati),
                    }
                } else {
                    melee.push(Unit::CivitatesScottiFoederati);
                }
            }
        }
        Faction::Dux => {
            if space.cavalry > 0 {
                charge_or_ambush.push(Unit::Cavalry);
            }
            if space.dux_saxon_foederati > 0 {
                if space.terrain == Terrain::Fens {
                    let selection = Select::new()
                        .with_prompt("What do Dux Saxon Foederati choose?")
                        .items(&prebattle)
                        .interact()
                        .unwrap();

                    match prebattle[selection] {
                        PreBattle::Ambush => roll_ambush.push(Unit::DuxSaxonFoederati),
                        PreBattle::Evade => roll_evade.push(Unit::DuxSaxonFoederati),
                        PreBattle::None => melee.push(Unit::DuxSaxonFoederati),
                    }
                } else {
                    melee.push(Unit::DuxSaxonFoederati);
                }
            }
            if space.dux_scotti_foederati > 0 {
                if space.terrain == Terrain::Hills {
                    let selection = Select::new()
                        .with_prompt("What do Dux Scotti Foederati choose?")
                        .items(&prebattle)
                        .interact()
                        .unwrap();

                    match prebattle[selection] {
                        PreBattle::Ambush => roll_ambush.push(Unit::DuxScottiFoederati),
                        PreBattle::Evade => roll_evade.push(Unit::DuxScottiFoederati),
                        PreBattle::None => melee.push(Unit::DuxScottiFoederati),
                    }
                } else {
                    melee.push(Unit::DuxScottiFoederati);
                }
            }
        }
        Faction::Saxons => {
            if space.saxon_raider > 0 {
                if space.terrain == Terrain::Fens {
                    let selection = Select::new()
                        .with_prompt("What do Saxon Raiders choose?")
                        .items(&prebattle)
                        .interact()
                        .unwrap();

                    match prebattle[selection] {
                        PreBattle::Ambush => roll_ambush.push(Unit::SaxonRaider),
                        PreBattle::Evade => roll_evade.push(Unit::SaxonRaider),
                        PreBattle::None => harass.push(Unit::SaxonRaider),
                    }
                } else {
                    let selection = Select::new()
                        .with_prompt("What do Saxon Raiders choose?")
                        .items(&raider_prebattle)
                        .interact()
                        .unwrap();

                    match raider_prebattle[selection] {
                        PreBattle::Evade => roll_evade.push(Unit::SaxonRaider),
                        PreBattle::None => harass.push(Unit::SaxonRaider),
                        _ => {}
                    }
                }
            }
            if space.saxon_warband > 0 {
                if space.terrain == Terrain::Fens {
                    let selection = Select::new()
                        .with_prompt("What do Saxon Warbands choose?")
                        .items(&prebattle)
                        .interact()
                        .unwrap();

                    match prebattle[selection] {
                        PreBattle::Ambush => roll_ambush.push(Unit::SaxonWarband),
                        PreBattle::Evade => roll_evade.push(Unit::SaxonWarband),
                        PreBattle::None => melee.push(Unit::SaxonWarband),
                    }
                } else {
                    melee.push(Unit::SaxonWarband);
                }
            }
        }
        Faction::Scotti => {
            if space.scotti_raider > 0 {
                if space.terrain == Terrain::Hills {
                    let selection = Select::new()
                        .with_prompt("What do Scotti Raiders choose?")
                        .items(&prebattle)
                        .interact()
                        .unwrap();

                    match prebattle[selection] {
                        PreBattle::Ambush => roll_ambush.push(Unit::ScottiRaider),
                        PreBattle::Evade => roll_evade.push(Unit::ScottiRaider),
                        PreBattle::None => harass.push(Unit::ScottiRaider),
                    }
                } else {
                    let selection = Select::new()
                        .with_prompt("What do Scotti Raiders choose?")
                        .items(&raider_prebattle)
                        .interact()
                        .unwrap();

                    match raider_prebattle[selection] {
                        PreBattle::Evade => roll_evade.push(Unit::ScottiRaider),
                        PreBattle::None => harass.push(Unit::ScottiRaider),
                        _ => {}
                    }
                }
            }
            if space.scotti_warband > 0 {
                if space.terrain == Terrain::Hills {
                    let selection = Select::new()
                        .with_prompt("What do Scotti Warbands choose?")
                        .items(&prebattle)
                        .interact()
                        .unwrap();

                    match prebattle[selection] {
                        PreBattle::Ambush => roll_ambush.push(Unit::ScottiWarband),
                        PreBattle::Evade => roll_evade.push(Unit::ScottiWarband),
                        PreBattle::None => melee.push(Unit::ScottiWarband),
                    }
                } else {
                    melee.push(Unit::ScottiWarband);
                }
            }
        }
        Faction::None => {}
    };
    return (roll_ambush, roll_evade, charge_or_ambush, melee, harass);
}

fn prebattle_roll(
    ambush: Vec<Unit>,
    evade: Vec<Unit>,
    t: Terrain,
) -> (Vec<Unit>, Vec<Unit>, Vec<Unit>) {
    let mut charge_or_ambush = vec![];
    let mut melee = vec![];
    let mut harass = vec![];
    let mut rng = rand::rng();

    for u in ambush {
        let die_roll = rng.random_range(1..=6);
        if die_roll >= get_ambush(u.clone(), t).unwrap() {
            println!("{}: {} ambushed!", die_roll, u);
            charge_or_ambush.push(u);
        } else if u == Unit::SaxonRaider || u == Unit::ScottiRaider {
            println!("{}: {} failed to ambush.", die_roll, u);
            harass.push(u);
        } else {
            println!("{}: {} failed to ambush.", die_roll, u);
            melee.push(u)
        }
    }

    for u in evade {
        let die_roll = rng.random_range(1..=6);
        if die_roll < get_evade(u.clone(), t).unwrap() {
            if u == Unit::SaxonRaider || u == Unit::ScottiRaider {
                println!("{}: {} failed to evade.", die_roll, u);
                harass.push(u);
            } else {
                println!("{}: {} failed to evade.", die_roll, u);
                melee.push(u)
            }
        } else {
            println!("{}: {} evaded!", die_roll, u);
        }
    }

    return (charge_or_ambush, melee, harass);
}

fn battle(attacker: Faction, defender: Faction, space: Space) {
    /*
       Pre-Battle
    */

    let (att_roll_ambush, att_roll_evade, mut att_charge_or_ambush, mut att_melee, mut att_harass) =
        prebattle_assign(attacker, space.clone());
    let (def_roll_ambush, def_roll_evade, mut def_charge_or_ambush, mut def_melee, mut def_harass) =
        prebattle_assign(defender, space.clone());

    let (mut a0, mut a1, mut a2) = prebattle_roll(att_roll_ambush, att_roll_evade, space.terrain);
    let (mut d0, mut d1, mut d2) = prebattle_roll(def_roll_ambush, def_roll_evade, space.terrain);

    att_charge_or_ambush.append(&mut a0);
    def_charge_or_ambush.append(&mut d0);
    att_melee.append(&mut a1);
    def_melee.append(&mut d1);
    att_harass.append(&mut a2);
    def_harass.append(&mut d2);

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
