use bevy::ecs::query::QueryFilter;

use crate::{
  actors::Player,
  level::{Difficulty, Obstacle},
};

use {super::Stats, crate::prelude::*};

pub fn plugin(app: &mut App) {
  app.add_systems(
    PreUpdate,
    (
      (player_near_scorer,).in_set(BigBrainSet::Scorers),
      (chase_action, attack_action, patrol_action).in_set(BigBrainSet::Actions),
    ),
  );
}

pub fn thinker() -> ThinkerBuilder {
  let chase_and_attack = Steps::build().step(Chase).step(Attack);

  Thinker::build()
    .picker(FirstToScore { threshold: 0.8 })
    .when(PlayerVisibleScorer { hysteresis: 1.2 }, chase_and_attack)
    .otherwise(Patrol)
}

fn player_near_scorer(
  query: Query<(&Transform2D, &Stats)>,
  players: Query<&Transform2D, With<Player>>,
  mut scores: Query<(&Actor, &PlayerVisibleScorer, &mut Score)>,
  mut state: Local<HashMap<Entity, bool>>,
) {
  for (&Actor(actor), scorer, mut score) in scores.iter_mut() {
    if let Ok((&Transform2D { translation, .. }, stats)) = query.get(actor)
      && let Some(player) = find_closest(&players, translation)
    {
      let is_active = state.get(&actor).copied().unwrap_or(false);

      let hysteresis = if is_active { scorer.hysteresis } else { 1.0 };

      if translation.distance(player) < stats.vision * hysteresis {
        score.set(1.0);
      } else {
        score.set(0.0);
      }

      state.insert(actor, is_active);
    }
  }
}

#[derive(Component, ActionBuilder, Debug, Clone)]
pub struct Chase;

#[derive(Component, ScorerBuilder, Debug, Clone)]
pub struct PlayerVisibleScorer {
  /// memories coefficient to avoid jittering
  pub hysteresis: f32,
}

fn find_closest<F: QueryFilter>(
  query: &Query<&Transform2D, F>,
  position: Vec2,
) -> Option<Vec2> {
  query
    .iter()
    .map(Transform2D::translation)
    .min_by_key(|entity| OrderedFloat(entity.distance_squared(position)))
}

type SensorsQuery<'w, 's, 'q> = (
  Query<'w, 's, &'q Transform2D, With<Player>>,
  Query<'w, 's, &'q Transform2D, (With<Obstacle>, Without<Difficulty>)>,
);

fn chase_action(
  query: Query<(&Transform2D, &Stats, &CollidingEntities)>,
  mut actions: Query<(&Actor, &mut ActionState), With<Chase>>,
  (players, obstacles): SensorsQuery,
  mut commands: Commands,
) {
  for (&Actor(actor), mut state) in actions.iter_mut() {
    let Ok((&Transform2D { translation, .. }, stats, colliding)) =
      query.get(actor)
    else {
      continue;
    };

    // todo!> mb use promise-like config with cancellation and Ok/Err?
    //  and also components requirements?
    match *state {
      ActionState::Requested => {
        *state = ActionState::Executing;
      }
      ActionState::Executing => {
        let Some(player) = find_closest(&players, translation) else {
          *state = ActionState::Cancelled;
          continue;
        };
        commands
          .entity(actor)
          .insert((Target::new(player), Agent::new(stats.speed)));

        if colliding.iter().find_map(|&e| obstacles.get(e).ok()).is_some()
          || translation.distance(player) < stats.attack.range
        {
          *state = ActionState::Success;
        }
      }
      ActionState::Cancelled => {
        *state = ActionState::Failure;
      }
      _ => {
        commands.entity(actor).remove::<Target>().remove::<Agent>();
      }
    }
  }
}

#[derive(Component, ActionBuilder, Debug, Clone)]
pub struct Attack;

fn attack_action(
  query: Query<(&Transform2D, &Stats, &CollidingEntities)>,
  mut actions: Query<(&Actor, &mut ActionState), With<Attack>>,
  (players, obstacles): SensorsQuery,
) {
  for (&Actor(actor), mut state) in actions.iter_mut() {
    let Ok((&Transform2D { translation, .. }, stats, colliding)) =
      query.get(actor)
    else {
      continue;
    };

    match *state {
      ActionState::Requested => {
        *state = ActionState::Executing;
      }
      ActionState::Executing => {
        if let Some(player) = find_closest(&players, translation)
          && translation.distance(player) < stats.attack.range
        {
          println!("attack: PLAYER");
        } else if let Some(entity) =
          colliding.iter().copied().filter(|&e| obstacles.get(e).is_ok()).next()
        {
          println!("attack: {entity:?}");
        } else {
          *state = ActionState::Cancelled;
        }
      }
      ActionState::Cancelled => {
        *state = ActionState::Failure;
      }
      _ => {}
    }
  }
}

#[derive(Component, ActionBuilder, Debug, Clone)]
pub struct Patrol;

use {
  crate::level::{Agent, Target},
  big_brain::prelude::ActionState,
};

fn patrol_action(
  query: Query<(&Transform2D, &Stats, Option<&Target>)>,
  mut actions: Query<(&Actor, &mut ActionState), With<Patrol>>,
  mut commands: Commands,
  spatial: SpatialQuery,
) {
  for (&Actor(actor), mut state) in actions.iter_mut() {
    let Ok((transform, stats @ Stats { patrol, .. }, target)) =
      query.get(actor)
    else {
      continue;
    };

    match *state {
      ActionState::Requested => {
        *state = ActionState::Executing;
      }
      ActionState::Executing => {
        let mut rng = rand::rng();

        if target.is_some() {
          continue;
        }

        let radius = rng.random_range(0.0..stats.vision);
        let mut hits =
          cast_places(&spatial, transform.translation, patrol.rays, radius);
        hits.shuffle(&mut rng);

        let dir =
          if let Some((dir, _)) = hits.iter().find(|(_, hit)| hit.is_none()) {
            dir.as_vec2() * radius
          } else if let Some((dir, hit)) = hits.choose(&mut rng) {
            // SAFETY: there is no `None` in hits
            let hit = unsafe { hit.unwrap_unchecked() };

            // todo!> use enemy collider stats here
            dir.as_vec2() * hit.distance
          } else {
            continue;
          };

        let dir = dir + transform.translation;

        commands
          .entity(actor)
          .insert((Target::new(dir), Agent::new(stats.speed)));
      }
      ActionState::Cancelled => {
        *state = ActionState::Failure;
      }
      _ => {
        commands.entity(actor).remove::<Target>().remove::<Agent>();
      }
    }
  }
}

fn cast_places(
  query: &SpatialQuery,
  origin: Vec2,
  amount: u8,
  radius: f32,
) -> Vec<(Dir2, Option<RayHitData>)> {
  (0..amount)
    .map(|offset| {
      let angle = 360.0 * (offset as f32 / amount as f32);
      // SAFETY: `Rot2` rotates unit `Vec2::Y` vector
      let direction = Dir2::new_unchecked(Rot2::degrees(angle) * Vec2::Y);

      let filter = SpatialQueryFilter::from_mask(physics::ENV);
      (direction, query.cast_ray(origin, direction, radius, true, &filter))
    })
    .collect()
}
