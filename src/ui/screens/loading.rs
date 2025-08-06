//! A loading screen during which game assets are loaded if necessary.
//! This reduces stuttering, especially for audio on Wasm.

use bevy::prelude::*;

use crate::{
  prelude::*,
  ui::{Game, widget},
};

pub(super) fn plugin(app: &mut App) {
  app.add_systems(OnEnter(Game::Loading), spawn_loading_screen);
}

fn spawn_loading_screen(mut commands: Commands) {
  commands.spawn((
    widget::ui_root("Loading Screen"),
    StateScoped(Game::Loading),
    children![widget::label("Loading...")],
  ));
}
