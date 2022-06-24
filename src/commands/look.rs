use bevy::prelude::*;

use crate::{components::look_description::LookDescription, output::animate_typing};

/// The player issued a look command
pub struct LookEvent(pub String);

pub fn handle_look(
    mut ev_look: EventReader<LookEvent>,
    q_entities: Query<(&Name, &LookDescription)>,
) {
    'event_loop: for ev in ev_look.iter() {
        let target = ev.0.clone().to_uppercase();

        for (name, description) in q_entities.iter() {
            let name = name.as_str().to_uppercase();

            if name == target {
                // Entity with the name found, show its description
                animate_typing(description);
                break 'event_loop;
            }
        }

        animate_typing(format!("I don't know what {target} is."))
    }
}
