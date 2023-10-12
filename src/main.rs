mod turns;
mod constants;
use bevy::prelude::*;

fn main() {
    
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::WHITE))
        .add_systems(Startup, setup)
        .run();
}

fn setup (
    mut commands: Commands
){
    let mut starting_turn: i32 = 1;
    let mut game_has_finished: bool = false;
    let mut game_board: [[char; 3]; 3] = [[constants::EMPTY_CELL_CHARACTER; 3]; 3];

    commands.spawn(Camera2dBundle::default());

    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Bevy Tic-Tac-Toe",
                TextStyle {
                    font_size: 40.0, 
                    color: Color::BLACK,
                    ..default()
                }
            )
        ])
        .with_text_alignment(TextAlignment::Center)
        .with_style(
            Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                right: Val::Px(10.0),
                ..default()
            }
        )
    );
}