mod editor;
mod physics;
mod picking;
mod ui;

use bevy::{dev_tools::states::log_transitions, prelude::*};

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
  app.add_systems(Update, log_transitions::<Game>);

  app.add_plugins((
    #[cfg(feature = "dev_native")]
    editor::plugin,
    physics::plugin,
    picking::plugin,
    ui::plugin,
  ));

  app.insert_resource(NavMeshesDebug(RED_800.into()));
}
