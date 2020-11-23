use bevy::prelude::*;

use crate::game_data::GameData;

pub fn spawn_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(TextComponents {
        text: Text {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            value: "".to_string(),
            style: TextStyle {
                color: Color::rgb(1., 1., 1.),
                font_size: 100.0,
                ..Default::default()
            },
        },
        /*
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(50.0),
                left: Val::Px(50.0),
                ..Default::default()
            },
            align_self: AlignSelf::FlexEnd,
            border: Rect {
                top: Val::Px(5.0),
                right: Val::Px(5.0),
                bottom: Val::Px(5.0),
                left: Val::Px(5.0),
            },
            ..Default::default()
        },
        */
        ..Default::default()
    });
}

pub fn update_text(
    game_data: Res<GameData>,
    mut query: Query<&mut Text>
) {
    for mut text in query.iter_mut() {
        text.value = format!("Lives: {}\n Level: {}", game_data.lives, game_data.level);
    }
}