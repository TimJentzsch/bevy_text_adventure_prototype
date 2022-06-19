use std::io::stdin;

use ::bevy::prelude::*;

/// The name of the entity
#[derive(Component)]
struct Name(String);

/// The description to show for the entity
#[derive(Component)]
struct Description(String);

/// Mark the entity as location
#[derive(Component)]
struct Location;

/// Mark the entity as the current location where the player is
#[derive(Component)]
struct CurLocation;

/// Mark the entity as item
#[derive(Component)]
struct Item;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_startup_system(startup)
        .add_system(location)
        .run();
}

fn startup(mut commands: Commands) {
    commands
        .spawn()
        .insert(Location)
        .insert(Name("Home".to_string()))
        .insert(Description("A cozy place for lovely people.".to_string()))
        .insert(CurLocation);
}

fn location(query: Query<(&Name, &Description), (With<Location>, With<CurLocation>)>) {
    println!("Current location: {}", query.single().0 .0);

    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    println!("Input: {line}");
}
