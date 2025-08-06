mod camera;
pub mod tilemap;

use crate::{actors::Player, prelude::*};

pub use tilemap::{Storage, Tilemap};

pub fn plugin(app: &mut App) {
  app.register_type::<LevelAssets>();
  app.configure_loading_state(
    LoadingStateConfig::new(Game::Loading)
      .load_collection::<LevelAssets>()
      .load_collection::<TilesAssets>(),
  );

  app.add_plugins((camera::plugin, tilemap::plugin));
}

pub fn spawn_level(mut commands: Commands, level_assets: Res<LevelAssets>) {
  commands.spawn((
    Name::new("Level"),
    Transform::default(),
    Visibility::default(),
    StateScoped(Game::Gameplay),
    children![
      (Name::new("Player"), Player),
      (
        Name::new("Tilemap"),
        Tilemap {
          size: TilemapSize { x: 32, y: 32 },
          map_type: TilemapType::Square,
          tile_size: TilemapTileSize { x: tilemap::TILE, y: tilemap::TILE },
        },
        Transform2D::layer(BACKGROUND_OFFSET),
      ),
      (Name::new("Gameplay Music"), music(level_assets.music.clone()))
    ],
  ));
}
