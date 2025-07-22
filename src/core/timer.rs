use bevy::time::{Timer, TimerMode};

pub fn new(secs: f32) -> Timer {
  Timer::from_seconds(secs, TimerMode::Once)
}

pub fn repeat(secs: f32) -> Timer {
  Timer::from_seconds(secs, TimerMode::Repeating)
}