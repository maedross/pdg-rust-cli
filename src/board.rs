use std::collections::HashMap;

use super::concepts::{Player, Stronghold, Unit};

pub struct Board<'a> {
    map: Map<'a>,
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

pub struct Map<'a> {
    land: HashMap<u8, Space<'a>>,
    off_map_land: HashMap<u8, OffMapLand>,
    seas: HashMap<u8, Sea>,
}

#[derive(Clone, Debug)]
pub enum Terrain {
    Clear,
    Fens,
    Hills,
}

#[derive(Clone, Debug)]
pub struct Space<'a> {
    pub id: u8,
    pub name: String,
    pub space_type: SpaceType,
    pub terrain: Option<Terrain>,
    pub adj_spaces: Vec<u8>,
    pub adj_seas: Vec<u8>,
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
        id: u8,
        name: &str,
        space_type: SpaceType,
        terrain: Option<Terrain>,
        pop: u8,
        stronghold_sites: Vec<StrongholdSite<'a>>,
    ) -> Space<'a> {
        Space {
            id,
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
pub struct OffMapLand {
    id: u8,
    name: String,
    patrol_spaces: Vec<u8>,
    adj: Vec<u8>,
}

impl OffMapLand {
    fn new(id: u8, name: &str) -> OffMapLand {
        OffMapLand {
            id,
            name: name.to_string(),
            patrol_spaces: vec![],
            adj: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Sea {
    id: u8,
    name: String,
    patrol: bool,
    adj: Vec<u8>,
}

impl Sea {
    fn new(id: u8, name: &str) -> Sea {
        Sea {
            id,
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

fn build_map() -> Map<'static> {
    let mut land = HashMap::new();
    let mut off_map_land = HashMap::new();
    let mut seas = HashMap::new();

    // CREATE SPACES
    let mut atrebates: Space<'_> = Space::new(
        0,
        "Atrebates",
        SpaceType::Region,
        Some(Terrain::Clear),
        3,
        vec![
            StrongholdSite::new("Venta Belgarum", StrongholdSiteType::Town),
            StrongholdSite::new("Calleva Atrebatum", StrongholdSiteType::Town),
        ],
    );
    let mut brigantes: Space<'_> = Space::new(
        1,
        "Brigantes",
        SpaceType::Region,
        Some(Terrain::Hills),
        1,
        vec![
            StrongholdSite::new("Barwick", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Mam Tor", StrongholdSiteType::Hillfort),
        ],
    );
    let mut cantiaci: Space<'_> = Space::new(
        2,
        "Cantiaci",
        SpaceType::Region,
        Some(Terrain::Fens),
        2,
        vec![
            StrongholdSite::new("Rutupiae", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Durovernum", StrongholdSiteType::Town),
        ],
    );
    let mut carvetii: Space<'_> = Space::new(
        3,
        "Carvetii",
        SpaceType::Region,
        Some(Terrain::Hills),
        1,
        vec![
            StrongholdSite::new("Uxellodunum", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Luguvalium", StrongholdSiteType::Town),
        ],
    );
    let mut catuvellauni: Space<'_> = Space::new(
        4,
        "Catuvellauni",
        SpaceType::Region,
        Some(Terrain::Clear),
        3,
        vec![
            StrongholdSite::new("Durocobrivis", StrongholdSiteType::Town),
            StrongholdSite::new("Verulamium", StrongholdSiteType::Town),
        ],
    );
    let mut corieltauvi: Space<'_> = Space::new(
        5,
        "Corieltauvi",
        SpaceType::Region,
        Some(Terrain::Fens),
        2,
        vec![
            StrongholdSite::new("Ratae", StrongholdSiteType::Town),
            StrongholdSite::new("Lindum", StrongholdSiteType::Town),
        ],
    );
    let mut cornovii: Space<'_> = Space::new(
        6,
        "Cornovii",
        SpaceType::Region,
        Some(Terrain::Clear),
        2,
        vec![
            StrongholdSite::new("Old Oswestry", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Viroconium", StrongholdSiteType::Town),
        ],
    );
    let mut decangli: Space<'_> = Space::new(
        7,
        "Decangli",
        SpaceType::Region,
        Some(Terrain::Clear),
        2,
        vec![
            StrongholdSite::new("Dinorben", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Deva", StrongholdSiteType::Town),
        ],
    );
    let mut demetae: Space<'_> = Space::new(
        8,
        "Demetae",
        SpaceType::Region,
        Some(Terrain::Hills),
        1,
        vec![
            StrongholdSite::new("Moridunum", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Gateholm", StrongholdSiteType::Hillfort),
        ],
    );
    let mut dobunni: Space<'_> = Space::new(
        9,
        "Dobunni",
        SpaceType::Region,
        Some(Terrain::Clear),
        2,
        vec![
            StrongholdSite::new("Glevum", StrongholdSiteType::Town),
            StrongholdSite::new("Corinium", StrongholdSiteType::Town),
        ],
    );
    let mut dumnonii: Space<'_> = Space::new(
        10,
        "Dumnonii",
        SpaceType::Region,
        Some(Terrain::Hills),
        1,
        vec![
            StrongholdSite::new("Tintagel", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Isca Dumnoniorum", StrongholdSiteType::Town),
        ],
    );
    let mut durotriges: Space<'_> = Space::new(
        11,
        "Durotriges",
        SpaceType::Region,
        Some(Terrain::Clear),
        2,
        vec![
            StrongholdSite::new("South Cadbury", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Aquae Sulis", StrongholdSiteType::Town),
        ],
    );
    let mut eboracum: Space<'_> = Space::new(
        12,
        "Eboracum",
        SpaceType::City,
        None,
        1,
        vec![StrongholdSite::new("Eboracum", StrongholdSiteType::Town)],
    );
    let mut iceni: Space<'_> = Space::new(
        13,
        "Iceni",
        SpaceType::Region,
        Some(Terrain::Fens),
        2,
        vec![
            StrongholdSite::new("Branodunum", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Venta Icenorum", StrongholdSiteType::Hillfort),
        ],
    );
    let mut londinium: Space<'_> = Space::new(
        14,
        "Londinium",
        SpaceType::City,
        None,
        2,
        vec![StrongholdSite::new("Londinium", StrongholdSiteType::Town)],
    );
    let mut novantae: Space<'_> = Space::new(
        15,
        "Novantae",
        SpaceType::Region,
        Some(Terrain::Hills),
        1,
        vec![
            StrongholdSite::new("Mote of Mark", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Alclud", StrongholdSiteType::Hillfort),
        ],
    );
    let mut ordovices: Space<'_> = Space::new(
        16,
        "Ordovices",
        SpaceType::Region,
        Some(Terrain::Hills),
        1,
        vec![
            StrongholdSite::new("Segontium", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Dinas Emrys", StrongholdSiteType::Hillfort),
        ],
    );
    let mut parisi: Space<'_> = Space::new(
        17,
        "Parisi",
        SpaceType::Region,
        Some(Terrain::Clear),
        2,
        vec![
            StrongholdSite::new("Petuaria", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Derventio", StrongholdSiteType::Hillfort),
        ],
    );
    let mut regni: Space<'_> = Space::new(
        18,
        "Regni",
        SpaceType::Region,
        Some(Terrain::Clear),
        2,
        vec![
            StrongholdSite::new("Anderida", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Noviomagus", StrongholdSiteType::Town),
        ],
    );
    let mut silures: Space<'_> = Space::new(
        19,
        "Silures",
        SpaceType::Region,
        Some(Terrain::Hills),
        1,
        vec![
            StrongholdSite::new("Dinas Powys", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Isca Silurum", StrongholdSiteType::Town),
        ],
    );
    let mut trinovantes: Space<'_> = Space::new(
        20,
        "Trinovantes",
        SpaceType::Region,
        Some(Terrain::Fens),
        2,
        vec![
            StrongholdSite::new("Camolodunum", StrongholdSiteType::Town),
            StrongholdSite::new("Walton Castle", StrongholdSiteType::Hillfort),
        ],
    );
    let mut textoverdi: Space<'_> = Space::new(
        21,
        "Textoverdi",
        SpaceType::Region,
        Some(Terrain::Hills),
        2,
        vec![
            StrongholdSite::new("Pons Aelius", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Cataractonium", StrongholdSiteType::Town),
        ],
    );
    let mut votadini: Space<'_> = Space::new(
        22,
        "Carvetii",
        SpaceType::Region,
        Some(Terrain::Hills),
        2,
        vec![
            StrongholdSite::new("Yeavering", StrongholdSiteType::Hillfort),
            StrongholdSite::new("Taprain Law", StrongholdSiteType::Hillfort),
        ],
    );

    // CREATE CALEDONIA AND SEAS
    let mut oceanus_britannicus: Sea = Sea::new(23, "Oceanus Britannicus");
    let mut oceanus_germanicus: Sea = Sea::new(24, "Oceanus Germanicus");
    let mut oceanus_hibernicus: Sea = Sea::new(25, "Oceanus Hibernicus");
    let mut oceanus_septentrionalis: Sea = Sea::new(26, "Oceanus Septentrionalis");

    let mut caledonia: OffMapLand = OffMapLand::new(27, "Caledonia");
    caledonia.patrol_spaces = vec![carvetii.id, textoverdi.id];

    // ADD ADJACENT SPACES
    atrebates.adj_spaces = vec![
        dobunni.id,
        catuvellauni.id,
        londinium.id,
        cantiaci.id,
        regni.id,
        durotriges.id,
    ];
    brigantes.adj_spaces = vec![
        carvetii.id,
        textoverdi.id,
        eboracum.id,
        parisi.id,
        corieltauvi.id,
        cornovii.id,
        decangli.id,
    ];
    cantiaci.adj_spaces = vec![londinium.id, regni.id, atrebates.id];
    carvetii.adj_spaces = vec![
        novantae.id,
        votadini.id,
        textoverdi.id,
        brigantes.id,
        decangli.id,
    ];
    catuvellauni.adj_spaces = vec![
        corieltauvi.id,
        iceni.id,
        trinovantes.id,
        londinium.id,
        atrebates.id,
        dobunni.id,
    ];
    corieltauvi.adj_spaces = vec![
        parisi.id,
        iceni.id,
        catuvellauni.id,
        dobunni.id,
        cornovii.id,
        brigantes.id,
    ];
    cornovii.adj_spaces = vec![
        decangli.id,
        brigantes.id,
        corieltauvi.id,
        dobunni.id,
        silures.id,
        demetae.id,
        ordovices.id,
    ];
    decangli.adj_spaces = vec![carvetii.id, brigantes.id, cornovii.id, ordovices.id];
    demetae.adj_spaces = vec![ordovices.id, cornovii.id, silures.id];
    dobunni.adj_spaces = vec![
        silures.id,
        cornovii.id,
        corieltauvi.id,
        catuvellauni.id,
        atrebates.id,
        durotriges.id,
    ];
    dumnonii.adj_spaces = vec![durotriges.id];
    durotriges.adj_spaces = vec![dobunni.id, atrebates.id, dumnonii.id];
    eboracum.adj_spaces = vec![textoverdi.id, parisi.id, brigantes.id];
    iceni.adj_spaces = vec![corieltauvi.id, trinovantes.id, catuvellauni.id];
    londinium.adj_spaces = vec![catuvellauni.id, trinovantes.id, cantiaci.id, atrebates.id];
    novantae.adj_spaces = vec![votadini.id, carvetii.id];
    ordovices.adj_spaces = vec![decangli.id, cornovii.id, demetae.id];
    parisi.adj_spaces = vec![textoverdi.id, eboracum.id, corieltauvi.id, brigantes.id];
    regni.adj_spaces = vec![atrebates.id, cantiaci.id];
    silures.adj_spaces = vec![demetae.id, cornovii.id, dobunni.id];
    textoverdi.adj_spaces = vec![
        carvetii.id,
        votadini.id,
        parisi.id,
        eboracum.id,
        brigantes.id,
    ];
    trinovantes.adj_spaces = vec![iceni.id, londinium.id, catuvellauni.id];
    votadini.adj_spaces = vec![novantae.id, carvetii.id, textoverdi.id];

    // ADD ADJACENT SEAS
    atrebates.adj_seas = vec![oceanus_britannicus.id];
    cantiaci.adj_seas = vec![oceanus_britannicus.id, oceanus_germanicus.id];
    carvetii.adj_seas = vec![oceanus_hibernicus.id];
    corieltauvi.adj_seas = vec![oceanus_germanicus.id];
    decangli.adj_seas = vec![oceanus_hibernicus.id];
    demetae.adj_seas = vec![oceanus_hibernicus.id];
    dumnonii.adj_seas = vec![oceanus_britannicus.id];
    durotriges.adj_seas = vec![oceanus_germanicus.id, oceanus_hibernicus.id];
    iceni.adj_seas = vec![oceanus_germanicus.id];
    londinium.adj_seas = vec![oceanus_germanicus.id];
    novantae.adj_seas = vec![oceanus_hibernicus.id];
    ordovices.adj_seas = vec![oceanus_hibernicus.id];
    parisi.adj_seas = vec![oceanus_germanicus.id, oceanus_septentrionalis.id];
    regni.adj_seas = vec![oceanus_britannicus.id];
    silures.adj_seas = vec![oceanus_hibernicus.id];
    textoverdi.adj_seas = vec![oceanus_septentrionalis.id];
    trinovantes.adj_seas = vec![oceanus_germanicus.id];
    votadini.adj_seas = vec![oceanus_septentrionalis.id];

    oceanus_britannicus.adj = vec![
        cantiaci.id,
        regni.id,
        atrebates.id,
        durotriges.id,
        dumnonii.id,
    ];
    oceanus_germanicus.adj = vec![
        parisi.id,
        corieltauvi.id,
        iceni.id,
        trinovantes.id,
        londinium.id,
        cantiaci.id,
    ];
    oceanus_hibernicus.adj = vec![
        novantae.id,
        carvetii.id,
        decangli.id,
        ordovices.id,
        demetae.id,
        silures.id,
        durotriges.id,
        dumnonii.id,
    ];
    oceanus_septentrionalis.adj = vec![votadini.id, textoverdi.id, parisi.id];
    caledonia.adj = vec![novantae.id, votadini.id];

    // SAVE SPACE TO MAP
    land.insert(atrebates.id, atrebates);
    land.insert(brigantes.id, brigantes);
    land.insert(catuvellauni.id, catuvellauni);
    land.insert(cantiaci.id, cantiaci);
    land.insert(carvetii.id, carvetii);
    land.insert(corieltauvi.id, corieltauvi);
    land.insert(cornovii.id, cornovii);
    land.insert(decangli.id, decangli);
    land.insert(demetae.id, demetae);
    land.insert(dobunni.id, dobunni);
    land.insert(dumnonii.id, dumnonii);
    land.insert(durotriges.id, durotriges);
    land.insert(eboracum.id, eboracum);
    land.insert(iceni.id, iceni);
    land.insert(londinium.id, londinium);
    land.insert(novantae.id, novantae);
    land.insert(ordovices.id, ordovices);
    land.insert(regni.id, regni);
    land.insert(silures.id, silures);
    land.insert(textoverdi.id, textoverdi);
    land.insert(trinovantes.id, trinovantes);
    land.insert(votadini.id, votadini);

    off_map_land.insert(caledonia.id, caledonia);

    seas.insert(oceanus_britannicus.id, oceanus_britannicus);
    seas.insert(oceanus_germanicus.id, oceanus_germanicus);
    seas.insert(oceanus_hibernicus.id, oceanus_hibernicus);
    seas.insert(oceanus_septentrionalis.id, oceanus_septentrionalis);

    return Map {
        land,
        off_map_land,
        seas,
    };
}

pub fn setup_barbarian_conspiracy() -> Board<'static> {
    Board {
        map: todo!(),
        edge_track: todo!(),
        civitates_available: todo!(),
        civitates_not_yet_in_play: todo!(),
        dux_available: todo!(),
        dux_casualties: todo!(),
        dux_out_of_play: todo!(),
        saxon_available: todo!(),
        scotti_available: todo!(),
        scotti_niall_noigiallach: todo!(),
        imperium: todo!(),
        roads_maintained: todo!(),
    }
}

pub fn setup_etty_tyrants() -> Board<'static> {
    Board {
        map: todo!(),
        edge_track: todo!(),
        civitates_available: todo!(),
        civitates_not_yet_in_play: todo!(),
        dux_available: todo!(),
        dux_casualties: todo!(),
        dux_out_of_play: todo!(),
        saxon_available: todo!(),
        scotti_available: todo!(),
        scotti_niall_noigiallach: todo!(),
        imperium: todo!(),
        roads_maintained: todo!(),
    }
}

pub fn setup_the_harder_they_fall() -> Board<'static> {
    Board {
        map: todo!(),
        edge_track: todo!(),
        civitates_available: todo!(),
        civitates_not_yet_in_play: todo!(),
        dux_available: todo!(),
        dux_casualties: todo!(),
        dux_out_of_play: todo!(),
        saxon_available: todo!(),
        scotti_available: todo!(),
        scotti_niall_noigiallach: todo!(),
        imperium: todo!(),
        roads_maintained: todo!(),
    }
}

pub fn setup_de_excidio_britanniae() -> Board<'static> {
    Board {
        map: todo!(),
        edge_track: todo!(),
        civitates_available: todo!(),
        civitates_not_yet_in_play: todo!(),
        dux_available: todo!(),
        dux_casualties: todo!(),
        dux_out_of_play: todo!(),
        saxon_available: todo!(),
        scotti_available: todo!(),
        scotti_niall_noigiallach: todo!(),
        imperium: todo!(),
        roads_maintained: todo!(),
    }
}
