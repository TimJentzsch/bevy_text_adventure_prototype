#![feature(str_split_whitespace_as_str)]

use ::bevy::prelude::*;
use events::EventPlugin;
use general::Description;
use locations::{CurLocation, Location, LocationPlugin};

mod events;
mod general;
mod locations;
mod output;

/// Mark the entity as item
#[derive(Component)]
struct Item;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(LocationPlugin)
        .add_plugin(EventPlugin)
        .add_startup_system(startup)
        .run();
}

fn startup(mut commands: Commands) {
    let home = commands
        .spawn()
        .insert(Location)
        .insert(Name::new("Home"))
        .insert(Description("A cozy place for lovely people.".to_string()))
        .id();

    commands
        .spawn()
        .insert(Location)
        .insert(Name::new("Not Home"))
        .insert(Description("A weird place that's outside.".to_string()));

    commands.spawn().insert(CurLocation(home));
}
