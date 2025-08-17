#![allow(unused_imports)]

pub use core::{fmt::Debug, hash::Hash, marker::PhantomData, time::Duration};

pub use bevy::{
  audio::Volume,
  color::palettes::tailwind::*,
  diagnostic::FrameCount,
  ecs::{
    entity_disabling::Disabled,
    spawn::{SpawnIter, SpawnWith},
  },
  input::common_conditions::*,
  math::{vec2, vec3},
  platform::collections::{HashMap, HashSet},
  // gently forbid `Transform` to avoid random usages
  prelude::{Transform as _, *},
  sprite::Anchor,
  ui::{FocusPolicy, Val::*},
  window::PrimaryWindow,
};

pub use {
  avian2d::prelude::*,
  bevy_asset_loader::prelude::*,
  big_brain::prelude::*,
  ecs_tilemap::prelude::*,
  leafwing_input_manager::{
    InputControlKind,
    prelude::{ActionState, *},
  },
  navigator::prelude::*,
  ordered_float::OrderedFloat,
  rand::prelude::*,
  seldom::{
    StateMachinePlugin,
    prelude::{
      AnyState, EntityState, EntityTrigger, NotState, OneOfState, StateMachine,
      Trans, done, on_event,
    },
    trigger,
  },
  std::collections::VecDeque,
};

pub use crate::{
  assets::{
    CreditsAssets, InteractionAssets, LevelAssets, StepsAssets, TilesAssets,
  },
  core::*,
  ui::Game,
};

pub mod physics {
  use avian2d::prelude::*;

  pub const ALL: LayerMask = LayerMask::ALL;
  pub const NONE: LayerMask = LayerMask::NONE;
  pub const DEFAULT: LayerMask = LayerMask::DEFAULT;
  //
  pub const ENV: LayerMask = LayerMask(1 << 1);
  pub const PROJ: LayerMask = LayerMask(1 << 2);
  pub const ENEMY: LayerMask = LayerMask(1 << 3);

  pub fn env() -> CollisionLayers {
    CollisionLayers::new(ENV, ALL)
  }

  pub fn enemy() -> CollisionLayers {
    CollisionLayers::new(ENEMY, ENV | PROJ)
  }

  pub fn projectile() -> CollisionLayers {
    CollisionLayers::new(PROJ, ENEMY)
  }

  pub const COLLIDER_OFFSET: f32 = 1.0;
}
