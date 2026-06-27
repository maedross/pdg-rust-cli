use super::board::{Space, StrongholdSite, StrongholdSiteType};
use super::concepts::{CivitatesHolding, Player, StrongholdClass, Unit};
use dialoguer::Input;

// TODO: func for selecting spaces
// TODO: Muster (and other commands) as state machine?
// States would be selecting spaces and spending, type of muster
// But need to allow for feats as well
pub fn muster(loc: Space, wealth: u8, avail: CivitatesHolding) -> (Space, u8) {
    let resulting_loc: Space;
    let w: u8;
    if true {
        (resulting_loc, w) = muster_units(loc, wealth, avail);
    } else {
        (resulting_loc, w) = muster_units(loc, wealth, avail);
        // resulting_loc = muster_strongholds(loc);
    }
    return (resulting_loc, w);
}

// TODO: Check available when adding units
fn muster_units(loc: Space, wealth: u8, avail: CivitatesHolding) -> (Space, u8) {
    let mut resulting_loc: Space<'_> = loc.clone();
    let mut cubes_to_place = 0;

    for stronghold_site in loc.stronghold_sites {
        match stronghold_site.stronghold {
            Some(s) => match s.class {
                StrongholdClass::Hillfort => cubes_to_place += 1,
                StrongholdClass::Town => cubes_to_place += 1,
                _ => {}
            },
            None => {}
        }
    }

    match loc.control {
        Some(p) => {
            if p == Player::Civitates {
                cubes_to_place += loc.pop;
            }
        }
        None => {}
    }

    println!(
        "Placing {} cubes. Place Comitates instead of Militia?\nEach Comitates costs 1 Wealth.\nCurrent Wealth: {}",
        cubes_to_place, wealth
    );
    let mut num_com: Result<u8, std::num::ParseIntError>;
    loop {
        let comitates_to_place: String = Input::new()
            .allow_empty(true)
            .with_prompt("Enter number of Comitates to place instead (default: 0)")
            .interact()
            .unwrap();
        if comitates_to_place == "" {
            num_com = Ok(0);
        } else {
            num_com = comitates_to_place.parse::<u8>();
        }
        match num_com {
            Ok(n) => {
                if n > cubes_to_place {
                    println!(
                        "Error: tried placing {} Comitates but there are only {} cubes to place",
                        n, cubes_to_place
                    );
                }
                if n > wealth {
                    println!(
                        "Error: tried placing {} Comitates but may only spend {} Wealth",
                        n, wealth
                    );
                } else {
                    println!("Placed {} Comitates", num_com.unwrap());
                    break;
                }
            }
            _ => println!("Invalid input, must enter a non-negative integer"),
        }
    }

    resulting_loc
        .units
        .append(&mut Unit::con_militia(cubes_to_place));
    return (resulting_loc, wealth);
}

// fn muster_strongholds(loc: Space) -> Space {}

#[cfg(test)]
mod tests {
    use super::super::board::{Space, SpaceType, StrongholdSite, StrongholdSiteType, Terrain};
    use super::super::concepts::{Player, Stronghold};

    use super::*;

    #[test]
    fn test_muster() {
        let town: Stronghold = Stronghold::new(StrongholdClass::Town, None, None);
        let aquae_sulis: StrongholdSite<'_> = StrongholdSite {
            name: String::from("Aquae Sulis"),
            site_type: StrongholdSiteType::Town,
            stronghold: Some(&town),
        };
        let south_cadbury: StrongholdSite<'_> = StrongholdSite {
            name: String::from("South Cadbury"),
            site_type: StrongholdSiteType::Hillfort,
            stronghold: None,
        };

        let avail: CivitatesHolding = CivitatesHolding::blank();

        let test_space: Space<'_> = Space {
            id: 0,
            name: String::from("Durotriges"),
            space_type: SpaceType::Region,
            terrain: Some(Terrain::Clear),
            adj_spaces: vec![],
            adj_seas: vec![],
            pop: 2,
            max_pop: 3,
            top_prosp: 2,
            bottom_prosp: 2,
            stronghold_sites: vec![aquae_sulis, south_cadbury],
            units: vec![],
            control: Some(Player::Civitates),
        };
        let (after, _): (Space<'_>, u8) = muster(test_space, 2, avail);
        assert_eq!(after.units.len(), 3);
    }
}
