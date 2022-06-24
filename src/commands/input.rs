use std::io::{stdin, stdout, Write};

use bevy::prelude::EventWriter;

/// The player wrote something
pub struct InputEvent(pub String);

/// Ask the player for new input
pub fn ask_input(mut ev_input: EventWriter<InputEvent>) {
    print!("> ");
    stdout().flush().unwrap();

    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    ev_input.send(InputEvent(line.strip_suffix('\n').unwrap().to_owned()));
}
