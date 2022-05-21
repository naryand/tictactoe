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
                    Input::Up => y = f32::min(100.0, y + 100.0),
                    Input::Down => y = f32::max(-100.0, y - 100.0),
                    Input::Left => x = f32::max(-100.0, x - 100.0),
                    Input::Right => x = f32::min(100.0, x + 100.0),
                    _ => {}
                },
                _ => {}
            }
            transform.translation.x = x;
            transform.translation.y = y;
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            ..default()
        })
        .insert(MoveCamera);
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
