use std::cmp::max;

use blip_buf::BlipBuf;

use crate::oscillator::Oscillator;
use crate::settings::{EFFECT_NONE, MAX_EFFECT, MAX_NOTE, MAX_TONE, MAX_VOLUME, TONE_TRIANGLE};
use crate::sound::Sound;
use crate::types::{Effect, Note, Tone, Volume};

pub struct Channel {
    oscillator: Oscillator,
    sounds: Vec<Sound>,
    is_playing: bool,
    is_looping: bool,
    sound_index: u32,
    note_index: u32,
    tick_count: u32,
    pub volume: Volume,
}

impl Channel {
    pub fn new() -> Channel {
        Channel {
            oscillator: Oscillator::new(),
            sounds: Vec::new(),
            is_playing: false,
            is_looping: false,
            sound_index: 0,
            note_index: 0,
            tick_count: 0,
            volume: MAX_VOLUME,
        }
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn is_looping(&self) -> bool {
        self.is_looping
    }

    pub fn sound_index(&self) -> u32 {
        self.sound_index
    }

    pub fn note_index(&self) -> u32 {
        self.note_index
    }

    pub fn play(&mut self, sounds: Vec<Sound>, is_looping: bool) {
        self.sounds = sounds;
        self.is_playing = true;
        self.is_looping = is_looping;
        self.sound_index = 0;
        self.note_index = 0;
        self.tick_count = 0;
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
        self.is_looping = false;
        self.sound_index = 0;
        self.note_index = 0;

        self.oscillator.stop();
    }

    pub fn update(&mut self, blip_buf: &mut BlipBuf) {
        if !self.is_playing {
            return;
        }

        let sound = &self.sounds[self.sound_index as usize];
        let speed = max(sound.speed, 1);

        if self.tick_count % speed as u32 == 0 {
            if self.note_index >= sound.notes.len() as u32 {
                self.sound_index += 1;
                self.note_index = 0;

                if self.sound_index >= self.sounds.len() as u32 {
                    if self.is_looping {
                        self.sound_index = 0;
                    } else {
                        self.stop();
                        return;
                    }
                }
            }

            let sound = &self.sounds[self.sound_index as usize];
            let note = Channel::circular_note(&sound.notes, self.note_index);
            let tone = Channel::circular_tone(&sound.tones, self.note_index);
            let volume = Channel::circular_volume(&sound.volumes, self.note_index);
            let effect = Channel::circular_effect(&sound.effects, self.note_index);
            let speed = max(sound.speed, 1);

            assert!(note <= MAX_NOTE, "invalid sound note {}", note);
            assert!(tone <= MAX_TONE, "invalid sound tone {}", tone);
            assert!(volume <= MAX_VOLUME, "invalid sound volume {}", volume);
            assert!(
                self.volume <= MAX_VOLUME,
                "invalid channel volume {}",
                self.volume
            );
            assert!(effect <= MAX_EFFECT, "invalid sound effect {}", effect);

            if note >= 0 && volume > 0 {
                self.oscillator.play(
                    note as f64,
                    tone,
                    (volume as f64 / MAX_VOLUME as f64) * (self.volume as f64 / MAX_VOLUME as f64),
                    effect,
                    speed as u32,
                );
            }

            self.note_index += 1;
        }

        self.oscillator.update(blip_buf);
        self.tick_count += 1;
    }

    fn circular_note(notes: &[Note], index: u32) -> Note {
        let len = notes.len();

        if len > 0 {
            notes[index as usize % len]
        } else {
            0
        }
    }

    fn circular_tone(tones: &[Tone], index: u32) -> Tone {
        let len = tones.len();

        if len > 0 {
            tones[index as usize % len]
        } else {
            TONE_TRIANGLE
        }
    }

    fn circular_volume(volumes: &[Volume], index: u32) -> Volume {
        let len = volumes.len();

        if len > 0 {
            volumes[index as usize % len]
        } else {
            MAX_VOLUME
        }
    }

    fn circular_effect(effects: &[Effect], index: u32) -> Effect {
        let len = effects.len();

        if len > 0 {
            effects[index as usize % len]
        } else {
            EFFECT_NONE
        }
    }
}
