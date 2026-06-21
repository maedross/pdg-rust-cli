use super::concepts::{
    Player, Space, StrongholdClass, StrongholdSite, StrongholdSiteType, Unit,
};
use dialoguer::Input;

// TODO: func for selecting spaces

pub fn muster(loc: Space) -> Space {
    let resulting_loc: Space;
    if true {
        resulting_loc = muster_units(loc);
    } else {
        resulting_loc = muster_units(loc);
        // resulting_loc = muster_strongholds(loc);
    }
    return resulting_loc;
}

// TODO: Check available when adding units
// TODO: Spend wealth for Comitates
fn muster_units(loc: Space) -> Space {
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
        "Placing {} cubes. Place Comitates instead of Militia?",
        cubes_to_place
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
        match  num_com {
            Ok(_) => break,
            _ => println!("Invalid input, must enter a non-negative integer"),
        }
    }
    println!("Placed {} Comitates", num_com.unwrap());
    resulting_loc
        .units
        .append(&mut Unit::con_militia(cubes_to_place));
    return resulting_loc;
}

// fn muster_strongholds(loc: Space) -> Space {}

#[cfg(test)]
mod tests {
    use super::super::concepts::{
        Player, Space, SpaceType, Stronghold, StrongholdSite, StrongholdSiteType, Terrain,
    };

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

        let test_space: Space<'_> = Space {
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
        let after: Space<'_> = muster(test_space);
        assert_eq!(after.units.len(), 3);
    }
}
