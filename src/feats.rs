use std::fmt;
use super::board::{Board, Imperium, Space};
use super::concepts::{Player, StrongholdClass};
use super::commands::{Command};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Feat {
    Build,
    Invite,
    Requisition,
    Retaliate,
    Rule,
    Reinforce,
    Pillage,
    Settle,
    Surprise,
    Ravage,
    ShieldWall,
    Ransom,
    Entreat,
}

impl fmt::Display for Feat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Feat::Build => write!(f, "Build"),
            Feat::Invite => write!(f, "Invite"),
            Feat::Requisition => write!(f, "Requisition"),
            Feat::Retaliate => write!(f, "Retaliate"),
            Feat::Rule => write!(f, "Rule"),
            Feat::Reinforce => write!(f, "Reinforce"),
            Feat::Pillage => write!(f, "Pillage"),
            Feat::Settle => write!(f, "Settle"),
            Feat::Surprise => write!(f, "Surprise"),
            Feat::Ravage => write!(f, "Ravage"),
            Feat::ShieldWall => write!(f, "ShieldWall"),
            Feat::Ransom => write!(f, "Ransom"),
            Feat::Entreat => write!(f, "Entreat"),
        }
    }
}

pub fn get_faction_feats(
    current_player: Player,
    command: Option<Command>,
    imperium: Imperium,
) -> Vec<Feat> {
    match command {
        None => match (current_player, imperium) {
            (Player::Civitates, Imperium::Fragmentation) => vec![Feat::Rule, Feat::Reinforce, Feat::Pillage],
            (Player::Civitates, _) => vec![Feat::Rule, Feat::Invite, Feat::Reinforce, Feat::Pillage],
            (Player::Dux, Imperium::Fragmentation) => vec![Feat::Build, Feat::Requisition, Feat::Retaliate],
            (Player::Dux, _) => vec![Feat::Build, Feat::Invite, Feat::Requisition, Feat::Retaliate],
            (Player::Saxons, _) => vec![Feat::Settle, Feat::Surprise, Feat::Ravage, Feat::ShieldWall],
            (Player::Scotti, _) => vec![Feat::Settle, Feat::Surprise, Feat::Ransom, Feat::Entreat],
        },
        Some(c) => match(c, current_player, imperium) {
            (Command::Muster, Player::Civitates, Imperium::Fragmentation) => vec![Feat::Rule],
            (Command::Muster, Player::Civitates, _) => vec![Feat::Rule, Feat::Invite],
            (Command::March, Player::Civitates, Imperium::Fragmentation) => vec![Feat::Rule, Feat::Pillage],
            (Command::March, Player::Civitates, _) => vec![Feat::Rule, Feat::Invite, Feat::Pillage],
            (Command::Trade, Player::Civitates, Imperium::Fragmentation) => vec![Feat::Rule],
            (Command::Trade, Player::Civitates, _) => vec![Feat::Rule, Feat::Invite],
            (Command::Battle, Player::Civitates, _) => vec![Feat::Reinforce, Feat::Pillage],
            (Command::Train, Player::Dux, Imperium::Fragmentation) => vec![Feat::Build, Feat::Requisition],
            (Command::Train, Player::Dux, _) => vec![Feat::Build, Feat::Invite, Feat::Requisition],
            (Command::March, Player::Dux, Imperium::Fragmentation) => vec![Feat::Build, Feat::Requisition],
            (Command::March, Player::Dux, _) => vec![Feat::Build, Feat::Invite, Feat::Requisition],
            (Command::Intercept, Player::Dux, Imperium::Fragmentation) => vec![Feat::Retaliate],
            (Command::Intercept, Player::Dux, _) => vec![Feat::Invite, Feat::Retaliate],
            (Command::Battle, Player::Dux, _) => vec![Feat::Requisition, Feat::Retaliate],
            (Command::Raid, Player::Saxons, _) => vec![Feat::Surprise, Feat::Ravage],
            (Command::Return, Player::Saxons, _) => vec![Feat::Settle],
            (Command::March, Player::Saxons, _) => vec![Feat::Settle],
            (Command::Battle, Player::Saxons, _) => vec![Feat::Surprise, Feat::Ravage, Feat::ShieldWall],
            (Command::Raid, Player::Scotti, _) => vec![Feat::Surprise, Feat::Ransom],
            (Command::Return, Player::Scotti, _) => vec![Feat::Settle, Feat::Entreat],
            (Command::March, Player::Scotti, _) => vec![Feat::Settle, Feat::Entreat],
            (Command::Battle, Player::Scotti, _) => vec![Feat::Surprise, Feat::Ransom, Feat::Entreat],
            _ => panic!("Invalid combination of Command, Player, and Imperium: {:?}, {}, {:?}", command, current_player, imperium)
        }
    }
}