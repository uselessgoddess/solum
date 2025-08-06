//! The main menu (seen on the title screen).

use bevy::prelude::*;

use crate::{
  prelude::*,
  ui::{Game, Menu, widget},
};

pub(super) fn plugin(app: &mut App) {
  app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
}

fn spawn_main_menu(mut commands: Commands) {
  commands.spawn((
    widget::ui_root("Main Menu"),
    GlobalZIndex(2),
    StateScoped(Menu::Main),
    #[cfg(not(target_family = "wasm"))]
    children![
      widget::button("Play", enter_gameplay),
      widget::button("Settings", open_settings_menu),
      widget::button("Credits", open_credits_menu),
      widget::button("Exit", exit_app),
    ],
    #[cfg(target_family = "wasm")]
    children![
      widget::button("Play", enter_loading_or_gameplay_screen),
      widget::button("Settings", open_settings_menu),
      widget::button("Credits", open_credits_menu),
    ],
  ));
}

fn enter_gameplay(
  _: Trigger<Pointer<Click>>,
  mut next_screen: ResMut<NextState<Game>>,
) {
  next_screen.set(Game::Gameplay);
}

fn open_settings_menu(
  _: Trigger<Pointer<Click>>,
  mut next_menu: ResMut<NextState<Menu>>,
) {
  next_menu.set(Menu::Settings);
}

fn open_credits_menu(
  _: Trigger<Pointer<Click>>,
  mut next_menu: ResMut<NextState<Menu>>,
) {
  next_menu.set(Menu::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
  app_exit.write(AppExit::Success);
}
