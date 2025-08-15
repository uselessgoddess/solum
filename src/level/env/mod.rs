use crate::{
  actors::Enemy,
  level::{Agent, Target},
  prelude::*,
};

pub fn plugin(app: &mut App) {
  app.add_systems(Update, spawn_enemies.in_set(PausableSystems));
}

#[derive(Component)]
#[require(Visibility, Transform)]
pub struct Environment {
  timer: Timer,
}

impl Default for Environment {
  fn default() -> Self {
    Self { timer: Timer::from_seconds(2.0, TimerMode::Repeating) }
  }
}

fn spawn_enemies(
  mut query: Query<(Entity, &mut Environment)>,
  mut commands: Commands,
  time: Res<Time>,
) {
  for (entity, mut env) in query.iter_mut() {
    if env.timer.tick(time.delta()).just_finished() {
      let transform = Transform2D::from_xy(
        rand::random_range(-500.0..500.0),
        rand::random_range(-500.0..500.0),
      );
      commands.entity(entity).with_child((
        Name::new("Enemy"),
        Enemy,
        transform,
      ));
    }
  }
}
