use bevy::prelude::*;
use crate::{
    Materials,
    collider::{BallHitEvent},
};

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system_to_stage("spawn", spawn_music.system())
            .add_system(play_or_stop_music.system())
            .add_system(sound_effects.system());
    }
}

pub struct Sounds {
    song_1: Handle<AudioSource>,
    hit_1: Handle<AudioSource>,
}

pub fn spawn_music(
    mut commands: Commands,
    materials: Res<Materials>,
    asset_server: Res<AssetServer>,
    audio: ResMut<Audio>
) {
    commands.spawn(ButtonComponents {
        style: Style {
            size: Size::new(Val::Px(80.0), Val::Px(80.0)),
            position_type: PositionType::Absolute,
            position: Rect {
                right: Val::Px(50.),
                bottom: Val::Px(50.),
                ..Default::default()
            },
            ..Default::default()
        },
        material: materials.sound_button.clone(),
        ..Default::default()
    });
    let music = asset_server.load("music/08 Stage A.mp3");
    audio.play(music.clone());
    let hit_1 = asset_server.load("music/hit_1.mp3");
    commands.insert_resource(Sounds {
        song_1: music,
        hit_1,
    });
}

pub fn play_or_stop_music(
    mut audio: ResMut<Audio>,
    sounds: Res<Sounds>,
    interaction_q: Query<&Interaction>,
) {
    if let Some(Interaction::Clicked) = interaction_q.iter().next() {
        
        // Check if music is playing and either play or remove it
        println!("{:?}", audio.queue);
        let has_songs = audio.queue.read().iter().next().is_some();
        
        println!("Clicked on music button, currently songs: {}", has_songs);
        if has_songs {
            println!("Removed song");
            audio.queue.get_mut().pop_front();
        } else {
            println!("Playing song");
            audio.play(sounds.song_1.clone());
            let has_songs = audio.queue.read().iter().next().is_some();
        
            println!("Clicked on music button, currently songs: {}", has_songs);
            audio.queue.get_mut().pop_front();
            audio.queue.get_mut().pop_front();
        }
    }
}

pub fn sound_effects (
    mut reader: Local<EventReader<BallHitEvent>>,
    ball_hit_events: Res<Events<BallHitEvent>>, 
    audio: Res<Audio>,
    sounds: Res<Sounds>,
) {
    if let Some(_) = reader.iter(&ball_hit_events).next() {
        println!("Playing ball hit sound");
        audio.play(sounds.hit_1.clone());
    }
}