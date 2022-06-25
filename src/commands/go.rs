use bevy::prelude::*;

use crate::{
    locations::{CurLocation, Location},
    output::animate_typing,
    relations::LocationConnection,
};

/// The player issued a go command
pub struct GoEvent(pub String);

pub fn handle_go(
    mut ev_go: EventReader<GoEvent>,
    mut q_cur_location: Query<&mut CurLocation>,
    q_location_connections: Query<&LocationConnection>,
    q_locations: Query<(Entity, &Name), With<Location>>,
) {
    let mut cur_location = q_cur_location.single_mut();

    for ev in ev_go.iter() {
        let target = ev.0.clone().to_uppercase();
        let mut new_location: Option<(Entity, String)> = None;

        // Check which locations are connected to the current one
        for connection in q_location_connections.iter() {
            if let Some(connected_entity) = connection.connected_location(cur_location.0) {
                // Check if the connected location matches the name
                if let Ok((entity, name)) = q_locations.get(connected_entity) {
                    let name = name.as_str().to_uppercase();

                    // Go to the new location
                    if name == target {
                        new_location = Some((entity, name))
                    }
                }
            }
        }

        if let Some((new_location, name)) = new_location {
            // Change the location
            cur_location.0 = new_location;
            animate_typing(format!("You are now at {name}!"));
        } else {
            // Check if the player is already at the location
            if let Ok((_, name)) = q_locations.get(cur_location.0) {
                if target == name.as_str().to_uppercase() {
                    animate_typing(format!("You are already at {target}!"));
                    return;
                }
            }

            // Check if a location with this name exists at all
            for (_, name) in q_locations.iter() {
                if target == name.as_str().to_uppercase() {
                    animate_typing(format!("You can't get to {target} from here!"));
                    return;
                }
            }

            // Invalid location name
            animate_typing(format!("I don't know what {target} is!"));
        }
    }
}
