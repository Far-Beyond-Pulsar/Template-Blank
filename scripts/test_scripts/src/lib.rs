//! <project>_scripts — your gameplay code, hot-reloadable in Play-In-Editor.
//!
//! Register every actor type in [`register_scripts`] via
//! `game.register_actor::<Type>(...)`. That call stamps the entity with the
//! type identity PIE hot reload matches on (#653): press Play again while a
//! game runs and your actors re-bind to their existing entities instead of
//! spawning duplicates.

use pulsar_game::scene::{MeshAssetPath, StaticMeshComponent, Transform};
use pulsar_game::tick::TickLoop;
// Re-exports: script crates need no direct scenedb dependency — everything
// gameplay-facing rides on `pulsar_game`.
use pulsar_game::{Actor, Entity, World};

/// Rotates a cube forever. Spawned at the world origin on Play.
///
/// `begin_play` gives the entity something visible (absent-only: scene-provided
/// components always win), and `tick` mutates the LIVE `Transform` component —
/// the same row the renderer reads, so edits show next frame and survive reload.
pub struct Spinner {
    degrees_per_second: f32,
}

impl Actor for Spinner {
    fn begin_play(&mut self, entity: Entity, world: &mut World) {
        if world.get::<StaticMeshComponent>(entity).is_none() {
            world.insert(
                entity,
                StaticMeshComponent {
                    mesh_asset: MeshAssetPath::new("meshes/primitives/SM_Cube.fbx"),
                    ..Default::default()
                },
            );
        }
        if world.get::<Transform>(entity).is_none() {
            world.insert(entity, Transform::default());
        }
    }

    fn tick(&mut self, entity: Entity, world: &mut World) {
        // Fixed-step approximation; see the tutorial for a time-driven variant.
        if let Some(mut transform) = world.get_mut::<Transform>(entity) {
            transform.rotation[1] += self.degrees_per_second * 0.016;
        }
    }
}

/// Called by generated `engine_main.rs` on every build/play.
///
/// The explicit turbofish is REQUIRED by convention: it is the marker the
/// level editor scans to offer your types in the add-object flow.
pub fn register_scripts(game: &mut TickLoop) -> Result<(), String> {
    game.register_actor::<Spinner>(Spinner {
        degrees_per_second: 45.0,
    });
    Ok(())
}
