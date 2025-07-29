mod actors;
mod camera;
mod follow;
pub mod marker;

use crate::prelude::*;

pub use {
  actors::{Player, Speed, Enemy},
  follow::{Damp, Follow, FollowMouse},
};

pub fn plugin(app: &mut App) {
  app.add_plugins((
    actors::plugin,
    camera::plugin,
    marker::plugin,
    follow::plugin,
  ));
}
