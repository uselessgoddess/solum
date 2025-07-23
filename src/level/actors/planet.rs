use crate::prelude::*;

pub fn plugin(app: &mut App) {
  app.add_systems(Update, spawn);
}

#[derive(Component)]
pub struct Planet {
  pub radius: f32,
  pub mass: f32,
}

fn spawn(
  query: Query<(Entity, &Planet), Added<Planet>>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  for (entity, planet) in query.iter() {
    let mesh = meshes.add(Circle::new(planet.radius));
    let material = materials.add(Color::srgb(0.8, 0.8, 0.8));

    commands.entity(entity).insert((Mesh2d(mesh), MeshMaterial2d(material)));
  }
}
