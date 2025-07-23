mod planet;

use crate::prelude::*;

pub use planet::Planet;

pub fn plugin(app: &mut App) {
  app.add_plugins(planet::plugin);
}
