use crate::prelude::*;

pub fn plugin(app: &mut App) {}

#[derive(Component)]
pub struct MoveTo {
  current: Vec2,
  next: Vec<Vec2>,
  target: Entity,
}
