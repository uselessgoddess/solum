use crate::prelude::*;

pub fn plugin(app: &mut App) {
  app.configure_loading_state(
    LoadingStateConfig::new(Game::Loading)
      .load_collection::<InteractionAssets>()
      .load_collection::<CreditsAssets>()
      .load_collection::<TilesAssets>()
      .load_collection::<LevelAssets>()
      .load_collection::<StepsAssets>(),
  );
}

#[derive(AssetCollection, Resource, Reflect)]
pub struct InteractionAssets {
  #[asset(path = "audio/sounds/button_click.ogg")]
  pub click: Handle<AudioSource>,
  #[asset(path = "audio/sounds/button_hover.ogg")]
  pub hover: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource, Reflect)]
pub struct CreditsAssets {
  #[asset(path = "audio/music/Monkeys Spinning Monkeys.ogg")]
  pub music: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource, Reflect)]
pub struct TilesAssets {
  #[asset(path = "images/tiles/default.png")]
  pub default: Handle<Image>,
}

#[derive(AssetCollection, Resource, Reflect)]
pub struct LevelAssets {
  #[asset(path = "audio/music/Fluffing A Duck.ogg")]
  pub music: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource, Reflect)]
pub struct StepsAssets {
  #[asset(path = "audio/sounds/steps/tiles", collection(typed))]
  pub tiles: Vec<Handle<AudioSource>>,
}
