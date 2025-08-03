mod asset_tracking;
pub mod audio;
mod debug;
mod dev;
pub mod ecs;
mod physics;
mod system;

use crate::prelude::*;

pub use {
  asset_tracking::{LoadResource, ResourceHandles},
  audio::{Music, SoundEffect, music, sound_effect},
  ecs::{PausableSystems, Pause, Transform2D},
};

pub fn plugin(app: &mut App) {
  app.add_plugins((
    system::plugin,
    ecs::plugin,
    physics::plugin,
    asset_tracking::plugin,
    audio::plugin,
  ));

  if debug::dev() {
    app.add_plugins(dev::plugin);
  }
}

#[derive(Component)]
#[require(Camera2d)]
pub struct PrimaryCamera;
