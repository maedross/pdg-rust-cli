use dialoguer::Select;
use rand::prelude::*;
use std::collections::VecDeque;
use std::fmt;

// State

enum Player {
    Civitates,
    Dux,
    Saxons,
    Scotti,
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

struct SequenceOfPlay {
    eligible_players: Vec<Player>,
    ineligible_players: Vec<Player>,
    passed_players: Vec<Player>,
    current_state: SequenceOfPlayState,
    action_chains: Vec<ActionChain>,
    selected_action_chain: usize,
}

enum SequenceOfPlayState {
    FirstAction,
    SecondAction,
    Cleanup,
    // TODO: may be unnecessary
    Epoch,
}

struct ActionChain {
    first: Actions,
    second: Actions,
}

enum Actions {
    Command,
    Feat,
    Event,
    LimitedCommand,
    Pass,
}

// Board

struct Space {
    name: String,
    space_type: SpaceType,
    terrain: Option<Terrain>,
    adj_spaces: Vec<&Space>,
    adj_seas: Vec<&Sea>,
    pop: u8,
    max_pop: u8,
    top_prosp: u8,
    bottom_prosp: u8,
    stronghold_sites: Vec<StrongholdSite>,
    control: Option<Player>,
}

enum SpaceType {
    Region,
    City,
}

struct Sea {
    name: String,
    patrol: bool,
    adj: Vec<Space>,
}

struct StrongholdSite {
    name: String,
    site_type: StrongholdSiteType,
    stronghold: Option<&Stronghold>,
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

/*
    Sequence of play

    FIRST just get turn taking with Command only -> Command only -> next round
        Passing
        Eligible factions
        First acted faction
        Second acted faction
        Ineligible factions
        THE LOOP

    SECOND Limited commands

    THIRD Command + Feat

    FOURTH events (Command XOR event)

    FIFTH the actual different options for first player (and how second is different)
*/

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

// Seq

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
}
