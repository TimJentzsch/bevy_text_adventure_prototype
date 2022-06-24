use bevy::prelude::*;

use crate::output::animate_typing;

use super::{go::GoEvent, input::InputEvent, look::LookEvent, HelpEvent, UseEvent};

/// Parse the input given by the player
pub fn parse_input(
    mut ev_input: EventReader<InputEvent>,
    mut ev_help: EventWriter<HelpEvent>,
    mut ev_go: EventWriter<GoEvent>,
    mut ev_look: EventWriter<LookEvent>,
    mut ev_use: EventWriter<UseEvent>,
) {
    for ev in ev_input.iter() {
        let mut tokens = ev.0.split_ascii_whitespace();

        if let Some(cmd_token) = tokens.next() {
            let rest = tokens.as_str();

            match cmd_token.to_lowercase().as_str() {
                "help" => ev_help.send(HelpEvent),
                "go" => ev_go.send(GoEvent(rest.to_string())),
                "look" => ev_look.send(LookEvent(rest.to_string())),
                "use" => ev_use.send(UseEvent(rest.to_string())),
                _ => animate_typing(format!("Unknown command: '{cmd_token}'")),
            }
        }
    }
}
