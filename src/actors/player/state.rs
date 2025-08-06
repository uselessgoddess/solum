pub use {
  super::*,
  crate::prelude::*,
  trigger::{axis_pair_max_length, axis_pair_min_length},
};

pub fn plugin(app: &mut App) {
  app.register_type::<Stats>();
  app.register_type::<Player>();
  app.add_plugins(InputManagerPlugin::<Action>::default());
  app.add_systems(FixedUpdate, (idle, walk));
}

pub fn bundle() -> impl Bundle {
  (Idle, input(), machine())
}

// todo!> make configurable or use more accurate way
const DEAD_ZONE: f32 = 0.1;

pub fn machine() -> StateMachine {
  StateMachine::default()
    .trans::<Walk, _>(axis_pair_max_length(Action::Move, DEAD_ZONE), Idle)
    .trans_builder(
      axis_pair_min_length(Action::Move, DEAD_ZONE),
      |trans: Trans<AnyState, _>| Walk::forward(trans.out),
    )
}

pub fn input() -> InputMap<Action> {
  InputMap::default()
    .with_dual_axis(Action::Move, VirtualDPad::wasd())
    .with_dual_axis(Action::Move, GamepadStick::LEFT)
}

#[derive(Actionlike, Clone, Eq, Hash, PartialEq, Reflect, Debug)]
pub enum Action {
  #[actionlike(DualAxis)]
  Move,
}

#[derive(Component, Clone)]
#[component(storage = "SparseSet")]
pub struct Idle;

fn idle(mut player: Query<(&Player, &mut LinearVelocity), With<Idle>>) {
  for (_player, mut velocity) in player.iter_mut() {
    velocity.0 = Vec2::ZERO;
  }
}


#[derive(Component, Reflect, Clone)]
#[component(storage = "SparseSet")]
pub struct Walk {
  pub forward: Rot2,
}

impl Walk {
  pub fn forward(Vec2 { x, y }: Vec2) -> Self {
    Self { forward: Rot2::radians(f32::atan2(-x, y)) }
  }
}

fn walk(mut player: Query<(&Player, &Stats, &mut Walk, &mut LinearVelocity)>) {
  for (_player, stats, walk, mut velocity) in player.iter_mut() {
    velocity.0 = walk.forward * Vec2::Y * stats.speed;
  }
}
