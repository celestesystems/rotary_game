use std::f32::consts::PI;

use bevy::prelude::*;

use super::{camera::CameraGameplay, enemy::Enemy, GameInfo, ObjectColor};

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_spawn);
        app.add_systems(Update, (player_control, player_rotate, player_check_enemy, player_update_mesh));
    }
}

fn player_spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    let shapes = [
        meshes.add(CircularSector::new(92., PI / 2.)),
        meshes.add(CircularSector::new(92., PI / 2.)),
    ];

    commands.spawn((Player, Transform::default()))
        .with_children(|parent|{
            parent.spawn((
                Mesh2d(shapes[0].clone()),
                MeshMaterial2d(materials.add(Color::LinearRgba(LinearRgba::RED))),
                Transform::default(),
            ));
            parent.spawn((
                Mesh2d(shapes[1].clone()),
                MeshMaterial2d(materials.add(Color::LinearRgba(LinearRgba::BLUE))),
                Transform::from_rotation(Quat::from_rotation_z(PI)),
            ));
    });
}

fn player_update_mesh(
    mut commands: Commands,
    player_q: Query<Entity, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_info: ResMut<GameInfo>,
){
    if game_info.wave_ended{
        let player_entity = player_q.single();

        commands.entity(player_entity).despawn_descendants();

        for i in 0..game_info.player_color_side{
            let shape = meshes.add(CircularSector::new(92., PI / game_info.player_color_side as f32));

            let mesh_color = match i {
                0 => LinearRgba::RED,
                1 => LinearRgba::BLUE,
                2 => LinearRgba::GREEN,
                3 => LinearRgba::new(1., 0.3, 1., 1.),
                4 => LinearRgba::new(1., 1., 0.3, 1.),
                5 => LinearRgba::new(0.3, 1., 1., 1.),
                _ => LinearRgba::RED,
            };

            let child = commands.spawn((
                Mesh2d(shape),
                MeshMaterial2d(materials.add(Color::LinearRgba(mesh_color))),
                Transform::from_rotation(Quat::from_rotation_z(i as f32 * PI * 2. / game_info.player_color_side as f32)),
            )).id();

            commands.entity(player_entity).add_child(child);

            game_info.wave_ended = false;
        }
    }
}

fn player_control(
    mut player_q: Query<&mut Transform, With<Player>>,
    camera_q: Single<(&Camera, &GlobalTransform), With<CameraGameplay>>,
    windows: Query<&Window>,
){
    let (camera, camera_transform) = *camera_q;

    let Ok(window) = windows.get_single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    let mut player_transform = player_q.single_mut();

    player_transform.translation.x = point.x;
    player_transform.translation.y = point.y;
}

fn player_rotate(
    mut player_q: Query<&mut Transform, With<Player>>,
    input_key: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
){
    let mut rotate_speed= 0.;

    if input_key.pressed(MouseButton::Left){
        rotate_speed += 1.;
    }

    if input_key.pressed(MouseButton::Right) {
        rotate_speed -= 1.;
    }

    let mut player_transform = player_q.single_mut();

    player_transform.rotate_z(rotate_speed * time.delta_secs());
}

fn player_check_enemy(
    mut commands: Commands,
    player_q: Query<&Transform, With<Player>>,
    enemy_q: Query<(Entity, &Transform, &Enemy), Without<Player>>,
    mut game_info: ResMut<GameInfo>,
){
    let player_transform = player_q.single();

    for (enemy_entity, enemy_transform, enemy)  in enemy_q.iter(){
        if player_transform.translation.distance(enemy_transform.translation) < 92. + 16.{
            let angle_range = 2. * PI / game_info.player_color_side as f32;

            let angle_index = match enemy.color{
                ObjectColor::RED => 0,
                ObjectColor::BLUE => 1,
                ObjectColor::GREEN => 2,
                ObjectColor::PINK => 3,
                ObjectColor::CYAN => 4,
                ObjectColor::YELLOW => 5,
            };

            let player_angle = player_transform.rotation.to_euler(EulerRot::ZXY).0;

            let check_angle_min = (player_angle + angle_range * angle_index as f32 + PI) % (PI * 2.);
            let check_angle_max = (player_angle + angle_range * (angle_index + 1) as f32 + PI) % (PI * 2.);

            let check_translation = enemy_transform.translation - player_transform.translation;

            let check_angle = (check_translation.y.atan2(check_translation.x) + PI) % (PI * 2.);

            if check_angle_max > check_angle_min{
                if check_angle > check_angle_min && check_angle < check_angle_max{
                    info!("enemy killed");
                }
                else {
                    info!("player hit");

                    game_info.game_over = true;

                    commands.spawn((
                        Text2d::new("Game Over :("),
                        TextFont::default(),
                    ));
                }
            }
            else {
                if check_angle > check_angle_min || check_angle < check_angle_max{
                    info!("enemy killed");
                }
                else {
                    info!("player hit");

                    commands.spawn((
                        Text2d::new("Game Over :("),
                        TextFont::default(),
                    ));
                }
            }

            commands.entity(enemy_entity).despawn_recursive();

            game_info.enemy_wave_current -= 1;
        }
    }
}