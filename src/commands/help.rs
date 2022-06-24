use bevy::prelude::EventReader;

use crate::output::animate_typing;

pub struct HelpEvent;

pub fn handle_help(mut ev_help: EventReader<HelpEvent>) {
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
