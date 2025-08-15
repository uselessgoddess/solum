mod camera;
pub mod env;
pub mod nav;
pub mod tilemap;

use crate::{
  actors::{Enemy, Player, enemy},
  prelude::*,
};

pub use {
  env::Environment,
  nav::{Agent, NavSystemSet, Path, Target},
  tilemap::{Storage, Tilemap},
};

pub fn plugin(app: &mut App) {
  app.register_type::<LevelAssets>();
  app.configure_loading_state(
    LoadingStateConfig::new(Game::Loading)
      .load_collection::<LevelAssets>()
      .load_collection::<TilesAssets>(),
  );

  app.add_plugins((camera::plugin, tilemap::plugin, nav::plugin, env::plugin));
  app.add_plugins(NavmeshUpdaterPlugin::<Collider, Obstacle>::default());
}

#[derive(Component, Default, Copy, Clone)]
pub struct Obstacle;

#[derive(Component)]
#[require(Visibility, Transform)]
pub struct Level {
  /// `NavMesh` of level
  pub navmesh: Entity,
}

pub fn spawn_level(mut commands: Commands, level_assets: Res<LevelAssets>) {
  let settings = NavMeshSettings {
    fixed: Triangulation::from_outer_edges(&[
      vec2(-500.0, -500.0),
      vec2(500.0, -500.0),
      vec2(500.0, 500.0),
      vec2(-500.0, 500.0),
    ]),
    agent_radius: 1.0 * tilemap::METER,
    simplify: 4.0,
    merge_steps: 1,
    ..default()
  };
  let navmesh = commands
    .spawn((Name::new("Navmesh"), settings, NavMeshUpdateMode::Direct))
    .id();

  commands
    .spawn((Name::new("Level"), StateScoped(Game::Gameplay), Level { navmesh }))
    .insert(children![
      (Name::new("Player"), Player, Obstacle),
      (Name::new("Env"), Environment::default()),
      (
        Name::new("Tilemap"),
        Tilemap {
          size: TilemapSize { x: 64, y: 64 },
          map_type: TilemapType::Square,
          tile_size: TilemapTileSize { x: tilemap::TILE, y: tilemap::TILE },
          anchor: TilemapAnchor::Center,
        },
        Transform2D::layer(BACKGROUND_OFFSET),
      ),
      // (Name::new("Gameplay Music"), music(level_assets.music.clone()))
    ])
    .add_child(navmesh);
}
