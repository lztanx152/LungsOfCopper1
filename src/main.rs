use bevy::a11y::ActionRequest;
use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::*;

#[derive(Debug,Clone,Copy,Default,Eq,PartialEq,Hash, States)]
enum GameState {
    #[default]
    Menu,
    Playing,
}

// fn text_button(label: &'static str, action: ButtonAc)

fn setup_menu(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(GameState::Menu),
        Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items:AlignItems::Center,
            row_gap: px(20),
            ..default()
        },
        children![
            (Text::new("Test For mines"), TextFont::from_font_size(35.0)),
            (
                Node {
                flex_direction: FlexDirection::Column,
                row_gap: px(10),
                ..default()
            },
            )
        ],
        // children![
        //     text_button("easy")
        // ]
        ));
}

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::Menu),setup_menu)
        .run();
}
