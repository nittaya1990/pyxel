mod audio;
mod canvas;
mod channel;
mod event;
mod graphics;
mod image;
mod input;
mod key;
mod music;
mod oscillator;
mod platform;
mod profiler;
mod rectarea;
mod resource;
mod sdl2;
mod settings;
mod sound;
mod system;
mod tilemap;
mod types;
mod utility;

use array_macro::array;

use crate::audio::Audio;
use crate::graphics::Graphics;
use crate::input::Input;
use crate::platform::Platform;
use crate::resource::Resource;
use crate::sdl2::Sdl2;
use crate::system::System;

pub use crate::canvas::Canvas;
pub use crate::channel::Channel;
pub use crate::image::{Image, SharedImage};
pub use crate::key::*;
pub use crate::music::{Music, SharedMusic};
pub use crate::settings::*;
pub use crate::sound::{SharedSound, Sound};
pub use crate::tilemap::{SharedTilemap, Tilemap};
pub use crate::types::*;

pub type TargetPlatform = Sdl2;

pub struct Pyxel {
    platform: TargetPlatform,
    system: System,
    resource: Resource,
    input: Input,
    graphics: Graphics,
    audio: Audio,

    pub colors: [Rgb8; COLOR_COUNT as usize],
    pub palette: [Color; COLOR_COUNT as usize],
    pub screen: SharedImage,
    pub cursor: SharedImage,
    pub font: SharedImage,
}

pub trait PyxelCallback {
    fn update(&mut self, pyxel: &mut Pyxel);
    fn draw(&mut self, pyxel: &mut Pyxel);
}

impl Pyxel {
    pub fn new(
        width: u32,
        height: u32,
        title: Option<&str>,
        scale: Option<u32>,
        fps: Option<u32>,
        quit_key: Option<Key>,
    ) -> Pyxel {
        let title = title.unwrap_or(DEFAULT_TITLE);
        let scale = scale.unwrap_or(DEFAULT_SCALE);
        let fps = fps.unwrap_or(DEFAULT_FPS);
        let quit_key = quit_key.unwrap_or(DEFAULT_QUIT_KEY);

        let mut platform = TargetPlatform::new(title, width, height, scale);
        let system = System::new(fps, quit_key);
        let resource = Resource::new(width, height);
        let input = Input::new();
        let graphics = Graphics::new();
        let audio = Audio::new(&mut platform);

        let colors = DEFAULT_COLORS.clone();
        let palette = array![i => i as Color; COLOR_COUNT as usize];
        let screen = Image::new(width, height);
        let cursor = Graphics::new_cursor_image();
        let font = Graphics::new_font_image();

        Pyxel {
            platform: platform,
            system: system,
            resource: resource,
            input: input,
            graphics: graphics,
            audio: audio,

            colors: colors,
            palette: palette,
            screen: screen,
            cursor: cursor,
            font: font,
        }
    }
}
