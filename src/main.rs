use std::{
    io::{stdin, stdout, Write},
    thread,
    time::Duration,
};

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

/// The player wrote something
struct InputEvent(String);

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_event::<InputEvent>()
        .add_startup_system(startup)
        .add_system(location)
        .add_system(parse_input.after(location))
        .run();
}

fn startup(mut commands: Commands) {
    commands
        .spawn()
        .insert(Location)
        .insert(Name("Home".to_string()))
        .insert(Description("A cozy place for lovely people.".to_string()))
        .insert(CurLocation);

    commands
        .spawn()
        .insert(Location)
        .insert(Name("Not Home".to_string()))
        .insert(Description("A weird place that's outside.".to_string()));
}

fn location(
    mut ev_input: EventWriter<InputEvent>,
    query: Query<(&Name, &Description), (With<Location>, With<CurLocation>)>,
) {
    let (name, description) = query.single();

    animate_typing(&name.0.to_uppercase());
    animate_typing(&description.0);

    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    ev_input.send(InputEvent(line.strip_suffix('\n').unwrap().to_owned()));
}

fn parse_input(mut ev_input: EventReader<InputEvent>) {
    for ev in ev_input.iter() {
        println!("Parsing input '{}'...", ev.0);
    }
}

/// Give the text a typing animation
fn animate_typing(text: &str) {
    let chars: Vec<char> = text.chars().collect();

    if chars.is_empty() {
        return;
    }

    for ch in &chars.as_slice()[..(chars.len() - 1)] {
        print!("{}", ch);
        stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(10));
    }

    println!("{}", chars.last().unwrap());
}
