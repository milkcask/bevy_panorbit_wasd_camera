use bevy::prelude::*;

use crate::{ActiveCameraData, PanOrbitCamera};

#[derive(Resource, Default, Debug)]
pub struct KeyboardTracker {
    pub pan: Vec2,
    pub yaw: f32,
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

    if let Some(key) = pan_orbit.forward_key {
        if key_input.pressed(key) {
            pan.y -= 1.;
        }
    }
    if let Some(key) = pan_orbit.backward_key {
        if key_input.pressed(key) {
            pan.y += 1.;
        }
    }
    if let Some(key) = pan_orbit.left_key {
        if key_input.pressed(key) {
            pan.x += 1.;
        }
    }
    if let Some(key) = pan_orbit.right_key {
        if key_input.pressed(key) {
            pan.x -= 1.;
        }
    }

    // normalize to avoid faster diagonal movement
    if pan.length_squared() > 1. {
        pan = pan.normalize();
    }
    camera_movement.pan = pan;

    let mut yaw = 0.0;
    
    if let Some(key) = pan_orbit.counter_clockwise_key {
        if key_input.pressed(key) {
            yaw -= 1.0_f32.to_radians();
        }
    }
    
    if let Some(key) = pan_orbit.clockwise_key {
        if key_input.pressed(key) {
            yaw += 1.0_f32.to_radians();
        }
    }
    camera_movement.yaw = yaw;
}
