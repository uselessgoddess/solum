pub mod enemy;
pub mod player;

use crate::prelude::*;

pub use {enemy::Enemy, player::Player};

pub fn plugin(app: &mut App) {
  app.add_plugins((player::plugin, enemy::plugin));
}
