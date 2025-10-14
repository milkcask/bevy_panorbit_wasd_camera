use bevy::prelude::*;

use crate::{ActiveCameraData, PanOrbitCamera};

#[derive(Resource, Default, Debug)]
pub struct KeyboardTracker {
    pub pan: Vec2,
}

pub fn keyboard_tracker(
    mut camera_movement: ResMut<KeyboardTracker>,
    key_input: Res<ButtonInput<KeyCode>>,
    active_cam: Res<ActiveCameraData>,
    orbit_cameras: Query<&PanOrbitCamera>,
) {
    let active_entity = match active_cam.entity {
        Some(entity) => entity,
        None => return,
    };

    let pan_orbit = match orbit_cameras.get(active_entity) {
        Ok(camera) => camera,
        Err(_) => return,
    };

    let mut pan = Vec2::ZERO;

    if pan_orbit.forward_key.is_some() && key_input.pressed(pan_orbit.forward_key.unwrap()) {
        pan.y -= 1.;
    }
    if pan_orbit.backward_key.is_some() && key_input.pressed(pan_orbit.backward_key.unwrap()) {
        pan.y += 1.;
    }
    if pan_orbit.left_key.is_some() && key_input.pressed(pan_orbit.left_key.unwrap()) {
        pan.x += 1.;
    }
    if pan_orbit.right_key.is_some() && key_input.pressed(pan_orbit.right_key.unwrap()) {
        pan.x -= 1.;
    }

    // normalize to avoid faster diagonal movement
    if pan.length_squared() > 1. {
        pan = pan.normalize();
    }

    camera_movement.pan = pan;
}
