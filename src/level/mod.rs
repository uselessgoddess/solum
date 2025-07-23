mod actors;
mod camera;

use crate::prelude::*;

pub use actors::Planet;

pub fn plugin(app: &mut App) {
  app.add_plugins((actors::plugin, camera::plugin));
}
