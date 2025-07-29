mod enemy;
mod player;

use crate::prelude::*;

pub use {enemy::Enemy, player::Player};

pub fn plugin(app: &mut App) {
  app.add_plugins((player::plugin, enemy::plugin));
}

#[derive(Component)]
pub struct Speed(pub f32);

impl Default for Speed {
  fn default() -> Self {
    Speed(1.0)
  }
}
