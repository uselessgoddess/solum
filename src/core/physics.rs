use crate::prelude::*;

pub const PIXELS_PER_METER: f32 = 16.0;

pub fn plugin(app: &mut App) {
  app
    .add_plugins(PhysicsPlugins::default().with_length_unit(PIXELS_PER_METER))
    .insert_resource(Gravity::ZERO)
    .insert_resource(DefaultFriction(Friction::ZERO));

  app
    .add_systems(OnEnter(Pause(true)), |mut time: ResMut<Time<Physics>>| {
      time.pause();
    })
    .add_systems(OnEnter(Pause(false)), |mut time: ResMut<Time<Physics>>| {
      time.unpause();
    });
}
