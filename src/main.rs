use ::bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_system(hello)
        .run();
}

fn hello() {
    println!("Hello World!");
}
