use std::collections::HashMap;

use super::concepts::{Player, Stronghold, Unit};

//TODO: Add Catelonia
pub struct Board<'a> {
    map: HashMap<String, Space<'a>>,
    edge_track: EdgeTrack,
    civitates_available: CivitatesAvailable,
    civitates_not_yet_in_play: CivitatesNotYetInPlay,
    dux_available: DuxAvailable,
    dux_casualties: DuxCasualties,
    dux_out_of_play: DuxOutOfPlay,
    saxon_available: SaxonsAvailable,
    scotti_available: ScottiAvailable,
    scotti_niall_noigiallach: ScottiNiallNoigiallach,
    imperium: Imperium,
    roads_maintained: bool,
}

#[derive(Clone, Debug)]
pub enum Terrain {
    Clear,
    Fens,
    Hills,
}

#[derive(Clone, Debug)]
pub struct Space<'a> {
    pub name: String,
    pub space_type: SpaceType,
    pub terrain: Option<Terrain>,
    pub adj_spaces: Vec<&'a Space<'a>>,
    pub adj_seas: Vec<&'a Sea<'a>>,
    pub pop: u8,
    pub max_pop: u8,
    pub top_prosp: u8,
    pub bottom_prosp: u8,
    pub stronghold_sites: Vec<StrongholdSite<'a>>,
    pub units: Vec<Unit>,
    pub control: Option<Player>,
}

impl<'a> Space<'_> {
    fn new(
        name: &str,
        space_type: SpaceType,
        terrain: Option<Terrain>,
        pop: u8,
        stronghold_sites: Vec<StrongholdSite<'a>>,
    ) -> Space<'a> {
        Space {
            name: name.to_string(),
            space_type,
            terrain,
            adj_spaces: vec![],
            adj_seas: vec![],
            pop,
            max_pop: pop + 1,
            top_prosp: 0,
            bottom_prosp: 0,
            stronghold_sites,
            units: vec![],
            control: None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum SpaceType {
    Region,
    City,
}

#[derive(Clone, Debug)]
pub enum StrongholdSiteType {
    Hillfort,
    Town,
    City,
}

#[derive(Clone, Debug)]
pub struct StrongholdSite<'a> {
    pub name: String,
    pub site_type: StrongholdSiteType,
    pub stronghold: Option<&'a Stronghold>,
}

impl<'a> StrongholdSite<'_> {
    fn new(name: &str, site_type: StrongholdSiteType) -> StrongholdSite<'a> {
        StrongholdSite {
            name: name.to_string(),
            site_type,
            stronghold: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Sea<'a> {
    name: String,
    patrol: bool,
    adj: Vec<&'a Space<'a>>,
}

impl<'a> Sea<'_> {
    fn new(name: &str) -> Sea<'a> {
        Sea {
            name: name.to_string(),
            patrol: false,
            adj: vec![],
        }
    }
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
struct CivitatesAvailable {
    militia: u8,
    comitates: u8,
    towns: u8,
    hillforts: u8,
    refugees: u8,
}

struct CivitatesNotYetInPlay {
    comitates: u8,
}

struct ScottiAvailable {
    raiders: u8,
    warbands: u8,
    settlements: u8,
    max_settlements: u8,
}

struct ScottiNiallNoigiallach {
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

fn build_map() -> HashMap<String, Space<'static>> {
    let mut map = HashMap::new();

    let mut dumnonii: Space<'_> = Space::new(
        "Dumnonii",
        SpaceType::Region,
        Some(Terrain::Hills),
        1,
        vec![
            StrongholdSite::new("Tintagel", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Isca Dumnoniorum", StrongholdSiteType::Town),
        ],
    );
    map.insert(dumnonii.name.clone(), dumnonii);

    let mut durotriges: Space<'_> = Space::new(
        "Durotriges",
        SpaceType::Region,
        Some(Terrain::Clear),
        2,
        vec![
            StrongholdSite::new("South Cadbury", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Aquae Sulis", StrongholdSiteType::Town),
        ],
    );
    map.insert(durotriges.name.clone(), durotriges);

    let mut atrebates: Space<'_> = Space::new(
        "Atrebates",
        SpaceType::Region,
        Some(Terrain::Clear),
        3,
        vec![
            StrongholdSite::new("Venta Belgarum", StrongholdSiteType::Town),
            StrongholdSite::new("Calleva Atrebatum", StrongholdSiteType::Town),
        ],
    );
    map.insert(atrebates.name.clone(), atrebates);

    let mut regni: Space<'_> = Space::new(
        "Regni",
        SpaceType::Region,
        Some(Terrain::Clear),
        2,
        vec![
            StrongholdSite::new("Anderida", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Noviomagus", StrongholdSiteType::Town),
        ],
    );
    map.insert(regni.name.clone(), regni);

    let mut cantiaci: Space<'_> = Space::new(
        "Cantiaci",
        SpaceType::Region,
        Some(Terrain::Fens),
        2,
        vec![
            StrongholdSite::new("Rutupiae", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Durovernum", StrongholdSiteType::Town),
        ],
    );
    map.insert(cantiaci.name.clone(), cantiaci);

    let mut londinium: Space<'_> = Space::new(
        "Londinium",
        SpaceType::City,
        None,
        2,
        vec![StrongholdSite::new("Londinium", StrongholdSiteType::Town)],
    );
    map.insert(londinium.name.clone(), londinium);

    let mut catuvellauni: Space<'_> = Space::new(
        "Catuvellauni",
        SpaceType::Region,
        Some(Terrain::Clear),
        3,
        vec![
            StrongholdSite::new("Durocobrivis", StrongholdSiteType::Town),
            StrongholdSite::new("Verulamium", StrongholdSiteType::Town),
        ],
    );
    map.insert(catuvellauni.name.clone(), catuvellauni);

    let mut dobunni: Space<'_> = Space::new(
        "Dobunni",
        SpaceType::Region,
        Some(Terrain::Clear),
        2,
        vec![
            StrongholdSite::new("Glevum", StrongholdSiteType::Town),
            StrongholdSite::new("Corinium", StrongholdSiteType::Town),
        ],
    );
    map.insert(dobunni.name.clone(), dobunni);

    let mut cornovii: Space<'_> = Space::new(
        "Cornovii",
        SpaceType::Region,
        Some(Terrain::Clear),
        2,
        vec![
            StrongholdSite::new("Old Oswestry", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Viroconium", StrongholdSiteType::Town),
        ],
    );
    map.insert(cornovii.name.clone(), cornovii);

    let mut decangli: Space<'_> = Space::new(
        "Decangli",
        SpaceType::Region,
        Some(Terrain::Clear),
        2,
        vec![
            StrongholdSite::new("Dinorben", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Deva", StrongholdSiteType::Town),
        ],
    );
    map.insert(decangli.name.clone(), decangli);

    let mut iceni: Space<'_> = Space::new(
        "Iceni",
        SpaceType::Region,
        Some(Terrain::Fens),
        2,
        vec![
            StrongholdSite::new("Branodunum", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Venta Icenorum", StrongholdSiteType::Hillfort),
        ],
    );
    map.insert(iceni.name.clone(), iceni);

    let mut corieltauvi: Space<'_> = Space::new(
        "Corieltauvi",
        SpaceType::Region,
        Some(Terrain::Fens),
        2,
        vec![
            StrongholdSite::new("Ratae", StrongholdSiteType::Town),
            StrongholdSite::new("Lindum", StrongholdSiteType::Town),
        ],
    );
    map.insert(corieltauvi.name.clone(), corieltauvi);

    let mut silures: Space<'_> = Space::new(
        "Silures",
        SpaceType::Region,
        Some(Terrain::Hills),
        1,
        vec![
            StrongholdSite::new("Dinas Powys", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Isca Silurum", StrongholdSiteType::Town),
        ],
    );
    map.insert(silures.name.clone(), silures);

    let mut demetae: Space<'_> = Space::new(
        "Demetae",
        SpaceType::Region,
        Some(Terrain::Hills),
        1,
        vec![
            StrongholdSite::new("Moridunum", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Gateholm", StrongholdSiteType::Hillfort),
        ],
    );
    map.insert(demetae.name.clone(), demetae);

    let mut ordovices: Space<'_> = Space::new(
        "Ordovices",
        SpaceType::Region,
        Some(Terrain::Hills),
        1,
        vec![
            StrongholdSite::new("Segontium", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Dinas Emrys", StrongholdSiteType::Hillfort),
        ],
    );
    map.insert(ordovices.name.clone(), ordovices);

    let mut brigantes: Space<'_> = Space::new(
        "Brigantes",
        SpaceType::Region,
        Some(Terrain::Hills),
        1,
        vec![
            StrongholdSite::new("Barwick", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Mam Tor", StrongholdSiteType::Hillfort),
        ],
    );
    map.insert(brigantes.name.clone(), brigantes);

    let mut eboracum: Space<'_> = Space::new(
        "Eboracum",
        SpaceType::City,
        None,
        1,
        vec![StrongholdSite::new("Eboracum", StrongholdSiteType::Town)],
    );
    map.insert(eboracum.name.clone(), eboracum);

    let mut textoverdi: Space<'_> = Space::new(
        "Textoverdi",
        SpaceType::Region,
        Some(Terrain::Hills),
        2,
        vec![
            StrongholdSite::new("Pons Aelius", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Cataractonium", StrongholdSiteType::Town),
        ],
    );
    map.insert(textoverdi.name.clone(), textoverdi);

    let mut carvetii: Space<'_> = Space::new(
        "Carvetii",
        SpaceType::Region,
        Some(Terrain::Hills),
        1,
        vec![
            StrongholdSite::new("Uxellodunum", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Luguvalium", StrongholdSiteType::Town),
        ],
    );
    map.insert(carvetii.name.clone(), carvetii);

    let mut votadini: Space<'_> = Space::new(
        "Carvetii",
        SpaceType::Region,
        Some(Terrain::Hills),
        2,
        vec![
            StrongholdSite::new("Yeavering", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Taprain Law", StrongholdSiteType::Hillfort),
        ],
    );
    map.insert(votadini.name.clone(), votadini);

    let mut novantae: Space<'_> = Space::new(
        "Novantae",
        SpaceType::Region,
        Some(Terrain::Hills),
        1,
        vec![
            StrongholdSite::new("Mote of Mark", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Alclud", StrongholdSiteType::Hillfort),
        ],
    );
    map.insert(novantae.name.clone(), novantae);

    
    return map;
}
