#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::cargo_common_metadata,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::default_trait_access,
    clippy::fn_params_excessive_bools,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::multiple_crate_versions,
    clippy::must_use_candidate,
    clippy::pedantic,
    clippy::range_plus_one,
    clippy::suboptimal_flops,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::unreadable_literal,
    clippy::wildcard_dependencies,
    clippy::wildcard_imports
)]

#[macro_use]
mod utils;
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

use crate::audio::Audio;
pub use crate::channel::{Channel, SharedChannel};
use crate::graphics::Graphics;
pub use crate::image::{Image, SharedImage};
use crate::input::Input;
pub use crate::key::*;
pub use crate::music::{Music, SharedMusic};
use crate::platform::Platform;
use crate::resource::Resource;
use crate::sdl2::Sdl2;
pub use crate::settings::*;
pub use crate::sound::{SharedSound, Sound};
use crate::system::System;
pub use crate::tilemap::{SharedTilemap, Tilemap};
pub use crate::types::*;

type TargetPlatform = Sdl2;

pub struct Pyxel {
    platform: TargetPlatform,
    system: System,
    resource: Resource,
    input: Input,
    graphics: Graphics,
    audio: Audio,
    pub colors: [Rgb8; NUM_COLORS as usize],
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
        fps: Option<u32>,
        quit_key: Option<Key>,
        capture_sec: Option<u32>,
    ) -> Self {
        let title = title.unwrap_or(DEFAULT_TITLE);
        let fps = fps.unwrap_or(DEFAULT_FPS);
        let quit_key = quit_key.unwrap_or(DEFAULT_QUIT_KEY);
        let capture_sec = capture_sec.unwrap_or(DEFAULT_GIF_SEC);

        let mut platform = TargetPlatform::new(title, width, height, DISPLAY_RATIO);
        let system = System::new(fps, quit_key);
        let resource = Resource::new(width, height, fps, capture_sec);
        let input = Input::new();
        let graphics = Graphics::new();
        let audio = Audio::new(&mut platform);

        let colors = DEFAULT_COLORS;
        let screen = Image::new(width, height);
        let cursor = Graphics::new_cursor_image();
        let font = Graphics::new_font_image();

        let mut pyxel = Self {
            platform,
            system,
            resource,
            input,
            graphics,
            audio,
            colors,
            screen,
            cursor,
            font,
        };
        pyxel.icon(&ICON_DATA, ICON_SCALE);
        pyxel
    }
}
