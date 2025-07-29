pub use crate::prelude::*;

pub fn marker<T: Component>(app: &mut App) {
  app.add_systems(PostUpdate, clean::<T>);
}

fn clean<T: Component>(marker: Query<Entity, With<T>>, mut commands: Commands) {
  for entity in marker.iter() {
    commands.entity(entity).remove::<T>();
  }
}
