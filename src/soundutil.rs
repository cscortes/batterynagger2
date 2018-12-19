extern crate music;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Music {
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Sound {
    Ding,
    FatalMesg,
}

pub fn play_ding()  {
    // music::set_volume(music::MAX_VOLUME);
    music::bind_sound_file(Sound::Ding, "assets/ping.mp3");
    music::play_sound(&Sound::Ding, music::Repeat::Times(0), music::MAX_VOLUME);
}

pub fn play_fatal()  {
    // music::set_volume(music::MAX_VOLUME);
    music::bind_sound_file(Sound::FatalMesg, "assets/low_bat.mp3");
    music::play_sound(&Sound::FatalMesg, music::Repeat::Times(0), music::MAX_VOLUME);
}
