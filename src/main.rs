#![feature(str_split_whitespace_as_str)]

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

/// The player issued a go command
struct GoEvent(String);

/// The player issued a look command
struct LookEvent(String);

/// The player issued a use command
struct UseEvent(String);

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_event::<InputEvent>()
        .add_event::<GoEvent>()
        .add_event::<LookEvent>()
        .add_event::<UseEvent>()
        .add_startup_system(startup)
        .add_system(location)
        .add_system(parse_input.after(location))
        .add_system(handle_go.after(parse_input))
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

    print!("> ");
    stdout().flush().unwrap();

    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    ev_input.send(InputEvent(line.strip_suffix('\n').unwrap().to_owned()));
}

fn parse_input(
    mut ev_input: EventReader<InputEvent>,
    mut ev_go: EventWriter<GoEvent>,
    mut ev_look: EventWriter<LookEvent>,
    mut ev_use: EventWriter<UseEvent>,
) {
    for ev in ev_input.iter() {
        let mut tokens = ev.0.split_ascii_whitespace();

        if let Some(cmd_token) = tokens.next() {
            let rest = tokens.as_str();

            match cmd_token.to_lowercase().as_str() {
                "go" => ev_go.send(GoEvent(rest.to_string())),
                "look" => ev_look.send(LookEvent(rest.to_string())),
                "use" => ev_use.send(UseEvent(rest.to_string())),
                _ => animate_typing(&format!("Unknown command: '{cmd_token}'")),
            }
        }
    }
}

fn handle_go(
    mut commands: Commands,
    mut ev_go: EventReader<GoEvent>,
    query: Query<(Entity, &Name, Option<&CurLocation>), With<Location>>,
) {
    for ev in ev_go.iter() {
        let target = ev.0.clone().to_uppercase();
        let mut old_location: Option<Entity> = None;
        let mut new_location: Option<(Entity, String)> = None;

        for (entity, name, cur_location) in query.iter() {
            let name = name.0.clone().to_uppercase();

            // We are leaving the current location
            if cur_location.is_some() {
                old_location = Some(entity);
            }

            // Go to the new location
            if name == target {
                new_location = Some((entity, name))
            }
        }

        if let Some((new_location, name)) = new_location {
            if let Some(old_location) = old_location {
                if old_location == new_location {
                    // The player wants to go to the same location
                    animate_typing(format!("You are already at {name}!").as_str());
                    return;
                }

                // Leave the old location
                commands.entity(old_location).remove::<CurLocation>();
            }

            // Enter the new location
            commands.entity(new_location).insert(CurLocation);
            animate_typing(format!("You are now at {name}!").as_str());
        } else {
            // Invalid location name
            animate_typing(format!("I don't know what {target} is!").as_str());
        }
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
