use solum::{level::Speed, prelude::*};

fn main() {
  App::new()
    .insert_resource(D::L3)
    .add_plugins(GamePlugin)
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands) {
  let player = commands.spawn(level::Player {}).insert(Speed(10.0 / 3.6)).id();

  commands
    .spawn(PrimaryCamera)
    .insert(level::Damp::new(player).with_smooth(0.05));

  commands.spawn(level::Enemy {}).insert(Transform2D::from_xy(1.0, 1.0));
}
