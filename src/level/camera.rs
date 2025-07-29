use {crate::prelude::*, bevy::core_pipeline::bloom::Bloom};

pub fn plugin(app: &mut App) {
  app
    .add_plugins(InputManagerPlugin::<Action>::default())
    .add_systems(Update, (spawn, input));
}

fn spawn(query: Query<Entity, Added<PrimaryCamera>>, mut commands: Commands) {
  for entity in query.iter() {
    commands
      .entity(entity)
      .insert((
        UiSourceCamera::<0>,
        Transform2D::IDENTITY.with_layer(ui::DEPTH),
      ))
      .insert((
        Camera { hdr: true, ..default() },
        Bloom::OLD_SCHOOL,
        Projection::Orthographic(OrthographicProjection {
          scale: 0.01,
          ..OrthographicProjection::default_2d()
        }),
      ))
      .insert(InputMap::default().with_axis(Action::Zoom, MouseScrollAxis::Y))
      .insert(inspector_egui::bevy_egui::PrimaryEguiContext);
  }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum Action {
  #[actionlike(Axis)]
  Zoom,
}

fn input(
  mut query: Query<
    (&ActionState<Action>, &mut Projection),
    With<PrimaryCamera>,
  >,
) {
  for (state, proj) in query.iter_mut() {
    let zoom = state.value(&Action::Zoom);

    if let Projection::Orthographic(proj) = proj.into_inner() {
      proj.scale = (proj.scale * (1. - zoom * 0.05)).clamp(0.01, 0.1);
    }
  }
}
