use std::io::{stdin, stdout, Write};

use bevy::prelude::*;

use crate::{
    locations::{CurLocation, Location},
    output::animate_typing,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub struct EventSystems;

pub struct EventPlugin;

impl Plugin for EventPlugin {
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

/// The player wrote something
struct InputEvent(String);

struct HelpEvent;

/// The player issued a go command
struct GoEvent(String);

/// The player issued a look command
struct LookEvent(String);

/// The player issued a use command
struct UseEvent(String);

/// Ask the player for new input
fn ask_input(mut ev_input: EventWriter<InputEvent>) {
    print!("> ");
    stdout().flush().unwrap();

    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    ev_input.send(InputEvent(line.strip_suffix('\n').unwrap().to_owned()));
}

/// Parse the input given by the player
fn parse_input(
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

fn handle_help(mut ev_help: EventReader<HelpEvent>) {
    for _ in ev_help.iter() {
        animate_typing(
            "Available commands:
- HELP
- GO <location>
- LOOK <object>
- USE <item>",
        )
    }
}

fn handle_go(
    mut ev_go: EventReader<GoEvent>,
    mut q_cur_location: Query<&mut CurLocation>,
    q_locations: Query<(Entity, &Name), With<Location>>,
) {
    for ev in ev_go.iter() {
        let target = ev.0.clone().to_uppercase();
        let mut cur_location = q_cur_location.single_mut();
        let mut new_location: Option<(Entity, String)> = None;

        for (entity, name) in q_locations.iter() {
            let name = name.as_str().to_uppercase();

            // Go to the new location
            if name == target {
                new_location = Some((entity, name))
            }
        }

        if let Some((new_location, name)) = new_location {
            if cur_location.0 == new_location {
                // The player wants to go to the same location
                animate_typing(format!("You are already at {name}!"));
            } else {
                // Change the location
                cur_location.0 = new_location;
                animate_typing(format!("You are now at {name}!"));
            }
        } else {
            // Invalid location name
            animate_typing(format!("I don't know what {target} is!"));
        }
    }
}
