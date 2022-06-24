use bevy::prelude::*;

use self::{
    go::{handle_go, GoEvent},
    help::{handle_help, HelpEvent},
    input::{ask_input, InputEvent},
    parse::parse_input,
};

mod go;
mod help;
mod input;
mod parse;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub struct EventSystems;

pub struct CommandPlugin;

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputEvent>()
            .add_event::<HelpEvent>()
            .add_event::<GoEvent>()
            .add_event::<LookEvent>()
            .add_event::<UseEvent>()
            .add_system_set(
                SystemSet::new()
                    .label(EventSystems)
                    .with_system(ask_input)
                    .with_system(parse_input.after(ask_input))
                    .with_system(handle_help.after(parse_input))
                    .with_system(handle_go.after(parse_input)),
            );
    }
}

/// The player issued a look command
pub struct LookEvent(String);

/// The player issued a use command
pub struct UseEvent(String);
