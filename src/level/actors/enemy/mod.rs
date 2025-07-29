use crate::{level::Speed, prelude::*};

pub fn plugin(app: &mut App) {
  app.add_systems(Update, spawn);
}

#[derive(Component)]
#[require(Speed)]
pub struct Enemy {}

fn spawn(
  query: Query<(Entity, &Enemy), Added<Enemy>>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  let radius = 0.5;

  for (entity, _player) in query.iter() {
    let mesh = meshes.add(Circle::new(radius));
    let material = materials.add(Color::srgb(0.5, 0.1, 0.1));
    commands
      .entity(entity)
      .insert((RigidBody::Kinematic, Collider::circle(radius)))
      .insert((Mesh2d(mesh), MeshMaterial2d(material)));
  }
}
