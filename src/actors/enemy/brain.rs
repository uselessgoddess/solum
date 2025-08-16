use {super::Stats, crate::prelude::*};

pub fn plugin(app: &mut App) {
  app.add_systems(PreUpdate, patrol.in_set(BigBrainSet::Actions));
}

pub fn thinker() -> ThinkerBuilder {
  Thinker::build().picker(FirstToScore { threshold: 0.8 }).otherwise(Patrol)
}

#[derive(Component, ActionBuilder, Debug, Clone)]
pub struct Patrol;

use {
  crate::level::{Agent, Target},
  big_brain::prelude::ActionState,
};

fn patrol(
  query: Query<(&Transform2D, &Stats, Option<&Target>)>,
  mut actions: Query<(&Actor, &mut ActionState), With<Patrol>>,
  mut commands: Commands,
  spatial: SpatialQuery,
) {
  for (&Actor(actor), mut state) in actions.iter_mut() {
    if let Ok((transform, stats @ Stats { patrol, .. }, target)) =
      query.get(actor)
    {
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

          let dir = if let Some((dir, _)) =
            hits.iter().find(|(_, hit)| hit.is_none())
          {
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
          *state = ActionState::Success;
        }
        _ => {
          commands.entity(actor).remove::<Target>().remove::<Agent>();
        }
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
