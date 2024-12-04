use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use rand::random;

use super::{player::Player, GameInfo, ObjectColor};

#[derive(Component, Default)]
pub struct Enemy{
    pub color: ObjectColor,
}

#[derive(Resource)]
pub struct EnemySpawn{
    pub timer: Timer,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, enemy_spawn_timer_init);

        app.add_systems(Update, (enemy_spawn_timer, enemy_follow));
    }
}

fn enemy_spawn_timer_init(
    mut commands: Commands,
){
    commands.insert_resource(EnemySpawn{
        timer: Timer::new(Duration::from_secs(6), TimerMode::Once),
    });
}

fn enemy_spawn_timer(
    mut commands: Commands,
    player_q: Query<&Transform, With<Player>>,
    mut enemy_spawner: ResMut<EnemySpawn>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_info: ResMut<GameInfo>,
){
    enemy_spawner.timer.tick(time.delta());

    if enemy_spawner.timer.just_finished() && game_info.enemy_wave_spawned < game_info.enemy_wave_total && !game_info.game_over && !game_info.end{
        for _ in 0..game_info.enemy_wave_amount {
            enemy_spawn(&mut commands, &player_q, &mut meshes, &mut materials, game_info.player_color_side);
        }

        enemy_spawner.timer.set_duration(Duration::from_secs(game_info.enemy_wave_time as u64));
        enemy_spawner.timer.reset();

        game_info.enemy_wave_spawned += 1;
    }
}

fn enemy_spawn(
    commands: &mut Commands,
    player_q: &Query<&Transform, With<Player>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    colors: u8,
){
    let shape = meshes.add(Triangle2d::new(Vec2::new(0., -16.), Vec2::new(0., 16.), Vec2::new(32., 0.)));

    let player_transform = player_q.single();

    let player_pos = player_transform.translation;

    let enemy_angle: f32 = random::<f32>() * PI * 2.;

    let enemy_spawn_pos = player_pos + Vec3::new((enemy_angle).cos() * 512., (enemy_angle).sin() * 512., 0.);

    let color_index = (random::<f32>() * colors as f32) as u8;

    let (color, mesh_color) = match color_index {
        0 => (ObjectColor::RED, LinearRgba::RED),
        1 => (ObjectColor::BLUE, LinearRgba::BLUE),
        2 => (ObjectColor::GREEN, LinearRgba::GREEN),
        3 => (ObjectColor::PINK, LinearRgba::new(1., 0.3, 1., 1.)),
        4 => (ObjectColor::YELLOW,LinearRgba::new(1., 1., 0.3, 1.)),
        5 => (ObjectColor::CYAN, LinearRgba::new(0.3, 1., 1., 1.)),
        _ => (ObjectColor::RED, LinearRgba::RED),
    };

    commands.spawn((Enemy{
        color: color,
        },
        Mesh2d(shape), 
        MeshMaterial2d(materials.add(Color::LinearRgba(mesh_color))), 
        Transform::from_translation(enemy_spawn_pos)
        ));
}

fn enemy_follow(
    mut enemy_q: Query<&mut Transform, With<Enemy>>,
    player_q: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>
){
    let player_transform = player_q.single();

    for mut enemy_transform in enemy_q.iter_mut() {        
        let angle = (player_transform.translation.y - enemy_transform.translation.y).atan2(player_transform.translation.x - enemy_transform.translation.x);

        enemy_transform.rotation = enemy_transform.rotation.slerp(Quat::from_rotation_z(angle), 0.1);

        let right = enemy_transform.right();

        enemy_transform.translation +=  right * 256. * time.delta_secs();
    }
}