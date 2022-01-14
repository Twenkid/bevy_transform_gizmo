use bevy::prelude::*;
use bevy_mod_raycast::DefaultRaycastingPlugin;

use crate::TransformGizmo;

pub type GizmoPickSource = bevy_mod_raycast::RayCastSource<GizmoRaycastSet>;
pub type PickableGizmo = bevy_mod_raycast::RayCastMesh<GizmoRaycastSet>;

/// Plugin with all the systems and resources used to raycast against gizmo handles separately from
/// the `bevy_mod_picking` plugin.
pub struct GizmoPickingPlugin;
impl Plugin for GizmoPickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DefaultRaycastingPlugin::<GizmoRaycastSet>::default())
            .add_system_to_stage(
                CoreStage::PreUpdate,
                update_gizmo_raycast_with_cursor.before(bevy_mod_raycast::RaycastSystem::BuildRays),
            )
            .add_system_to_stage(
                CoreStage::PreUpdate,
                disable_mesh_picking_during_gizmo_hover
                    .before(bevy_mod_picking::PickingSystem::Focus)
                    .after(bevy_mod_raycast::RaycastSystem::UpdateRaycast),
            );
    }
}

pub struct GizmoRaycastSet;

/// Update the gizmo's raycasting source with the current mouse position.
fn update_gizmo_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut GizmoPickSource>,
) {
    for mut pick_source in &mut query.iter_mut() {
        // Grab the most recent cursor event if it exists:
        if let Some(cursor_latest) = cursor.iter().last() {
            pick_source.cast_method =
                bevy_mod_raycast::RayCastMethod::Screenspace(cursor_latest.position);
        }
    }
}

/// Disable the picking plugin when the mouse is over one of the gizmo handles.
fn disable_mesh_picking_during_gizmo_hover(
    mut picking_state: ResMut<bevy_mod_picking::PickingPluginsState>,
    query: Query<&GizmoPickSource>,
    gizmo_query: Query<&TransformGizmo>,
) {
    let not_hovering_gizmo = if let Ok(source) = query.get_single() {
        source.intersect_top().is_none()
    } else {
        true
    };
    let gizmo_inactive = if let Ok(gizmo) = gizmo_query.get_single() {
        gizmo.current_interaction().is_none()
    } else {
        error!("Not exactly one gizmo.");
        return;
    };
    // Set the picking state based on current user interaction state
    picking_state.enable_interacting = gizmo_inactive && not_hovering_gizmo;
}
