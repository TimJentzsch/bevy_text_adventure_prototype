use bevy::prelude::*;

use crate::{
    components::look_description::LookDescription,
    items::Item,
    locations::{CurLocation, Location},
    output::animate_typing,
    relations::{ItemAtLocation, LocationConnection},
};

/// The player issued a look command
pub struct LookEvent(pub String);

pub fn handle_look(
    mut ev_look: EventReader<LookEvent>,
    q_cur_location: Query<&CurLocation>,
    q_items: Query<(&Name, &LookDescription), With<Item>>,
    q_item_locations: Query<&ItemAtLocation>,
    q_locations: Query<(&Name, &LookDescription), With<Location>>,
    q_location_connections: Query<&LocationConnection>,
) {
    let cur_location = q_cur_location.single();

    for ev in ev_look.iter() {
        let target = ev.0.clone().to_uppercase();

        // Check if an item with that name is available at this location
        for item_location in q_item_locations.iter() {
            if let Some(connected_entity) = item_location.available_item(cur_location.0) {
                if let Ok((name, description)) = q_items.get(connected_entity) {
                    let name = name.as_str().to_uppercase();

                    if name == target {
                        // Entity with the name found, show its description
                        animate_typing(description);
                        return;
                    }
                }
            }
        }

        // Check if a location with that name is reachable from the current one
        for connection in q_location_connections.iter() {
            if let Some(connected_entity) = connection.connected_location(cur_location.0) {
                if let Ok((name, description)) = q_locations.get(connected_entity) {
                    // Check if the connected location matches the name
                    let name = name.as_str().to_uppercase();

                    if name == target {
                        // Entity with the name found, show its description
                        animate_typing(description);
                        return;
                    }
                }
            }
        }

        // Check if the current location matches the name
        if let Ok((name, description)) = q_locations.get(cur_location.0) {
            // Check if the connected location matches the name
            let name = name.as_str().to_uppercase();

            if name == target {
                // Entity with the name found, show its description
                animate_typing(description);
                return;
            }
        }

        animate_typing(format!("There is no {target} here to look at."))
    }
}
