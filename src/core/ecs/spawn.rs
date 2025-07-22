use crate::prelude::*;

pub trait Build: Send + Sync + 'static {
  type Input: Send;

  fn apply(&self, input: Self::Input, world: &mut World, entity: Entity);
}

impl<B: Build + ?Sized> Build for Box<B> {
  type Input = B::Input;

  fn apply(&self, input: Self::Input, world: &mut World, entity: Entity) {
    (**self).apply(input, world, entity);
  }
}

pub trait CommandsExt {
  fn spawn_dynamic_in<B: Build>(
    &mut self,
    input: B::Input,
    build: B,
  ) -> EntityCommands;

  fn spawn_dynamic<B: Build<Input = ()>>(
    &mut self,
    build: B,
  ) -> EntityCommands {
    self.spawn_dynamic_in((), build)
  }
}

impl CommandsExt for Commands<'_, '_> {
  fn spawn_dynamic_in<B: Build>(
    &mut self,
    input: B::Input,
    build: B,
  ) -> EntityCommands {
    let entity = self.spawn_empty().id();
    self.queue(move |world: &mut World| {
      build.apply(input, world, entity);
    });
    self.entity(entity)
  }
}
