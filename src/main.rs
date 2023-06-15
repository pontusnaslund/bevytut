use bevy::prelude::*;

fn main() {
    App::new()
        .add_startup_system(add_players)
        .add_system(print_players)
        .run();
}

fn print_players(query: Query<&Name, With<Player>>) {
    for name in &query {
        println!("player: {:?} ", name.0);
    }
}

fn add_players(mut commands: Commands) {
    commands.spawn((Player, Name("Player 1".to_string())));
    commands.spawn((Player, Name("Player 2".to_string())));
    commands.spawn((Player, Name("Player 3".to_string())));
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Name(String);