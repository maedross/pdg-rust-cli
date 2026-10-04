use super::board::{Board, Imperium};
use super::commands::Command;
use super::concepts::{Player};
use super::sequence_of_play::{Action};
use std::fmt;

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

pub const RULE: Action = Action::Feat { name: "Rule", action: rule };
pub const INVITE_CIVITATTES: Action = Action::Feat { name: "Invite", action: invite_civitates };
pub const REINFORCE: Action = Action::Feat { name: "Reinforce", action: reinforce };
pub const PILLAGE: Action = Action::Feat { name: "Pillage", action: pillage };

pub const BUILD: Action = Action::Feat { name: "Build", action: build };
pub const INVITE_DUX: Action = Action::Feat { name: "Invite", action: invite_dux };
pub const REQUISITION: Action = Action::Feat { name: "Requisition", action: requisition };
pub const RETALIATE: Action = Action::Feat { name: "Retaliate", action: retaliate };

pub const SETTLE_SAXONS: Action = Action::Feat { name: "Settle", action: settle_saxons };
pub const SURPRISE_SAXONS: Action = Action::Feat { name: "Surprise", action: surprise_saxons };
pub const RAVAGE: Action = Action::Feat { name: "Ravage", action: ravage };
pub const SHIELDWALL: Action = Action::Feat { name: "Shieldwall", action: shieldwall };

pub const SETTLE_SCOTTI: Action = Action::Feat { name: "Settle", action: settle_scotti };
pub const SURPRISE_SCOTTI: Action = Action::Feat { name: "Surprise", action: surprise_scotti };
pub const RANSOM: Action = Action::Feat { name: "Ransom", action: ransom };
pub const ENTREAT: Action = Action::Feat { name: "Entreat", action: entreat };

fn rule(board: &mut Board) {

}

fn invite_civitates(board: &mut Board) {

}

fn reinforce(board: &mut Board) {

}

fn pillage(board: &mut Board) {

}

fn build(board: &mut Board) {

}

fn invite_dux(board: &mut Board) {

}

fn requisition(board: &mut Board) {

}

fn retaliate(board: &mut Board) {

}

fn settle_saxons(board: &mut Board) {

}

fn surprise_saxons(board: &mut Board) {

}

fn ravage(board: &mut Board) {

}

fn shieldwall(board: &mut Board) {

}

fn settle_scotti(board: &mut Board) {

}

fn surprise_scotti(board: &mut Board) {

}

fn ransom(board: &mut Board) {

}

fn entreat(board: &mut Board) {

}