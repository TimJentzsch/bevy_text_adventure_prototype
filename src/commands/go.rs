use bevy::prelude::*;

use crate::{
    locations::{CurLocation, Location},
    output::animate_typing,
};

/// The player issued a go command
pub struct GoEvent(pub String);

pub fn handle_go(
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
