use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

/// Give the text a typing animation
pub fn animate_typing(text: &str) {
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
