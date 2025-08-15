use {
  super::Pipeline,
  crate::{
    level::{Path, Target},
    prelude::*,
  },
};

pub fn plugin(app: &mut App) {
  app.add_systems(FixedUpdate, move_agent.in_set(Pipeline::Agents));
}

#[derive(Component, Reflect)]
#[require(Controller)]
pub struct Agent {
  pub speed: f32,
  pub nudge: f32,
}

impl Default for Agent {
  fn default() -> Self {
    Self { speed: 1.0, nudge: 60.0 }
  }
}

impl Agent {
  pub fn new(speed: f32) -> Self {
    Self { speed, ..default() }
  }
}

pub fn move_agent(
  mut navigator: Query<(
    Entity,
    &Agent,
    &Transform2D,
    &mut Controller,
    &mut Path,
  )>,
  mut commands: Commands,
  time: Res<Time<Fixed>>,
) {
  for (entity, agent, transform, mut controller, mut path) in
    navigator.iter_mut()
  {
    let Some(mut edge) = path.peek() else {
      continue;
    };
    let movement = edge - transform.translation;
    let _ = controller
      .slide(movement.normalize_or_zero() * agent.speed * time.delta_secs());

    while transform.translation.distance(edge) < agent.speed / agent.nudge {
      if let Some(next) = path.pop() {
        edge = next
      } else {
        // fixme!> we should remove target, to avoid empty paths
        commands.entity(entity).remove::<Target>();
        break;
      }
    }
  }
}
