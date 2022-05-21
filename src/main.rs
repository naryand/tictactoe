use bevy::{
    input::{keyboard::KeyboardInput, ElementState},
    prelude::*,
};

#[derive(Debug)]
enum Input {
    Up,
    Down,
    Left,
    Right,
    Space,
}

impl TryFrom<u32> for Input {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            // WASD and Arrow keys
            87 | 38 => Ok(Input::Up),
            83 | 40 => Ok(Input::Down),
            65 | 37 => Ok(Input::Left),
            68 | 39 => Ok(Input::Right),

            32 => Ok(Input::Space),

            _ => Err(()),
        }
    }
}

#[derive(Component)]
struct MoveCamera;
const SLOT_SIZE: f32 = 100.0;
const GRID_SIZE: u32 = 3; 

const MAX_CAM: f32 = SLOT_SIZE * (GRID_SIZE / 2) as f32;

fn move_camera(
    mut key_evr: EventReader<KeyboardInput>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    for ev in key_evr.iter() {
        for mut transform in camera.iter_mut() {
            let mut x = transform.translation.x;
            let mut y = transform.translation.y;
            match (ev.state, Input::try_from(ev.scan_code)) {
                (ElementState::Pressed, Ok(i)) => match i {
                    Input::Up => y = f32::min(MAX_CAM, y + SLOT_SIZE),
                    Input::Down => y = f32::max(-MAX_CAM, y - SLOT_SIZE),
                    Input::Left => x = f32::max(-MAX_CAM, x - SLOT_SIZE),
                    Input::Right => x = f32::min(MAX_CAM, x + SLOT_SIZE),
                    _ => {}
                },
                _ => {}
            }
            transform.translation.x = x;
            transform.translation.y = y;
        }
    }
}

const BAR_THICKNESS: f32 = 10.0;
const BAR_LENGTH: f32 = GRID_SIZE as f32 * SLOT_SIZE;
const BAR_POS: f32 = SLOT_SIZE / 2.0;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    const POSITIONS: [((f32, f32), (f32, f32)); 4] = [
        ((BAR_POS, 0.0), (BAR_THICKNESS, BAR_LENGTH)),
        ((-BAR_POS, 0.0), (BAR_THICKNESS, BAR_LENGTH)),
        ((0.0, BAR_POS), (BAR_LENGTH, BAR_THICKNESS)),
        ((0.0, -BAR_POS), (BAR_LENGTH, BAR_THICKNESS)),
    ];
    for ((tx, ty), (sx, sy)) in POSITIONS {
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec2::new(tx, ty).extend(0.0),
                    scale: Vec2::new(sx, sy).extend(0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::BLACK,
                    ..default()
                },
                ..default()
            })
            .insert(MoveCamera);
    }
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_camera)
        .run();
}
