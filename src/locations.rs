use bevy::prelude::*;

use crate::{events::EventSystems, general::Description, output::animate_typing};

pub struct LocationPlugin;

impl Plugin for LocationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(location.before(EventSystems));
    }
}

/// Mark the entity as location
#[derive(Component)]
pub struct Location;

/// The location where the player currently is
#[derive(Component)]
pub struct CurLocation(pub Entity);

fn location(q_cur_location: Query<&CurLocation>, q_components: Query<(&Name, &Description)>) {
    let cur_location = q_cur_location.single();

    let (name, description) = q_components
        .get(cur_location.0)
        .expect("No current location defined!");

    animate_typing(&name.as_str().to_uppercase());
    animate_typing(&description.0);
}
