use crate::{level::TilesAssets, prelude::*};

pub const TILE: f32 = 32.0;

pub fn plugin(app: &mut App) {
  app.add_systems(
    Update,
    spawn.in_set(PausableSystems).in_set(AppSystems::Spawn),
  );
}

#[derive(Component)]
pub struct Tilemap {
  pub size: TilemapSize,
  pub map_type: TilemapType,
  pub tile_size: TilemapTileSize,
}

#[derive(Component, Deref, DerefMut)]
pub struct Storage {
  pub entity: Entity,
  pub translation: Vec2,
  #[deref]
  pub storage: TileStorage,
}

pub fn spawn(
  tilemap: Single<(Entity, &Tilemap), Added<Tilemap>>,
  tiles: Res<TilesAssets>,
  mut commands: Commands,
) {
  let (tilemap_entity, &Tilemap { size, map_type, tile_size, .. }) = *tilemap;

  let mut storage = TileStorage::empty(size);
  let mut tilemap = commands.entity(tilemap_entity);

  for x in 0..size.x {
    for y in 0..size.y {
      let tile_pos = TilePos { x, y };
      tilemap.with_children(|tilemap| {
        let tile = tilemap
          .spawn(TileBundle {
            position: tile_pos,
            tilemap_id: TilemapId(tilemap_entity),
            ..Default::default()
          })
          .id();
        storage.set(&tile_pos, tile);
      });
    }
  }

  let grid_size = tile_size.into();
  let texture = TilemapTexture::Single(tiles.default.clone());

  tilemap.insert(TilemapBundle {
    grid_size,
    map_type,
    size,
    storage,
    texture,
    tile_size,
    anchor: TilemapAnchor::Center,
    ..Default::default()
  });
}
