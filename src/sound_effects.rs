use macroquad::audio::{load_sound, play_sound, PlaySoundParams, Sound};

pub struct AudioSystem {
    sfx_tetris: Sound,
    sfx_anvil: Sound,
    sfx_hold: Sound,
    sfx_same: Sound,
    sfx_diff: Sound,
    bgm: Sound,
}

impl AudioSystem {
    pub async fn new() -> Self {
        let sfx_hold = load_sound("src/assets/SE/onHold.wav")
            .await
            .expect("Failed to load onHold.wav");
        let sfx_same = load_sound("src/assets/SE/sameColor.wav")
            .await
            .expect("Failed to load sameColor.wav");
        let sfx_diff = load_sound("src/assets/SE/notSameColor.wav")
            .await
            .expect("Failed to load notSameColor.wav");
        let sfx_anvil = load_sound("src/assets/SE/anvill.wav")
            .await
            .expect("Failed to load anvill.wav");

        // Load once, clone for both uses
        let music_sound = load_sound("src/assets/SE/always.ogg")
            .await
            .expect("Failed to load always.ogg");

        let sfx_tetris = sfx_same.clone(); // Reuse same color sound for now, NOT the music!
        let bgm = music_sound;

        play_sound(
            &bgm,
            PlaySoundParams {
                looped: true,
                volume: 0.5,
            },
        );

        Self {
            sfx_tetris,
            sfx_anvil,
            sfx_hold,
            sfx_same,
            sfx_diff,
            bgm,
        }
    }

    pub fn toggle_music(&self, is_playing: bool) {
        // Macroquad doesn't have a direct "stop" or "pause" on the Sound handle easily accessible
        // without keeping track of the playback instance, but we can just set volume for now
        // or stop/start if we want to restart.
        // Actually, play_sound sends a "fire and forget" or returns a handle depending on API version.
        // The standard macroquad::audio::play_sound returns void.
        // To control volume, we might need `stop_sound` or just reissue play with 0 volume?
        // Wait, macroquad 0.4 has `stop_sound`.

        if is_playing {
            play_sound(
                &self.bgm,
                PlaySoundParams {
                    looped: true,
                    volume: 0.5,
                },
            );
        } else {
            macroquad::audio::stop_sound(&self.bgm);
        }
    }

    pub fn play_hold(&self) {
        play_sound(
            &self.sfx_hold,
            PlaySoundParams {
                looped: false,
                volume: 1.0,
            },
        );
    }

    pub fn play_land(&self, same_color: bool, diff_color: bool) {
        if same_color {
            play_sound(
                &self.sfx_same,
                PlaySoundParams {
                    looped: false,
                    volume: 1.0,
                },
            );
        }
        if diff_color {
            play_sound(
                &self.sfx_diff,
                PlaySoundParams {
                    looped: false,
                    volume: 1.0,
                },
            );
        }
    }

    pub fn play_tetris(&self) {
        play_sound(
            &self.sfx_tetris,
            PlaySoundParams {
                looped: false,
                volume: 1.0,
            },
        );
    }

    pub fn play_level_up(&self) {
        // Reuse hold sound for level up for now as a nice chime
        play_sound(
            &self.sfx_hold,
            PlaySoundParams {
                looped: false,
                volume: 1.0,
            },
        );
    }

    pub fn play_anvil(&self) {
        play_sound(
            &self.sfx_anvil,
            PlaySoundParams {
                looped: false,
                volume: 1.0,
            },
        );
    }
}

// Implement Clone to allow Game restart logic (Sound is a handle, so cheap clone)
impl Clone for AudioSystem {
    fn clone(&self) -> Self {
        Self {
            sfx_tetris: self.sfx_tetris.clone(),
            sfx_anvil: self.sfx_anvil.clone(),

            sfx_hold: self.sfx_hold.clone(),
            sfx_same: self.sfx_same.clone(),
            sfx_diff: self.sfx_diff.clone(),
            bgm: self.bgm.clone(),
        }
    }
}
