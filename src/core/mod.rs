mod dev;
mod ecs;
mod ext;
pub(crate) mod lens;
mod system;
mod timer;
mod transform;
mod utils;

pub use {
  ecs::{
    Affect, AppExt as _, Sensor, TriggerExt as _, marker,
    spawn::CommandsExt as _,
  },
  transform::Transform2D,
  utils::type_name,
};

pub use crate::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins((
        system::plugin,
        transform::plugin, // todo!: move to `ecs` mod
        ecs::plugin,
        lens::plugin,
      ))
      .add_plugins(PhysicsPlugins::default());

    app.insert_resource(SubstepCount(1));
    app.insert_resource(Gravity::ZERO);

    if debug::dev() {
      app.add_plugins(dev::plugin);
    }
  }
}

#[derive(Component)]
#[require(Camera2d)]
pub struct PrimaryCamera;
