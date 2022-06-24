#![feature(str_split_whitespace_as_str)]

use bevy::prelude::*;
use commands::CommandPlugin;
use components::{go_description::GoDescription, look_description::LookDescription};
use locations::{CurLocation, Location, LocationPlugin};

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
        .insert(Location)
        .insert(Name::new("Home"))
        .insert(GoDescription::new("A cozy place for lovely people."))
        .insert(LookDescription::new(
            "A dim lamp lights your DESK, with an old CHAIR standing in front of it.",
        ))
        .id();

    commands
        .spawn()
        .insert(Location)
        .insert(Name::new("Not Home"))
        .insert(GoDescription::new("A weird place that's outside."))
        .insert(LookDescription::new(
            "It's cloudy, maybe it's gonna rain soon.",
        ));

    commands.spawn().insert(CurLocation(home));
}
