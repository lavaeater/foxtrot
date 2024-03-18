use crate::{file_system_interaction::asset_loading::AudioAssets, GameState};
use bevy::prelude::*;
use bevy_kira_audio::prelude::{Audio, *};
use rusty_music::{create_arpeggiator, create_bassist, create_drummer, create_soloist, generate_chords, MusicPlugin};
use rusty_music::clock::{Beat, Clock, progress_clock_system};
use rusty_music::musicians::conductor::Conductor;
use rusty_music::musicians::drummer::{generate_hihat_beat, generate_kick_beat, generate_snare_beat};
use rusty_music::player::{Intensity, play_sound_on_the_beat};

/// Handles initialization of all sounds.
pub(super) fn plugin(app: &mut App) {
    app
        .add_plugins(AudioPlugin)
        .add_systems(OnExit(GameState::Loading), (init_audio, init_music))
        .insert_resource(Clock::new(4,4, 80.0))
        .insert_resource(Intensity(0.5))
        .add_event::<Beat>()
        .add_systems(Update, (
            progress_clock_system,
            play_sound_on_the_beat).run_if(in_state(GameState::Playing)))
    ;
}

#[derive(Debug, Clone, Resource)]
pub(crate) struct AudioHandles {
    pub(crate) walking: Handle<AudioInstance>,
}

fn init_audio(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.pause();
    let handle = audio
        .play(audio_assets.walking.clone())
        .looped()
        .with_volume(0.8)
        .handle();
    commands.insert_resource(AudioHandles { walking: handle });
}


fn init_music(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>
) {
    commands.spawn(
        create_soloist("Melody".to_string(),
                       audio_assets.acid_short_c.clone(),
                       0.251188643150958,
                       2,
                       4,
                       2));
    commands.spawn(
        create_arpeggiator("Arpeggio".to_string(), audio_assets.acid_long_c.clone(), 0.251188643150958));

    commands.spawn(
        create_bassist("Bassist".to_string(), audio_assets.bass_c.clone(), 0.7));

    commands.spawn(
        create_drummer("Kick".to_string(), audio_assets.drum_kick.clone(), 1.0, generate_kick_beat())
    );
    commands.spawn(
        create_drummer("Snare".to_string(), audio_assets.drum_snare.clone(), 1.0, generate_snare_beat())
    );


    commands.spawn(create_drummer("Hihat".to_string(), audio_assets.drum_hihat.clone(), 1.0, generate_hihat_beat()));

    commands.insert_resource(Conductor {
        chords: generate_chords()
    });
}
