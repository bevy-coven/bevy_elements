//! A jump of the clock is a cut: the picture of the new hour is never brighter than that hour's
//! settled picture (issue #56). Before the fix the camera's exposure eased in from the old hour
//! and the sky probe kept the old hour's cloud light in its history, so a night jumped to from
//! noon stayed over-exposed for seconds.
mod support;

use std::sync::{Arc, Mutex};
use std::time::Duration;

use bevy::app::AppExit;
use bevy::camera::RenderTarget;
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::prelude::*;
use bevy::render::gpu_readback::{Readback, ReadbackComplete};
use bevy::render::render_resource::{TextureFormat, TextureUsages};
use bevy::time::TimeUpdateStrategy;
use bevy_elements::*;

/// Edge of the render target, px.
const SIZE: u32 = 64;
/// Bytes per `Rgba16Float` texel.
const TEXEL_BYTES: usize = 8;
/// Real seconds per frame: the clock is paused, so only the adaptation and the probe move.
const FRAME_S: f64 = 1.0 / 60.0;
/// Frames rendered at the first hour before the jump: long enough for the probe to be lit
/// and the exposure to settle.
const BEFORE_FRAMES: usize = 300;
/// Real time watched after the jump, s (the issue's acceptance window).
const AFTER_S: f64 = 30.0;
/// Frames at the end of the watch averaged as the settled picture.
const SETTLED_FRAMES: usize = 60;
/// Readbacks that can still carry a frame rendered before the jump: the copy is queued with
/// the frame and arrives after it.
const IN_FLIGHT_READBACKS: usize = 3;
/// How much brighter than the settled picture any frame after the jump may be, as a fraction.
/// Cloud and probe noise from frame to frame stays well inside it; the over-exposure of
/// issue #56 was 150 %.
const OVERSHOOT_MARGIN: f32 = 0.2;

/// Mean display value (exposed scene luminance, before tone mapping) of every frame read back
/// after the jump, in arrival order.
fn watch_jump(from_hour: f64, to_hour: f64) -> Vec<f32> {
    let means: Arc<Mutex<Vec<f32>>> = Arc::default();
    let mut app = App::new();
    app.add_plugins((
        support::headless_plugins(),
        ElementsPlugins
            .build()
            .disable::<PrecipitationPlugin>()
            .disable::<LightningPlugin>(),
    ))
    .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(
        FRAME_S,
    )))
    .insert_resource(WorldClock {
        paused: true,
        ..WorldClock::new(CivilDate::new(2026, 6, 21)).at_hour(from_hour)
    });
    let sink = means.clone();
    app.add_systems(
        Startup,
        move |mut commands: Commands,
              mut images: ResMut<Assets<Image>>,
              mut meshes: ResMut<Assets<Mesh>>,
              mut materials: ResMut<Assets<StandardMaterial>>,
              mut director: ResMut<WeatherDirector>| {
            let mut image = Image::new_target_texture(SIZE, SIZE, TextureFormat::Rgba16Float, None);
            image.texture_descriptor.usage |= TextureUsages::COPY_SRC;
            let handle = images.add(image);
            commands.spawn((
                Camera3d::default(),
                RenderTarget::Image(handle.clone().into()),
                ElementsCamera::default(),
                Tonemapping::None,
                DebandDither::Disabled,
                Transform::from_xyz(0.0, 3.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
            ));
            commands.spawn((
                Mesh3d(meshes.add(Plane3d::default().mesh().size(40.0, 40.0))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(0.5, 0.5, 0.5),
                    perceptual_roughness: 1.0,
                    ..default()
                })),
            ));
            commands.spawn((
                Mesh3d(meshes.add(Sphere::new(1.0))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(0.8, 0.8, 0.8),
                    perceptual_roughness: 0.8,
                    ..default()
                })),
                Transform::from_xyz(0.0, 1.0, 0.0),
            ));
            let sink = sink.clone();
            commands
                .spawn(Readback::texture(handle))
                .observe(move |done: On<ReadbackComplete>| {
                    let stride = (SIZE as usize * TEXEL_BYTES).next_multiple_of(256);
                    let mut sum = 0.0;
                    for y in 0..SIZE as usize {
                        for x in 0..SIZE as usize {
                            let t = &done.data[y * stride + x * TEXEL_BYTES..][..TEXEL_BYTES];
                            let c = |k: usize| support::half(u16::from_le_bytes([t[k], t[k + 1]]));
                            sum += 0.2126 * c(0) + 0.7152 * c(2) + 0.0722 * c(4);
                        }
                    }
                    sink.lock().expect("lock").push(sum / (SIZE * SIZE) as f32);
                });
            // Broken cloud: the sky probe composes clouds into the environment map.
            director.force(WeatherPreset::PartlyCloudy, Duration::ZERO);
        },
    );
    support::finish(&mut app);
    for _ in 0..BEFORE_FRAMES {
        app.update();
    }
    let at_jump = means.lock().expect("lock").len();
    app.world_mut().resource_mut::<WorldClock>().hour = to_hour;
    for _ in 0..(AFTER_S / FRAME_S) as usize {
        app.update();
    }
    app.world_mut().write_message(AppExit::Success);
    app.update();
    means.lock().expect("lock")[at_jump + IN_FLIGHT_READBACKS..].to_vec()
}

/// The brightest frame after the jump over the settled picture, printed with the series.
fn overshoot(from_hour: f64, to_hour: f64) -> (f32, f32, f32) {
    let after = watch_jump(from_hour, to_hour);
    assert!(
        after.len() > 2 * SETTLED_FRAMES,
        "only {} frames read back",
        after.len()
    );
    let settled = after[after.len() - SETTLED_FRAMES..].iter().sum::<f32>() / SETTLED_FRAMES as f32;
    let (worst_at, worst) = after
        .iter()
        .copied()
        .enumerate()
        .fold((0, f32::MIN), |a, (i, m)| if m > a.1 { (i, m) } else { a });
    for (i, m) in after.iter().enumerate().step_by(30) {
        println!(
            "{from_hour}->{to_hour} frame {i:5} mean {m:.4e} ({:.3}x settled)",
            m / settled
        );
    }
    println!(
        "{from_hour}->{to_hour}: settled {settled:.4e}, brightest {worst:.4e} at frame {worst_at} \
         ({:.3}x)",
        worst / settled
    );
    assert!(settled > 0.0, "the settled picture is black");
    (settled, worst, worst_at as f32 * FRAME_S as f32)
}

#[test]
#[ignore = "needs a GPU"]
fn a_night_jumped_to_from_noon_is_never_brighter_than_the_settled_night() {
    let (settled, worst, at_s) = overshoot(12.0, 23.0);
    assert!(
        worst <= settled * (1.0 + OVERSHOOT_MARGIN),
        "{worst:.4e} at {at_s:.2} s after the jump is {:.2}x the settled night {settled:.4e}",
        worst / settled
    );
}

#[test]
#[ignore = "needs a GPU"]
fn a_noon_jumped_to_from_night_is_never_brighter_than_the_settled_noon() {
    let (settled, worst, at_s) = overshoot(23.0, 12.0);
    assert!(
        worst <= settled * (1.0 + OVERSHOOT_MARGIN),
        "{worst:.4e} at {at_s:.2} s after the jump is {:.2}x the settled noon {settled:.4e}",
        worst / settled
    );
}
