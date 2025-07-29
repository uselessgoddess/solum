use crate::{level::Speed, prelude::*};

pub fn plugin(app: &mut App) {
  app
    .add_plugins(InputManagerPlugin::<Action>::default())
    .add_systems(Update, (spawn, input));
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
#[actionlike(DualAxis)]
enum Action {
  Move,
  LookAround,
}

#[derive(Component)]
#[require(Speed)]
pub struct Player {}

fn spawn(
  query: Query<(Entity, &Player), Added<Player>>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  let radius = 0.5;

  for (entity, _player) in query.iter() {
    let mesh = meshes.add(Circle::new(radius));
    let material = materials.add(Color::srgb(0.1, 0.1, 0.1));
    commands
      .entity(entity)
      .insert((RigidBody::Kinematic, Collider::circle(radius)))
      .insert((Mesh2d(mesh), MeshMaterial2d(material)))
      .insert(
        InputMap::default()
          .with_dual_axis(Action::Move, VirtualDPad::wasd())
          .with_dual_axis(Action::LookAround, MouseMove::default()),
      );
  }
}

fn input(
  mut query: Query<
    (&ActionState<Action>, &Speed, &mut LinearVelocity),
    With<Player>,
  >,
) {
  for (state, Speed(speed), mut velocity) in query.iter_mut() {
    velocity.0 = state.axis_pair(&Action::Move).normalize_or_zero() * speed;
  }
}
