use crate::actions::{set_movement_actions, Actions};
use crate::loading::AudioAssets;
use crate::AppState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_system_set(SystemSet::on_enter(AppState::Playing).with_system(start_audio))
            .add_system_set(
                SystemSet::on_update(AppState::Playing)
                    .with_system(control_hit_sound.after(set_movement_actions)),
            );
    }
}

#[derive(Resource)]
struct FlyingAudio(Handle<AudioInstance>);

fn start_audio(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.pause();
    let handle = audio
        .play(audio_assets.flying.clone())
        .looped()
        .with_volume(0.3)
        .handle();
    commands.insert_resource(FlyingAudio(handle));
}

fn control_hit_sound(
    actions: Res<Actions>,
    audio: Res<FlyingAudio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(instance) = audio_instances.get_mut(&audio.0) {
        match instance.state() {
            PlaybackState::Paused { .. } => {
                if actions.player_movement.is_some() {
                    instance.resume(AudioTween::default());
                }
            }
            PlaybackState::Playing { .. } => {
                if actions.player_movement.is_none() {
                    instance.pause(AudioTween::default());
                }
            }
            _ => {}
        }
    }
}
