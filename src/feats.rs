use super::board::{Board, Imperium};
use super::commands::Command;
use super::concepts::{Player};
use super::sequence_of_play::{Action};
use std::fmt;

pub fn get_feats(command: Command, imperium: Imperium) -> Vec<Action> {
    match command {
        Command::Muster => match imperium {
            Imperium::Fragmentation => vec![RULE],
            _ => vec![RULE, INVITE_CIVITATTES],
        },
        Command::CivitatesMarch => match imperium {
            Imperium::Fragmentation => vec![RULE, PILLAGE],
            _ => vec![RULE, INVITE_CIVITATTES, PILLAGE],
        },
        Command::Trade => match imperium {
            Imperium::Fragmentation => vec![RULE],
            _ => vec![RULE, INVITE_CIVITATTES],
        },
        Command::CivitatesBattle => vec![REINFORCE, PILLAGE],
        Command::Train => match imperium {
            Imperium::Fragmentation => vec![BUILD, REQUISITION],
            _ => vec![BUILD, INVITE_DUX, REQUISITION],
        },
        Command::DuxMarch => match imperium {
            Imperium::Fragmentation => vec![BUILD, REQUISITION],
            _ => vec![BUILD, INVITE_DUX, REQUISITION],
        },
        Command::Intercept => match  imperium {
            Imperium::Fragmentation => vec![RETALIATE],
            _ => vec![INVITE_DUX, RETALIATE],
        },
        Command::DuxBattle => vec![REQUISITION, RETALIATE],
        Command::SaxonRaid => vec![SURPRISE_SAXONS, RAVAGE],
        Command::SaxonReturn => vec![SETTLE_SAXONS],
        Command::SaxonMarch => vec![SETTLE_SAXONS],
        Command::SaxonBattle => vec![SURPRISE_SAXONS, RAVAGE, SHIELDWALL],
        Command::ScottiRaid => vec![SURPRISE_SCOTTI, RANSOM],
        Command::ScottiReturn => vec![SETTLE_SCOTTI, ENTREAT],
        Command::ScottiMarch => vec![SETTLE_SCOTTI],
        Command::ScottiBattle => vec![SURPRISE_SCOTTI, RANSOM, ENTREAT],
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