mod enemy;
pub mod player;

use crate::prelude::*;

pub use player::{Player, Stats};

pub fn plugin(app: &mut App) {
  app.add_plugins((player::plugin, enemy::plugin));
}
