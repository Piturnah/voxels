use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use noise::{NoiseFn, Perlin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system(init)
        .add_system(update_voxel)
        .add_system(config_ui)
        .insert_resource(Offset::default())
        .insert_resource(Noise::default())
        .run();
}

#[derive(Resource, Default)]
struct Offset {
    x: f64,
    y: f64,
    z: f64,
}

fn config_ui(mut ctx: ResMut<EguiContext>, mut offset: ResMut<Offset>) {
    egui::Window::new("Offset").show(ctx.ctx_mut(), |ui| {
        ui.add(egui::Slider::new(&mut offset.x, -10.0..=10.0).text("X"));
        ui.add(egui::Slider::new(&mut offset.y, -10.0..=10.0).text("Y"));
        ui.add(egui::Slider::new(&mut offset.z, -10.0..=10.0).text("Z"));
    });
}

#[derive(Component)]
struct Vx(Vec3);

fn update_voxel(mut q: Query<(&mut Visibility, &Vx)>, offset: Res<Offset>, noise: Res<Noise>) {
    if offset.is_changed() {
        for (mut vis, pos) in &mut q {
            vis.is_visible = noise.0.get([
                (pos.0.x as f64 / 10.0) + offset.x,
                (pos.0.y as f64 / 10.0) + offset.y,
                (pos.0.z as f64 / 10.0) + offset.z,
            ]) >= 0.05;
        }
    }
}

#[derive(Resource, Default)]
struct Noise(Perlin);

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    let cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let mat = mats.add(Color::rgb(0.3, 0.5, 0.3).into());

    for x in -5..5 {
        for y in -5..5 {
            for z in -5..5 {
                let pos = Vec3::new(x as f32, y as f32, z as f32);
                commands.spawn((
                    PbrBundle {
                        mesh: cube.clone(),
                        material: mat.clone(),
                        transform: Transform::from_translation(pos),
                        ..default()
                    },
                    Vx(pos),
                ));
            }
        }
    }

    // Spawn light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(1.4, 3.8, 9.1),
        ..default()
    });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform {
            translation: Vec3::new(5.0, 5.0, 15.0),
            rotation: Quat::from_euler(EulerRot::XYZ, -0.4, 0.4, 0.0),
            ..default()
        },
        ..default()
    });
}
