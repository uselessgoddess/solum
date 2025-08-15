mod agent;

use crate::{level::Level, prelude::*};

pub use agent::Agent;

pub fn plugin(app: &mut App) {
  app
    .configure_sets(FixedUpdate, NavSystemSet.in_set(ControllerInputsSet))
    .configure_sets(
      FixedUpdate,
      (Pipeline::Refresh, Pipeline::Agents).chain().in_set(NavSystemSet),
    );

  app.register_type::<Path>().register_type::<Target>();

  app
    .add_plugins(agent::plugin)
    .add_systems(FixedUpdate, refresh_path.in_set(Pipeline::Refresh))
    .add_systems(Update, debug_path.run_if(in_debug(D::L2)));
}

#[derive(
  SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord,
)]
pub enum Pipeline {
  Refresh,
  Agents,
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct NavSystemSet;

#[derive(Component, Reflect)]
#[require(Path = Path::new_empty())]
pub struct Target {
  position: Vec2,
}

impl Target {
  pub fn new(position: Vec2) -> Self {
    Self { position }
  }
}

#[derive(Component, Reflect, Debug)]
#[component(storage = "SparseSet")]
pub struct Path {
  seq: VecDeque<Vec2>,
}

impl Path {
  pub fn new_empty() -> Self {
    Self { seq: VecDeque::new() }
  }

  pub fn new(seq: VecDeque<Vec2>) -> Self {
    debug_assert!(
      !seq.is_empty(),
      "use `Path::new_empty` to manually create empty path"
    );
    Self { seq }
  }

  pub fn clear(&mut self) {
    self.seq.clear();
  }

  pub fn len(&self) -> usize {
    self.seq.len()
  }

  pub fn is_empty(&self) -> bool {
    self.seq.is_empty()
  }

  pub fn peek(&self) -> Option<Vec2> {
    self.seq.front().cloned()
  }

  pub fn pop(&mut self) -> Option<Vec2> {
    self.seq.pop_front()
  }

  pub fn segments(&self) -> impl Iterator<Item = (Vec2, Vec2)> {
    self.seq.iter().map_windows(|[a, b]| (**a, **b))
  }
}

pub fn refresh_path(
  mut navigator: Query<(Entity, &Transform, &mut Target, &mut Path)>,
  (level, navmesh): (Single<&Level>, Query<(&ManagedNavMesh, &NavMeshStatus)>),
  mut navmeshes: ResMut<Assets<NavMesh>>,
  mut commands: Commands,
  mut delta: Local<f32>,
) {
  let Ok((handle, status)) = navmesh.get(level.navmesh) else { todo!() };
  let Some(navmesh) = navmeshes.get_mut(handle.id()) else { return };
  if *status != NavMeshStatus::Built && *delta == 0.0 {
    return;
  }

  for (entity, transform, target, mut path) in navigator.iter_mut() {
    let target = target.position.extend(0.0);
    if !navmesh.transformed_is_in_mesh(transform.translation) {
      *delta += 0.1;
      navmesh.set_search_delta(*delta);
      continue;
    } else if !navmesh.transformed_is_in_mesh(target) {
      commands.entity(entity).remove::<Target>();
      continue;
    }

    if let Some(new) = navmesh.transformed_path(transform.translation, target)
      && !new.path.is_empty()
    {
      *path = Path::new(new.path.into_iter().map(|vec| vec.xy()).collect());
      *delta = 0.0;
    } else {
      commands.entity(entity).remove::<Target>();
    };
  }
}

// todo!> use custom configurable Gizmos wrapper
fn debug_path(query: Query<(&Transform2D, &Path)>, mut gizmos: Gizmos) {
  let mut debug = |from, to| {
    // todo!> use custom color scheme
    gizmos.line_2d(from, to, Color::BLACK);
  };

  for (transform, path) in query.iter() {
    for (a, b) in path.segments() {
      debug(a, b);
    }
    if let Some(first) = path.peek() {
      debug(first, transform.translation);
    }
  }
}
