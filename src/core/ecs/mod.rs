mod marker;
pub mod sensor;
pub(crate) mod spawn;

use crate::prelude::*;

pub use {
  marker::marker,
  sensor::{Affect, AppExt, Sensor, effect},
  spawn::Build,
};

pub fn plugin(app: &mut App) {
  app.add_plugins(sensor::plugin);
}

pub trait TriggerExt<E> {
  fn read_event(&self) -> (Entity, E);
}

impl<E: Event + Clone> TriggerExt<E> for Trigger<'_, E> {
  fn read_event(&self) -> (Entity, E) {
    (self.target(), self.event().clone())
  }
}
