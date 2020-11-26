use bevy::prelude::*;

use crate::{
    Materials,
    game_data::{GameData, GameState, PausedState},
    paddle::Paddle,
    modifiers::{Modifiers, ModifierType},
    SPEED_COLOR,
    SIZE_COLOR,
    WINDOW_HEIGHT,
    WINDOW_WIDTH,
};

pub struct LevelLivesText;
pub struct SpeedText;
pub struct SizeText;
pub struct PausedText;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system_to_stage("spawn", spawn_text.system())
            .add_system(update_text.system())
            .add_system(paused_text.system());
    }
}

fn spawn_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: Res<Materials>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn(UiCameraComponents::default())
        // root node
        .spawn(NodeComponents {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(20.),
                    bottom: Val::Px(100.),
                    ..Default::default()
                },
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::Column,
                // flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            material: materials.background_material.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextComponents {
                    text: Text {
                        value: "...".to_string(), //just dumb init value
                        font: font.clone(),
                        style: TextStyle {
                            font_size: 30.0,
                            color: Color::rgb(0.8, 0.8, 0.8),
                        },
                    },
                    ..Default::default()
                })
                .with(LevelLivesText);
            parent
                .spawn(TextComponents {
                    text: Text {
                        value: "...".to_string(), //just dumb init value
                        font: font.clone(),
                        style: TextStyle {
                            font_size: 30.0,
                            color: Color::rgb(SPEED_COLOR.0, SPEED_COLOR.1, SPEED_COLOR.2),
                        },
                    },
                    style: Style {
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with(SpeedText);
            parent
                .spawn(TextComponents {
                    text: Text {
                        value: "...".to_string(), //just dumb init value
                        font: font.clone(),
                        style: TextStyle {
                            font_size: 30.0,
                            color: Color::rgb(SIZE_COLOR.0, SIZE_COLOR.1, SIZE_COLOR.2),
                        },
                    },
                    style: Style {
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with(SizeText);
        });

    // Spawn paused text
    commands
        .spawn(UiCameraComponents::default())
        // root node
        .spawn(NodeComponents {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(WINDOW_WIDTH / 2. - 250.),
                    bottom: Val::Px(WINDOW_HEIGHT / 2.),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: materials.background_material.clone(),
            ..Default::default()
        }).with_children(|parent| {
            parent
                .spawn(TextComponents {
                    text: Text {
                        value: "...".to_string(), //just dumb init value
                        font: font.clone(),
                        style: TextStyle {
                            font_size: 30.0,
                            color: Color::rgb(SPEED_COLOR.0, SPEED_COLOR.1, SPEED_COLOR.2),
                        },
                    },
                    ..Default::default()
                })
                .with(PausedText);
        });
}

fn update_text(
    game_data: Res<GameData>,
    mut level_lives_text_q: Query<With<LevelLivesText, &mut Text>>,
    mut size_text_q: Query<With<SizeText, &mut Text>>,
    mut speed_text_q: Query<With<SpeedText, &mut Text>>,
    paddle_q: Query<With<Paddle, &Modifiers>>
) {

    let text_maybe = level_lives_text_q.iter_mut().next();
    // For testing no UI
    if text_maybe.is_none() {
        return;
    }
    let mut text = text_maybe.unwrap();
    text.value = format!(
        "Lives: {}\nLevel: {}",
        game_data.lives,
        game_data.level,
    );

    let mut speed_text = speed_text_q.iter_mut().next().unwrap();
    let mut size_text = size_text_q.iter_mut().next().unwrap();

    // Modifiers are active only if the paddle is spawned, then we can display speed & size
    let modifiers = paddle_q.iter().next();
    if let Some(modifiers) = modifiers {
        let speed_mod = modifiers.modifiers.iter().find(|modifier| modifier.mod_type == ModifierType::Speed);
        let size_mod = modifiers.modifiers.iter().find(|modifier| modifier.mod_type == ModifierType::SizeX);

        let speed = match speed_mod {
            Some(modifier) => modifier.value,
            None => 1.,
        };
        speed_text.value =  format!("Speed: {}", speed);

        let size = match size_mod {
            Some(modifier) => modifier.value,
            None => 1.,
        };
        size_text.value =  format!("Size: {}", size);
    } else {
        speed_text.value =  format!("");
        size_text.value =  format!("");
    }
}

fn paused_text(
    game_data: Res<GameData>,
    mut paused_text_q: Query<With<PausedText,& mut Text>>,
) {
    let text_maybe = paused_text_q.iter_mut().next();
    // For testing no UI
    if text_maybe.is_none() {
        return;
    }
    let mut text = text_maybe.unwrap();
    match game_data.state {
        GameState::Paused(PausedState::Start) => text.value = format!("Press SPACE to start the game"),
        GameState::Paused(PausedState::Won) => {
            text.value = format!("You conquered the level! Press SPACE to start level {}", game_data.level);
        },
        GameState::Paused(PausedState::Lost) => {
            text.value = format!("You lost! Press SPACE to start from level 1");
        },
        _ => text.value = format!(""),
    }
}