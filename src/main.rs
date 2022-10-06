use std::f32::consts::FRAC_PI_8;

use bevy::{
    diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::WindowMode,
};
use bevy_editor_pls::{
    controls::{self, EditorControls},
    prelude::EditorPlugin,
};

#[derive(Component)]
struct GltfContainer {
    rotate: bool,
}

#[bevy_main]
fn main() {
    info!("Toggle rotation with R");
    let scale = 2.0;
    let mut app = App::new();
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Gltf Viewer".to_string(),
            width: 640.0 * scale,
            height: 480.0 * scale,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // Editor
        .add_plugin(EditorPlugin)
        .insert_resource(editor_controls())
        // Diagnostics
        .add_plugin(EntityCountDiagnosticsPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        // Actual Scene
        .add_startup_system(setup_world)
        .add_startup_system(setup_pbr)
        .add_system(rotate_pbr)
        .add_system(toggle_rotate);
    app.run();
}

fn setup_pbr(mut commands: Commands, asset_server: Res<AssetServer>) {
    let pos = Vec3::new(0.0, 0.0, 0.0);
    let scene_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "models/FlightHelmet/FlightHelmet.gltf#Scene0".to_string());

    info!("Loading scene from {:#?}", scene_path);

    commands
        .spawn_bundle(SpatialBundle::default())
        .with_children(|parent| {
            parent
                .spawn_bundle(SceneBundle {
                    scene: asset_server.load(&scene_path),
                    transform: {
                        Transform {
                            translation: pos,
                            ..Default::default()
                        }
                    },
                    ..default()
                })
                .insert(Name::new("Gltf"));
        })
        .insert(GltfContainer { rotate: true });
}

fn rotate_pbr(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &GltfContainer), With<GltfContainer>>,
) {
    if let Ok((mut transform, GltfContainer { rotate })) = query.get_single_mut() {
        if *rotate {
            let dt = time.delta_seconds();
            transform.rotate_y(dt * FRAC_PI_8);
        }
    }
}

fn toggle_rotate(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut GltfContainer>) {
    if keyboard_input.just_pressed(KeyCode::R) {
        if let Ok(mut container) = query.get_single_mut() {
            container.rotate = !container.rotate;
        }
    }
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 4.0, -8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 8.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.2, 0.2, 0.2),
            perceptual_roughness: 0.8,
            ..default()
        }),
        ..default()
    });

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(50.0, 50.0, 50.0),
        point_light: PointLight {
            intensity: 600000.,
            range: 100.,
            ..default()
        },
        ..default()
    });
}

// -----------------
// Editor Controls
// -----------------
fn editor_controls() -> EditorControls {
    let mut editor_controls = EditorControls::default_bindings();
    editor_controls.unbind(controls::Action::PlayPauseEditor);

    editor_controls.insert(
        controls::Action::PlayPauseEditor,
        controls::Binding {
            input: controls::UserInput::Single(controls::Button::Keyboard(KeyCode::Escape)),
            conditions: vec![controls::BindingCondition::ListeningForText(false)],
        },
    );

    editor_controls
}
