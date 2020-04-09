use amethyst::{
    assets::{AssetStorage, Loader},
    audio::{output::Output, OggFormat, Source, SourceHandle},
    ecs::prelude::{World, WorldExt},
};

const AUDIO_BOUNCE: &'static str = "audio/bounce.ogg";
const AUDIO_SCORE: &'static str = "audio/score.ogg";

pub struct Sounds {
    pub score: SourceHandle,
    pub bounce: SourceHandle,
}

pub fn initialize_audio(world: &mut World) {
    let sounds = {
        let loader = world.read_resource::<Loader>();

        Sounds {
            bounce: load_audio(&loader, &world, AUDIO_BOUNCE),
            score: load_audio(&loader, &world, AUDIO_SCORE),
        }
    };

    world.insert(sounds);
}

fn load_audio(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), &world.read_resource())
}

pub fn play_sound(sounds: &SourceHandle, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(output) = output {
        if let Some(sound) = storage.get(sounds) {
            output.play_once(sound, 1.0);
        }
    }
}
