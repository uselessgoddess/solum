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

pub use {avian2d::prelude::*, rand::prelude::*};

pub use crate::{AppSystems, core::*};
