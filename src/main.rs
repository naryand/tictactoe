use bevy::{
    input::{keyboard::KeyboardInput, ElementState},
    prelude::*,
};

use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

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

#[derive(Component, Debug, Default)]
struct Turn {
    o_turn: bool,
}

const SLOT_SIZE: f32 = 100.0;
const GRID_SIZE: u32 = 3;

const MAX_CAM: f32 = SLOT_SIZE * (GRID_SIZE / 2) as f32;

fn move_camera(
    mut key_evr: EventReader<KeyboardInput>,
    mut set: ParamSet<(
        Query<&mut Transform, With<Camera>>,
        Query<&mut Transform, With<Turn>>,
    )>,
) {
    let mut x = 0.0;
    let mut y = 0.0;
    for mut cam_transform in set.p0().iter_mut() {
        x = cam_transform.translation.x;
        y = cam_transform.translation.y;
        for ev in key_evr.iter() {
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
            cam_transform.translation.x = x;
            cam_transform.translation.y = y;
        }
    }

    for mut cur_transform in set.p1().iter_mut() {
        cur_transform.translation.x = x;
        cur_transform.translation.y = y;
    }
}

const SYMBOL_RATIO: f32 = 1.0 / 3.0;
const SYMBOL_RAD: f32 = SLOT_SIZE * SYMBOL_RATIO;

fn o_shape(x: f32, y: f32) -> shapes::Circle {
    shapes::Circle {
        radius: SYMBOL_RAD,
        center: Vec2::new(x, y),
    }
}

fn build_o(x: f32, y: f32, a: f32) -> ShapeBundle {
    GeometryBuilder::new().add(&o_shape(x, y)).build(
        DrawMode::Stroke(StrokeMode::new(Color::rgba(0.0, 0.0, 0.0, a), BAR_THICKNESS)),
        Transform::default(),
    )
}


fn x_shape1(x: f32, y: f32) -> shapes::Line {
    shapes::Line(
        Vec2::new(x - SYMBOL_RAD, y - SYMBOL_RAD),
        Vec2::new(x + SYMBOL_RAD, y + SYMBOL_RAD),
    )
}
fn x_shape2(x: f32, y: f32) -> shapes::Line {
    shapes::Line(
        Vec2::new(x + SYMBOL_RAD, y - SYMBOL_RAD),
        Vec2::new(x - SYMBOL_RAD, y + SYMBOL_RAD),
    )
}

fn build_x(x: f32, y: f32, a: f32) -> ShapeBundle {
    GeometryBuilder::new()
        .add(&x_shape1(x, y))
        .add(&x_shape2(x, y))
        .build(
            DrawMode::Stroke(StrokeMode::new(Color::rgba(0.0, 0.0, 0.0, a), BAR_THICKNESS)),
            Transform::default(),
        )
}

fn mark_space(
    mut commands: Commands,
    mut key_evr: EventReader<KeyboardInput>,
    mut cursor: Query<(&mut Turn, &mut Path)>,
    camera: Query<&Transform, With<Camera>>,
) {
    for ev in key_evr.iter() {
        for transform in camera.iter() {
            for (mut turn, mut path) in cursor.iter_mut() {
                let x = transform.translation.x;
                let y = transform.translation.y;
                match (ev.state, Input::try_from(ev.scan_code)) {
                    (ElementState::Pressed, Ok(Input::Space)) => {
                        commands.spawn_bundle(if turn.o_turn {
                            build_o(x, y, 1.0)
                        } else {
                            build_x(x, y, 1.0)
                        });

                        turn.o_turn = !turn.o_turn;
                        match turn.o_turn {
                            true => {
                                *path = ShapePath::new().add(&o_shape(0.0, 0.0)).build();
                            }
                            false => {
                                *path = ShapePath::new()
                                    .add(&x_shape1(0.0, 0.0))
                                    .add(&x_shape2(0.0, 0.0))
                                    .build();
                            }
                        }
                    }
                    _ => {}
                }
            }
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
        commands.spawn_bundle(SpriteBundle {
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
        });
    }

    commands
        .spawn_bundle(build_x(0.0, 0.0, 0.2))
        .insert(Turn::default());
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup)
        .add_system(mark_space)
        .add_system(move_camera)
        .run();
}
