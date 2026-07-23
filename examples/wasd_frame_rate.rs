//! Demonstrates that keyboard movement is frame rate independent.
//!
//! The app deliberately jumps between frame rates every couple of seconds, but the
//! camera keeps moving at the same speed throughout.
//!
//! Controls:
//!     Pan: W/A/S/D
//!     Rotate: Q/E
//!     Pitch: R/F

use std::thread;
use std::time::{Duration, Instant};

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_panorbit_wasd_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

/// Frame rates the app jumps between
const FRAME_RATES: [f32; 3] = [144.0, 60.0, 30.0];

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // Disable vsync so the frame limiter below is in full control
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PanOrbitCameraPlugin)
        .insert_resource(FrameRateCycle {
            index: 0,
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (cycle_frame_rate, update_status_text))
        .add_systems(Last, limit_frame_rate)
        .run();
}

#[derive(Resource)]
struct FrameRateCycle {
    index: usize,
    timer: Timer,
}

#[derive(Component)]
struct StatusText;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));
    // Cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // Camera
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
        PanOrbitCamera::default(),
    ));
    // Status text
    commands.spawn((Text::default(), StatusText));
}

// Jumps to the next frame rate cap every time the timer finishes
fn cycle_frame_rate(time: Res<Time<Real>>, mut cycle: ResMut<FrameRateCycle>) {
    if cycle.timer.tick(time.delta()).just_finished() {
        cycle.index = (cycle.index + 1) % FRAME_RATES.len();
    }
}

fn update_status_text(
    time: Res<Time<Real>>,
    cycle: Res<FrameRateCycle>,
    mut text_query: Query<&mut Text, With<StatusText>>,
) {
    let measured = 1.0 / time.delta_secs().max(f32::EPSILON);
    for mut text in &mut text_query {
        text.0 = format!(
            "\
Frame rate cap: {:.0}fps (measured: {:.0}fps)
Hold W/A/S/D, Q/E, or R/F - movement speed stays the same at every frame rate",
            FRAME_RATES[cycle.index], measured,
        );
    }
}

// Caps the frame rate by sleeping until the target frame duration has elapsed
fn limit_frame_rate(cycle: Res<FrameRateCycle>, mut frame_start: Local<Option<Instant>>) {
    let target = Duration::from_secs_f32(1.0 / FRAME_RATES[cycle.index]);
    if let Some(start) = *frame_start {
        if let Some(remaining) = target.checked_sub(start.elapsed()) {
            thread::sleep(remaining);
        }
    }
    *frame_start = Some(Instant::now());
}
