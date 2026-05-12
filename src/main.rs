use colored::Colorize;
use dialoguer::Select;
use std::collections::HashSet;
use std::fmt;
use std::vec;
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

enum Imperium {
    RomanRule(Dominance),
    Autonomy(Dominance),
    Fragmentation,
}

enum Dominance {
    Military,
    Civilian,
    None,
}

struct EdgeTrack {
    briton_resources: u8,
    wealth: u8,
    dux_resources: u8,
    prestige: u8,
    total_prosperity: u8,
    saxon_renown: u8,
    scotti_renown: u8,
    briton_control_threshold: u8,
    prosperity_plus_prestige_threshold: Option<u8>,
    control_plus_prestige_threshold: Option<u8>,
    saxon_control_threshold: u8,
    saxon_renown_threshold: Option<u8>,
    scotti_renown_threshold: u8,
}

// Board

struct Space<'a> {
    name: String,
    space_type: SpaceType,
    terrain: Option<Terrain>,
    adj_spaces: Vec<&'a Space<'a>>,
    adj_seas: Vec<&'a Sea<'a>>,
    pop: u8,
    max_pop: u8,
    top_prosp: u8,
    bottom_prosp: u8,
    stronghold_sites: Vec<StrongholdSite<'a>>,
    control: Option<Player>,
}

enum SpaceType {
    Region,
    City,
}

struct Sea<'a> {
    name: String,
    patrol: bool,
    adj: Vec<&'a Space<'a>>,
}

struct StrongholdSite<'a> {
    name: String,
    site_type: StrongholdSiteType,
    stronghold: Option<&'a Stronghold>,
}

enum StrongholdSiteType {
    Hillfort,
    Town,
    City,
}

struct City {}

struct Region {}

enum Terrain {
    Clear,
    Fens,
    Hills,
}

// Holding Boxes

struct CivitatesAvailable {
    militia: u8,
    comitates: u8,
    towns: u8,
    hillforts: u8,
    refugees: u8,
}

struct CivitatesOutOfPlay {
    comitates: u8,
}

struct ScottiAvailable {
    raiders: u8,
    warbands: u8,
    settlements: u8,
    max_settlements: u8,
}

struct NiallNoigiallach {
    raiders: u8,
}

struct SaxonsAvailable {
    raiders: u8,
    warbands: u8,
    settlements: u8,
    max_settlements: u8,
}

struct DuxAvailable {
    cavalry: u8,
    forts: u8,
}

struct DuxCasualties {
    cavalry: u8,
}

struct DuxOutOfPlay {
    cavalry: u8,
}

// Components

enum Nationality {
    Briton,
    Saxon,
    Scotti,
}

struct Stronghold {
    controller: Player,
    class: StrongholdClass,
    nationality: Nationality,
}

enum StrongholdClass {
    Fort {
        escalade: u8,
        garrison: u8,
        capacity: u8,
    },
    Hillfort {
        escalade: u8,
        garrison: u8,
        capacity: u8,
    },
    Town {
        escalade: u8,
        garrison: u8,
        capacity: u8,
    },
    Settlement {
        escalade: u8,
        garrison: u8,
        capacity: u8,
    },
    Eboracum {
        escalade: u8,
        garrison: u8,
        capacity: u8,
    },
    Londinium {
        escalade: u8,
        garrison: u8,
        capacity: u8,
    },
}

struct Unit {
    designation: UnitClass,
    controller: Player,
    nationality: Nationality,
    plunder: bool,
}

enum UnitClass {
    Cavalry,
    Comitates,
    Foederati,
    Militia,
    Raider,
    Warband,
}

// Cards
struct Event {
    eligibility: Vec<Player>,
    unshaded: Option<u8>,
    shaded: Option<u8>,
    historical_notes: String,
}

struct CardManager<'a> {
    deck: &'a mut Vec<Event>,
    current_event: &'a Event,
    upcoming_event: &'a Event,
    discard: &'a mut Vec<Event>,
}

// Sequence of Play

struct SequenceOfPlay {
    eligible_players: HashSet<Player>,
    ineligible_players: HashSet<Player>,
    passed_players: HashSet<Player>,
    first_acted_player: Option<Player>,
    second_acted_player: Option<Player>,
    current_state: SequenceOfPlayState,
    action_chains: Vec<ActionChain>,
    selected_action_chain: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum SequenceOfPlayState {
    FirstAction,
    SecondAction,
    Cleanup,
    // TODO: may be unnecessary
    Epoch,
}

struct ActionChain {
    first: Action,
    second: Action,
}

#[derive(Debug)]
enum Action {
    Command,
    Feat,
    Event,
    LimitedCommand,
    Pass,
}

enum SequenceOfPlayBox<'a> {
    EligibleBox {
        factions: HashSet<Player>,
        points_to: Vec<&'a SequenceOfPlayBox<'a>>,
    },
    ActionBox {
        actions: Vec<Action>,
        occupied: Option<Player>,
        points_to: Vec<&'a SequenceOfPlayBox<'a>>,
        sends_to: &'a SequenceOfPlayBox<'a>,
    },
    PassBox {
        factions: HashSet<Player>,
        sends_to: &'a SequenceOfPlayBox<'a>
    },
    IneligibleBox {
        factions: HashSet<Player>,
        sends_to: &'a SequenceOfPlayBox<'a>,
    }
}

impl fmt::Display for SequenceOfPlay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Current state: {:?}\nEligible factions: {:#?}\nIneligible factions: {:#?}",
            self.current_state, self.eligible_players, self.ineligible_players
        )
    }
}

impl SequenceOfPlay {
    fn run_sequence_of_play(&mut self, card_view: &CardManager) {
        self.current_state = SequenceOfPlayState::FirstAction;
        println!("{}", self);
        // Query pivotal
        let mut curr: usize = 0;
        let mut second_actions: Vec<&str> = vec![];
        while self.current_state == SequenceOfPlayState::FirstAction && curr < 4 {
            let player: Player = card_view.current_event.eligibility[curr].clone();
            curr += 1;
            if self.eligible_players.contains(&player) {
                self.eligible_players.remove(&player);
                let options: Vec<&str> = vec!["Command", "Command + Feat", "Event", "Pass"];
                let selection = Select::new()
                    .with_prompt(format!("{} select one of the following actions!", player))
                    .items(&options)
                    .interact()
                    .unwrap();
                println!("{:?} chose: {}", player, options[selection]);
                if options[selection] != "Pass" {
                    self.current_state = SequenceOfPlayState::SecondAction;
                    self.first_acted_player = Some(player);
                    match selection {
                        0 => second_actions.push("LimitedCommand"),
                        1 => {
                            second_actions.push("Limited Command");
                            second_actions.push("Event");
                        }
                        2 => second_actions.push("Command + Feat"),
                        _ => println!("Impossible!"),
                    }
                    second_actions.push("Pass");
                } else {
                    self.passed_players.insert(player);
                    self.eligible_players.remove(&player);
                }
            }
        }

        while self.current_state == SequenceOfPlayState::SecondAction && curr < 4 {
            let player: Player = card_view.current_event.eligibility[curr].clone();
            curr += 1;
            if self.eligible_players.contains(&player) {
                self.eligible_players.remove(&player);
                let selection = Select::new()
                    .with_prompt(format!("{} select one of the following actions!", player))
                    .items(&second_actions)
                    .interact()
                    .unwrap();
                println!("{:?} chose: {}", player, second_actions[selection]);
                if second_actions[selection] != "Pass" {
                    self.current_state = SequenceOfPlayState::Cleanup;
                    self.second_acted_player = Some(player);
                } else {
                    self.passed_players.insert(player);
                    self.eligible_players.remove(&player);
                }
            }
        }

        println!("Cleaning up...");
        println!(
            "Shifting {:#?} from passed to eligible...",
            self.passed_players
        );
        self.eligible_players.extend(self.passed_players.drain());

        println!(
            "Shifting {:#?} from ineligible to eligible...",
            self.ineligible_players
        );
        self.eligible_players
            .extend(self.ineligible_players.drain());

        match self.first_acted_player {
            Some(player) => {
                self.first_acted_player = None;
                self.ineligible_players.insert(player);
                println!("Setting {} to ineligible...", player);
            }
            None => {}
        }
        match self.second_acted_player {
            Some(player) => {
                self.second_acted_player = None;
                self.ineligible_players.insert(player);
                println!("Setting {} to ineligible...", player);
            }
            None => {}
        }
        // Current consideration is first in card order
        // While first acted is none and current consideration is not none
        // Query action
        // If pass, assign and continue
        // Else assign
        // While second acted is none and current
    }
}

// Commands

/*
    Any Command or Feat will go
    1. Get possible target spaces
    2. Select and pay for spaces
    3. Do the thing
*/

fn civitates_muster() {
    /*
       1. Filter spaces with units or strongholds controlled by Civitates
       2. Select spaces, costing 2 per (immediate)
       3. For each space
           1. Place troops
               to place = 1 per stronghold
               if Briton control
                   to place += pop
                pay wealth to place comitates instead?
           2. Place stronghold
    */
}

fn civitates_march() {}

fn civitates_trade() {}

fn civitates_battle() {}

fn dux_train() {
    /*
       1. Place troops where Fort (cost 3)
           Place cavalry
           May place militia if Civitates stronghold
       2. Add prosperity where Fort or friendly control (cost 2)
           Add prosperity

    */
}

fn dux_march() {
    /*
        My Sisyphean task

        FIRST OF ALL
        negotiating with Civitates

        select and pay for origins
        then move to destinations
            affected by roads
        BUT ALSO
            you can pick up allied pieces on the way that weren't in your
            initial origin location, and you have to pay for those and mark them as new origins

        Due to Road rules, some destinations cannot be waypoints
        You can't move off the road and then back on (both for spaces without roads and spaces
        with enemy control)

        FOR origin
            GET possible destinations (with routes)
            SELECT destinations
                FOR destinations
                    SELECT pieces
                    QUERY pickup?
                        DISPLAY route spaces

    */
}

fn dux_intercept() {}

fn dux_battle() {}

fn saxon_raid() {}

fn saxon_return() {}

fn saxon_march() {}

fn saxon_battle() {}

fn scotti_raid() {}

fn scotti_return() {}

fn scotti_march() {}

fn scotti_battle() {}

// Feats

fn civitates_rule() {}

fn civitates_invite() {}

fn civitates_reinforce() {}

fn civitates_pillage() {}

fn dux_build() {}

fn dux_invite() {}

fn dux_requisition() {}

fn dux_retaliate() {}

fn saxon_settle() {}

fn saxon_surprise() {}

fn saxon_ravage() {}

fn saxon_shieldwall() {}

fn scotti_settle() {}

fn scotti_surprise() {}

fn scotti_ransom() {}

fn scotti_entreat() {}

/*
    Events?
*/

/*
    Epoch round
*/

/*
    Victory
*/

/*
    Bots
*/

// Helper functions

fn move_units() {}

fn battle() {}

fn get_spaces() {}

struct GameMap {}

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
    println!("{:?} {} {} {}", Player::Civitates, Player::Dux, Player::Saxons, Player::Scotti);
    let mut sop: SequenceOfPlay = SequenceOfPlay {
        eligible_players: HashSet::from([
            Player::Civitates,
            Player::Dux,
            Player::Saxons,
            Player::Scotti,
        ]),
        ineligible_players: HashSet::from([]),
        passed_players: HashSet::from([]),
        first_acted_player: None,
        second_acted_player: None,
        current_state: SequenceOfPlayState::FirstAction,
        action_chains: vec![],
        selected_action_chain: 0,
    };
    let mut test_card_0: Event = Event {
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
    let mut test_card_1: Event = Event {
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
    let mut test_card_view: CardManager = CardManager {
        deck: &mut vec![],
        current_event: &test_card_0,
        upcoming_event: &test_card_1,
        discard: &mut vec![],
    };
    sop.run_sequence_of_play(&test_card_view);
    test_card_view.current_event = test_card_view.upcoming_event;
    sop.run_sequence_of_play(&test_card_view);
}
