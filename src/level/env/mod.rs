use crate::{
  actors::{Brick, Enemy},
  prelude::*,
};

pub fn plugin(app: &mut App) {
  app.add_systems(
    Update,
    (spawn.in_set(AppSystems::Spawn), spawn_enemies.in_set(PausableSystems)),
  );
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

fn spawn(query: Query<Entity, Added<Environment>>, mut commands: Commands) {
  for entity in query.iter() {
    for _ in 0..128 {
      let (x, y) = (rand::random_range(0..64), rand::random_range(0..64));
      commands.entity(entity).with_child((
        Name::new("Brick"),
        Brick,
        TilePos::new(x, y),
      ));
    }
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
