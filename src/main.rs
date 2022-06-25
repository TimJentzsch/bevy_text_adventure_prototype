#![feature(str_split_whitespace_as_str)]
// Temporary until <https://github.com/bevyengine/bevy/issues/4601> is fixed
#![allow(clippy::forget_non_drop)]

use bevy::prelude::*;
use commands::CommandPlugin;
use items::ItemBundle;
use locations::{CurLocation, LocationBundle, LocationPlugin};

mod commands;
mod components;
mod items;
mod locations;
mod output;

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
        .insert_bundle(ItemBundle::new(
            "Desk",
            "It's full of old papers and a thin layer of dust. It might be time for a clean up... maybe tomorrow."
        ));

    commands
        .spawn()
        .insert_bundle(ItemBundle::new(
            "Chair",
            "One of the legs is a bit shorter than the others, making it wiggle around every time you move."
        ));

    commands.spawn().insert(CurLocation(home));
}
