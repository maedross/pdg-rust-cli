use std::fmt::{self};

use crate::board::Board;
use crate::commands::Command;
use super::concepts::Player;


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Command {
        command_name: Command,
        space: Option<String>,
        actions: Vec<fn(String, &mut Board)>,
        filter: fn(&mut Board) -> Vec<String>,
    },
    Feat {
        name: &'static str,
        action: fn(&mut Board),
    },
    Pass {
        action: fn(Player, &mut Board),
    },
    Event {
        name: &'static str,
    },
    Done,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Command { command_name, space, actions: _ , filter: _} => {
                match space {
                    Some(s) => write!(f, "{}", s),
                    None => write!(f, "{}", command_name)
                }
            },
            Action::Feat { name, action: _ } => write!(f, "{}", name),
            Action::Pass { action: _ } => write!(f, "Pass"),
            Action::Event { name } => write!(f, "{}", name),
            Action::Done => write!(f, "Done"),
        }
    }
}

impl<'a> FromIterator<&'a Action> for Vec<Action> {
    fn from_iter<T: IntoIterator<Item = &'a Action>>(iter: T) -> Self {
        let mut v = vec![];
        for a in iter {
            v.push(a.clone());
        }
        v
    }
}

enum ActionSpace {
    Standard(String),
    Origin(String),
    Destination(String),
}