use bevy::a11y::ActionRequest;
use bevy::app::App;
use bevy::color::palettes::css::{DARK_GREEN, SEA_GREEN};
use bevy::DefaultPlugins;
use bevy::log::tracing_subscriber::util::SubscriberInitExt;
use bevy::prelude::*;
use bevy::text::LineHeight;

const TILE_BORDER_COLOR: Color = Color::Srgba(SEA_GREEN);
const BACKGROUND_COLOR: Color = Color::Srgba(DARK_GREEN);


#[derive(Debug,Clone,Copy,Default,Eq,PartialEq,Hash, States)]
enum GameState {
    #[default]
    Menu,
    Playing,
}

fn text_button(label: &'static str, action: ButtonAction) -> impl Bundle {
    (
        action,
        Node {
            min_width: px(144),
            border: px(3).all(),
            border_radius: px(8).into(),
            box_sizing: BoxSizing::ContentBox,
            ..default()
        },
        BorderColor::all(TILE_BORDER_COLOR),
        BackgroundColor(BACKGROUND_COLOR),
        Text::new(label),
        TextFont::from_font_size(24.0),
        TextLayout::justify(Justify::Center),
        LineHeight::Px(40.),
    )
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Difficulty {
    Easy,
    Normal,
    Hard,
}

#[derive(Component, Clone, Copy)]
enum ButtonAction {
    NewGame(Difficulty),
    Menu,
}

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
                children![
                text_button("easy", ButtonAction::NewGame(Difficulty::Easy)),
                text_button("normal", ButtonAction::NewGame(Difficulty::Normal)),
                text_button("hard", ButtonAction::NewGame(Difficulty::Hard)),
            ],
            ),
        ],

        ));
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(GameState::Menu),setup_menu)
        .run();
}
