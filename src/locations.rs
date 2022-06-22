use bevy::prelude::*;

use crate::{general::Description, output::animate_typing};

pub struct LocationPlugin;

impl Plugin for LocationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(location);
    }
}

/// Mark the entity as location
#[derive(Component)]
pub struct Location;

/// Mark the entity as the current location where the player is
#[derive(Component)]
pub struct CurLocation;

fn location(query: Query<(&Name, &Description), (With<Location>, With<CurLocation>)>) {
    let (name, description) = query.single();

    animate_typing(&name.as_str().to_uppercase());
    animate_typing(&description.0);
}
