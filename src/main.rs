#![feature(str_split_whitespace_as_str)]

use bevy::prelude::*;
use commands::CommandPlugin;
use components::look_description::LookDescription;
use locations::{CurLocation, LocationBundle, LocationPlugin};

mod commands;
mod components;
mod locations;
mod output;

/// Mark the entity as item
#[derive(Component)]
struct Item;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(LocationPlugin)
        .add_plugin(CommandPlugin)
        .add_startup_system(startup)
        .run();
}

fn startup(mut commands: Commands) {
    let home = commands
        .spawn()
        .insert_bundle(LocationBundle::new(
            "Home",
            "A cozy place for lovely people.",
            "A dim lamp lights your DESK, with an old CHAIR standing in front of it.",
        ))
        .id();

    commands.spawn().insert_bundle(LocationBundle::new(
        "Outside",
        "A weird place that's outside.",
        "It's cloudy, maybe it's gonna rain soon.",
    ));

    commands
        .spawn()
        .insert(Item)
        .insert(Name::new("Desk"))
        .insert(LookDescription::new(
            "It's full of old papers and a thin layer of dust. It might be time for a clean up... maybe tomorrow."
        ));

    commands
        .spawn()
        .insert(Item)
        .insert(Name::new("Chair"))
        .insert(LookDescription::new(
            "One of the legs is a bit shorter than the others, making it wiggle around every time you move."
        ));

    commands.spawn().insert(CurLocation(home));
}
