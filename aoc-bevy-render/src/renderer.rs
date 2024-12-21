use std::sync::{mpsc, Arc, Mutex};

use aoc_core::{Answer, SolutionCollection};
use bevy::sprite::Mesh2dHandle;
#[cfg(not(target_arch = "wasm32"))]
use bevy::{prelude::*, sprite::Anchor, sprite::MaterialMesh2dBundle, winit::WinitSettings};
mod camera;
mod ui;

#[derive(Resource, Clone)]
struct SolutionDisplayHandler {
    collection: Arc<SolutionCollection>,
    rx_component_pipeline: Arc<Mutex<mpsc::Receiver<aoc_core::RenderCommand>>>,
    tx_component_pipeline: mpsc::Sender<aoc_core::RenderCommand>,
}
impl SolutionDisplayHandler {
    fn new(mut collection: SolutionCollection) -> Self {
        let (tx, rx) = mpsc::channel();
        collection.set_render_pipeline(tx.clone());
        Self {
            collection: Arc::new(collection),
            rx_component_pipeline: Arc::new(Mutex::new(rx)),
            tx_component_pipeline: tx,
        }
    }

    fn get_days(&self) -> Vec<(u32, bool)> {
        self.collection.get_days_with_render()
    }

    pub fn run_part_a(&self, day: u32) -> (Answer, std::time::Duration) {
        self.tx_component_pipeline
            .send(aoc_core::RenderCommand::Clear)
            .expect("Valid pipeline");
        self.collection.run_and_render_part1(&day)
    }
    pub fn run_part_b(&self, day: u32) -> (Answer, std::time::Duration) {
        self.tx_component_pipeline
            .send(aoc_core::RenderCommand::Clear)
            .expect("Valid pipeline");
        self.collection.run_and_render_part2(&day)
    }
}

pub fn run(collection: SolutionCollection) {
    let mut app = App::new();
    let handler: SolutionDisplayHandler = SolutionDisplayHandler::new(collection);
    app.add_plugins(DefaultPlugins)
        .add_plugins(ui::UiPlugin)
        .insert_resource(handler)
        .add_plugins(camera::PanCamPlugin)
        .add_systems(Startup, setup)
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Update, process_pipeline)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), camera::PanCam::default()));
}

#[derive(Component)]
pub struct RenderedCommand;

const SCALE: f32 = 25.0;

fn process_pipeline(
    state: Res<SolutionDisplayHandler>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    rendered_query: Query<Entity, With<RenderedCommand>>,
) {
    while let Ok(cmd) = state.rx_component_pipeline.lock().unwrap().try_recv() {
        match cmd {
            aoc_core::RenderCommand::Clear => {
                for entity in &rendered_query {
                    commands.entity(entity).despawn_recursive();
                }
            }
            aoc_core::RenderCommand::Text(text) => {
                // let font = asset_server.load("fonts/FiraSans-Bold.ttf");
                let text_style = TextStyle {
                    //font: font.clone(),
                    color: Color::srgb_from_array(text.color.to_array()),
                    font_size: SCALE,
                    ..default()
                };
                // How to position so it fits in a grid... Lets create each char seperate for now
                for (i, c) in text.content.chars().enumerate() {
                    let x = text.x + i as f32;
                    let y = text.y;
                    commands.spawn((
                        Text2dBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: c.to_string(),
                                    style: text_style.clone(),
                                }],
                                ..default()
                            },
                            transform: Transform::from_translation(Vec3::new(
                                x * SCALE + (SCALE / 2.0),
                                -y * SCALE, // + SCALE / 2.0,
                                0.0,
                            )),
                            text_anchor: Anchor::TopCenter,
                            ..default()
                        },
                        RenderedCommand,
                    ));
                }
            }
            aoc_core::RenderCommand::Shape(grid_shape) => {
                let color = grid_shape
                    .color
                    .map(|c| Color::srgb_from_array(c.to_array()));
                match grid_shape.shape {
                    aoc_core::GridShapeType::Rectangle(width, height)
                        if grid_shape.border_size.is_none() =>
                    {
                        if color.is_none() {
                            continue;
                        }
                        let shape = Rectangle::new(width * SCALE, height * SCALE);
                        let mesh = Mesh2dHandle(meshes.add(shape));
                        let transform = Transform::from_xyz(
                            (grid_shape.x + (width / 2.)) * SCALE,
                            -(grid_shape.y + (height / 2.)) * SCALE,
                            0.,
                        );
                        commands.spawn((
                            MaterialMesh2dBundle {
                                mesh,
                                material: materials.add(color.unwrap()),
                                transform,
                                ..default()
                            },
                            RenderedCommand,
                        ));
                    }
                    aoc_core::GridShapeType::Rectangle(width, height) => {
                        let height = height * SCALE;
                        let width = width * SCALE;
                        let border_size = grid_shape.border_size.unwrap_or_default() * SCALE;

                        let shape = Rectangle::new(
                            width - (border_size * 2.0),
                            height - (border_size * 2.0),
                        );
                        let vertical_border_shape = Rectangle::new(border_size, height);
                        let horizontal_border_shape = Rectangle::new(width, border_size);
                        let mesh = Mesh2dHandle(meshes.add(shape));
                        let vertical_border_mesh = Mesh2dHandle(meshes.add(vertical_border_shape));
                        let horizontal_border_mesh =
                            Mesh2dHandle(meshes.add(horizontal_border_shape));

                        let border_color = grid_shape
                            .border_color
                            .map(|bc| Color::srgb_from_array(bc.to_array()))
                            .or(color);
                        let translation = Vec3::new(
                            (grid_shape.x * SCALE) + (width / 2.),
                            -((grid_shape.y * SCALE) + (height / 2.)),
                            0.,
                        );
                        if let Some(color) = color {
                            commands.spawn((
                                MaterialMesh2dBundle {
                                    mesh,
                                    material: materials.add(color),
                                    transform: Transform::from_translation(translation),
                                    ..default()
                                },
                                RenderedCommand,
                            ));
                        }
                        if border_color.is_none() {
                            continue;
                        }
                        let border_color = border_color.unwrap();
                        if grid_shape.borders.0 {
                            commands.spawn((
                                MaterialMesh2dBundle {
                                    mesh: horizontal_border_mesh.clone(),
                                    material: materials.add(border_color),
                                    transform: Transform::from_translation(
                                        translation
                                            + Vec3::new(0., (height / 2.) - (border_size / 2.), 0.),
                                    ),
                                    ..default()
                                },
                                RenderedCommand,
                            ));
                        }
                        if grid_shape.borders.1 {
                            commands.spawn((
                                MaterialMesh2dBundle {
                                    mesh: vertical_border_mesh.clone(),
                                    material: materials.add(border_color),
                                    transform: Transform::from_translation(
                                        translation
                                            - Vec3::new((width / 2.) - (border_size / 2.), 0., 0.),
                                    ),
                                    ..default()
                                },
                                RenderedCommand,
                            ));
                        }
                        if grid_shape.borders.2 {
                            commands.spawn((
                                MaterialMesh2dBundle {
                                    mesh: horizontal_border_mesh.clone(),
                                    material: materials.add(border_color),
                                    transform: Transform::from_translation(
                                        translation
                                            - Vec3::new(0., (height / 2.) - (border_size / 2.), 0.),
                                    ),
                                    ..default()
                                },
                                RenderedCommand,
                            ));
                        }
                        if grid_shape.borders.3 {
                            commands.spawn((
                                MaterialMesh2dBundle {
                                    mesh: vertical_border_mesh.clone(),
                                    material: materials.add(border_color),
                                    transform: Transform::from_translation(
                                        translation
                                            + Vec3::new((width / 2.) - (border_size / 2.), 0., 0.),
                                    ),
                                    ..default()
                                },
                                RenderedCommand,
                            ));
                        }
                    }
                    aoc_core::GridShapeType::Circle(radius) => {
                        if color.is_none() {
                            continue;
                        }
                        let shape = Circle {
                            radius: radius * SCALE,
                        };
                        let mesh = Mesh2dHandle(meshes.add(shape));
                        let transform = Transform::from_xyz(
                            (grid_shape.x + radius) * SCALE,
                            -(grid_shape.y + radius) * SCALE,
                            0.,
                        );
                        commands.spawn((
                            MaterialMesh2dBundle {
                                mesh,
                                material: materials.add(color.unwrap()),
                                transform,
                                ..default()
                            },
                            RenderedCommand,
                        ));
                    }
                }
            }
            aoc_core::RenderCommand::Line(line) => {
                let point1 = Vec2::new(line.x0, line.y0) * SCALE;
                let point2 = Vec2::new(line.x1, line.y1) * SCALE;
                let diff = point2 - point1;
                let center = (point1 + point2) / 2.;
                let angle = Dir2::new_unchecked(diff / diff.length()).to_angle();
                let shape = Capsule2d::new(1.0, diff.length());
                let mesh = Mesh2dHandle(meshes.add(shape));
                let color = Color::srgb_from_array(line.color.to_array());
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh,
                        material: materials.add(color),
                        transform: Transform::from_xyz(center.x, -center.y, 0.0).with_rotation(
                            Quat::from_rotation_z(angle + std::f32::consts::FRAC_PI_2),
                        ),
                        ..default()
                    },
                    RenderedCommand,
                ));
            }
        }
    }
}
