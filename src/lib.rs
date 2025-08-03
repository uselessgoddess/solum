#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]

use crate::prelude::*;

mod core;
mod game;
mod menus;
mod screens;
mod theme;

pub mod prelude;

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(core::plugin);

    app.add_plugins((
      game::plugin,
      menus::plugin,
      screens::plugin,
      theme::plugin,
    ));

    app.configure_sets(
      Update,
      (AppSystems::TickTimers, AppSystems::RecordInput, AppSystems::Update)
        .chain(),
    );

    app.add_systems(Startup, spawn_camera);
  }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(
  SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord,
)]
pub enum AppSystems {
  /// Tick timers.
  TickTimers,
  /// Record player input.
  RecordInput,
  /// Do everything
  Update,
}

fn spawn_camera(mut commands: Commands) {
  commands.spawn((Name::new("Camera"), PrimaryCamera));
}
