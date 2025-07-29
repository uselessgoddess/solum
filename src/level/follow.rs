use {
  crate::prelude::*,
  bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    window::PrimaryWindow,
  },
};

pub fn plugin(app: &mut App) {
  app
    .register_type::<Follow>()
    .register_type::<FollowMouse>()
    .add_systems(Update, (follow, follow_mouse, damp));
}

#[derive(Component, Reflect, Copy, Clone)]
#[require(Transform)]
#[component(on_add = on_add)]
pub struct Follow(pub Entity);

fn on_add(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
  let Some(Follow(target)) = world.get(entity).copied() else { unreachable!() };

  if entity != target
    && let Some(global) = world.get::<GlobalTransform>(target).copied()
    && let Some(mut transform) = world.get_mut::<Transform2D>(entity)
  {
    *transform = global.compute_transform().into();
  }
}

fn follow(
  query: Query<(Entity, &Follow)>,
  targets: Query<&GlobalTransform>,
  mut commands: Commands,
) {
  for (entity, &Follow(target)) in query.iter() {
    if entity != target
      && let Ok(&global) = targets.get(target)
    {
      commands.entity(entity).try_insert(global.compute_transform());
    }
  }
}

#[derive(Component, Reflect, Default, Copy, Clone)]
#[require(Transform)]
pub struct FollowMouse;

fn follow_mouse(
  mut query: Query<&mut Transform2D, With<FollowMouse>>,
  window: Single<&Window, With<PrimaryWindow>>,
  q_camera: Single<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
) {
  let (camera, transform) = q_camera.into_inner();

  if let Some(cursor) = window.cursor_position()
    && let Ok(cursor) = camera.viewport_to_world_2d(transform, cursor)
  {
    for mut follow in query.iter_mut() {
      follow.translation = cursor;
    }
  }
}

#[derive(Component, Reflect, Copy, Clone)]
#[require(Transform)]
pub struct Damp {
  pub entity: Entity,
  pub smooth: f32,
}

impl Damp {
  pub fn new(entity: Entity) -> Self {
    Self { entity, smooth: 0.5 }
  }

  pub fn with_smooth(mut self, smooth: f32) -> Self {
    self.smooth = smooth;
    self
  }
}

fn damp(
  mut query: Query<(&Damp, &mut Transform2D)>,
  targets: Query<&Transform2D, Without<Damp>>,
  time: Res<Time>,
) {
  let delta = time.delta_secs();

  for (damp, mut global) in query.iter_mut() {
    if let Ok(&transform) = targets.get(damp.entity) {
      let (current, target) = (global.translation, transform.translation);

      let omega = 2.0 / damp.smooth;
      let x = omega * delta;
      let exp = 1.0 / (1.0 + x + 0.48 * x * x + 0.235 * x * x * x);

      global.translation = {
        let change = current - target;
        target + (change + change * omega * delta) * exp
      };
    }
  }
}
