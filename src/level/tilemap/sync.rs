use {crate::prelude::*, bevy::ecs::query::QueryData};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyncSet;

pub fn plugin(app: &mut App) {
  app.register_type::<WeakPos>().add_systems(
    PostUpdate,
    (transform, tilepos)
      .chain()
      .before(Transform2DSystem::TransformPropagate)
      .in_set(SyncSet),
  );
}

#[derive(QueryData)]
#[query_data(derive(Debug))]
pub struct Storage {
  entity: Entity,
  storage: &'static TileStorage,
  transform: &'static Transform2D,
  pub size: &'static TilemapSize,
  pub map_type: &'static TilemapType,
  pub grid_size: &'static TilemapGridSize,
  pub tile_size: &'static TilemapTileSize,
  pub anchor: &'static TilemapAnchor,
}

impl<'w> StorageItem<'w> {
  pub fn center_in_world(&self, pos: impl Into<TilePos>) -> Vec2 {
    self.transform.translation
      + pos.into().center_in_world(
        self.size,
        self.grid_size,
        self.tile_size,
        self.map_type,
        self.anchor,
      )
  }

  pub fn from_world_pos(&self, in_world: Transform2D) -> Option<TilePos> {
    let world = in_world.translation - self.transform.translation;
    TilePos::from_world_pos(
      &world,
      self.size,
      self.grid_size,
      self.tile_size,
      self.map_type,
      self.anchor,
    )
  }

  pub fn offset(
    &self,
    pos: TilePos,
    x: i32,
    y: i32,
  ) -> Option<(Entity, TilePos)> {
    let pos =
      TilePos::from_i32_pair(pos.x as i32 + x, pos.y as i32 + y, &self.size)?;
    self.storage.get(&pos).map(|entity| (entity, pos))
  }
}

fn transform(
  storage: Single<Storage>,
  enemies: Query<(Entity, &TilePos, Option<&Transform2D>)>,
  mut commands: Commands,
) {
  for (entity, &pos, transform) in enemies.iter() {
    let transform = transform.copied().unwrap_or_default();
    commands.entity(entity).insert(Transform2D {
      translation: storage.center_in_world(pos),
      ..transform
    });
  }
}

#[derive(Component, Reflect, Default, Clone, Copy, Debug)]
#[component(immutable)]
#[reflect(Component)]
pub struct WeakPos {
  pub x: u32,
  pub y: u32,
}

impl From<TilePos> for WeakPos {
  fn from(TilePos { x, y }: TilePos) -> Self {
    Self { x, y }
  }
}

impl From<WeakPos> for TilePos {
  fn from(WeakPos { x, y }: WeakPos) -> Self {
    Self { x, y }
  }
}

fn tilepos(
  storage: Single<Storage>,
  enemies: Query<(Entity, &Transform2D, Option<&TilePos>)>,
  mut commands: Commands,
) {
  for (entity, &transform, tilepos) in enemies.iter() {
    if let Some(pos) = tilepos.copied().map(WeakPos::from) {
      commands.entity(entity).insert(pos);
    } else if let Some(pos) =
      storage.from_world_pos(transform).map(WeakPos::from)
    {
      commands.entity(entity).insert(pos);
    }
  }
}
