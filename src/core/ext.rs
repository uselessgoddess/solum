use crate::prelude::*;

pub trait TweenableExt {
  fn is_total_completed(&self) -> bool;
}

impl<T> TweenableExt for &dyn Tweenable<T> {
  fn is_total_completed(&self) -> bool {
    if let TotalDuration::Finite(duration) = self.total_duration()
      && self.elapsed() >= duration
    {
      true
    } else {
      false
    }
  }
}
