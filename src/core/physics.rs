use bevy::ecs::schedule::{InternedScheduleLabel, ScheduleLabel};

use crate::prelude::*;

pub const PIXELS_PER_METER: f32 = 32.0;

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

  app.add_plugins(ControllerPlugin::new(FixedUpdate));
}

pub struct ControllerPlugin {
  schedule: InternedScheduleLabel,
}

impl ControllerPlugin {
  pub fn new(schedule: impl ScheduleLabel) -> Self {
    Self { schedule: schedule.intern() }
  }
}

impl Plugin for ControllerPlugin {
  fn build(&self, app: &mut App) {
    app.configure_sets(
      self.schedule,
      (Pipeline::Sensors, ControllerInputsSet, Pipeline::Apply)
        .chain()
        .in_set(ControllerSystemSet),
    );
    app.configure_sets(
      self.schedule,
      ControllerSystemSet
        .before(PhysicsStepSet::First)
        .run_if(|time: Res<Time<Physics>>| !time.is_paused()),
    );
    app.add_systems(self.schedule, apply.in_set(Pipeline::Apply));
  }
}

#[derive(
  SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord,
)]
pub enum Pipeline {
  Sensors,
  Apply,
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ControllerSystemSet;

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ControllerInputsSet;

// todo!> find better name
#[derive(Component, Default)]
#[require(RigidBody::Dynamic, LockedAxes::ROTATION_LOCKED, LinearVelocity)]
pub struct Controller {
  desired: Option<Vec2>,
}

impl Controller {
  pub fn desire(&mut self, warp: Vec2) {
    if let Some(desired) = &mut self.desired {
      *desired += warp;
    } else {
      self.desired = Some(warp);
    }
  }
}

fn apply(mut query: Query<(&mut Controller, &mut LinearVelocity)>) {
  for (mut controller, mut velocity) in query.iter_mut() {
    velocity.0 = controller.desired.take().unwrap_or(Vec2::ZERO);
  }
}
