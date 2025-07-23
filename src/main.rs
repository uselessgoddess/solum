use solum::{level::Planet, prelude::*};

fn main() {
  App::new()
    .insert_resource(D::L3)
    .add_plugins(GamePlugin)
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands) {
  commands.spawn(PrimaryCamera);

  commands.spawn(Planet { radius: 100.0, mass: 0.0 });
}
