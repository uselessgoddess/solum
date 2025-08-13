pub mod brain;

use crate::{level::tilemap, prelude::*};

pub fn plugin(app: &mut App) {
  app.add_plugins(brain::plugin);
  app.add_systems(
    Update,
    spawn.in_set(PausableSystems).in_set(AppSystems::Spawn),
  );
}

#[derive(Component, Reflect)]
#[require(Stats)]
pub struct Enemy;

#[derive(Component, Reflect)]
pub struct Stats {
  pub speed: f32,
}

impl Default for Stats {
  fn default() -> Self {
    Self { speed: 3.0 * tilemap::METER }
  }
}

fn spawn(
  query: Query<(Entity, &Enemy), Added<Enemy>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut commands: Commands,
) {
  let radius = 0.33 * tilemap::METER;

  for (entity, _enemy) in query.iter() {
    let mesh = meshes.add(Circle::new(radius));
    let material = materials.add(Color::srgb(0.3, 0.2, 0.1));

    commands.entity(entity).insert((
      Controller::default(),
      Collider::circle(radius),
      YSort::default(),
      Mesh2d(mesh),
      MeshMaterial2d(material),
    ));
  }
}
