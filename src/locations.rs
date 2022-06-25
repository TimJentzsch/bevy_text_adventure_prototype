use std::borrow::Cow;

use bevy::prelude::*;

use crate::{
    commands::EventSystems,
    components::{go_description::GoDescription, look_description::LookDescription},
    output::animate_typing,
};

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

fn location(q_cur_location: Query<&CurLocation>, q_components: Query<(&Name, &GoDescription)>) {
    let cur_location = q_cur_location.single();

    let (name, description) = q_components
        .get(cur_location.0)
        .expect("No current location defined!");

    animate_typing(name.as_str().to_uppercase());
    animate_typing(description);
}

#[derive(Bundle)]
pub struct LocationBundle {
    location: Location,
    name: Name,
    go_description: GoDescription,
    look_description: LookDescription,
}

impl LocationBundle {
    pub fn new(
        name: impl Into<Cow<'static, str>>,
        go_description: impl Into<Cow<'static, str>>,
        look_description: impl Into<Cow<'static, str>>,
    ) -> Self {
        LocationBundle {
            location: Location,
            name: Name::new(name),
            go_description: GoDescription::new(go_description),
            look_description: LookDescription::new(look_description),
        }
    }
}
