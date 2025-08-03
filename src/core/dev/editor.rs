use editor_pls::{EditorPlugin, EditorWindowPlacement};

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
  app.register_type::<EditorWindow>().add_systems(
    Update,
    toggle_editor_window.run_if(input_just_pressed(TOGGLE_KEY)),
  );

  let window = app
    .world_mut()
    .spawn((
      Name::new("Editor Window"),
      EditorWindow,
      Window {
        title: "Editor".to_string(),
        focused: false,
        visible: false,
        ..default()
      },
    ))
    .id();

  app.add_plugins(EditorPlugin {
    window: EditorWindowPlacement::Window(window),
  });
}

const TOGGLE_KEY: KeyCode = KeyCode::F3;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct EditorWindow;

fn toggle_editor_window(
  mut window_query: Query<&mut Window, With<EditorWindow>>,
) {
  for mut window in &mut window_query {
    window.visible ^= true;
  }
}
