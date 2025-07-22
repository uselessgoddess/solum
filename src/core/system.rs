use bevy::{
  log::{DEFAULT_FILTER, LogPlugin},
  prelude::*,
  window::WindowResolution,
};

pub fn plugin(app: &mut App) {
  app
    .add_plugins(
      DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
          primary_window: Window {
            resolution: WindowResolution::new(1920.0, 1080.0),
            // todo!> make configurable to use core as lib
            title: "Solum".to_string(),
            visible: true,
            ..default()
          }
          .into(),
          ..default()
        })
        .set(LogPlugin {
          filter: format!("{DEFAULT_FILTER},bevy_hanabi=error"),
          ..default()
        }),
    )
    .insert_resource(ClearColor(Color::srgb(0.4, 0.4, 0.4)));
}
