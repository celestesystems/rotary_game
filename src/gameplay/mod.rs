use bevy::prelude::*;
use enemy::EnemySpawn;

pub mod player;
pub mod enemy;
pub mod camera;
pub mod utility;

#[derive(Default)]
pub enum ObjectColor {
    #[default]
    RED,
    BLUE,
    GREEN,
    PINK,
    YELLOW,
    CYAN,
}

#[derive(Resource)]
pub struct GameInfo{
    pub player_color_side: u8,
    pub enemy_wave_time: f32,
    pub enemy_wave_amount: u8,
    pub enemy_wave_total: u8,
    pub enemy_wave_current: u32,
    pub enemy_wave_spawned: u8,
    pub enemy_speed: f32,
    pub wave_ended: bool,
    level: u8,
    end: bool,
    game_over: bool,
}

#[derive(Component)]
pub struct LevelInfo{
    pub index: u8,
    pub player_color_side: u8,
    pub enemy_wave_time: f32,
    pub enemy_wave_amount: u8,
    pub enemy_speed: f32,
}

#[derive(Component)]
pub struct LevelText;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (game_info_init, level_info_init));

        app.add_systems(Update, game_info_update);
    }
}

fn game_info_init(
    mut commands: Commands
){
    commands.insert_resource(GameInfo{
        player_color_side: 2,
        enemy_wave_time: 3.,
        enemy_wave_amount: 1,
        enemy_wave_total: 8,
        enemy_wave_current: 8,
        enemy_wave_spawned: 0,
        enemy_speed: 256.,
        wave_ended: false,
        level: 0,
        end: false,
        game_over: false,
    });
}


fn level_info_init(
    mut commands: Commands
){
    commands.spawn((
        Text2d::new("Level 0"),
        TextFont::default(),
        Transform::from_xyz(0., 256., 0.),
        LevelText,
    ));

    let levels: [LevelInfo; 16] = [
        LevelInfo{
            index: 0,
            player_color_side: 2,
            enemy_wave_time: 3.,
            enemy_wave_amount: 1,
            enemy_speed: 256.
        },
        LevelInfo{
            index: 1,
            player_color_side: 2,
            enemy_wave_time: 2.,
            enemy_wave_amount: 1,
            enemy_speed: 256.
        },
        LevelInfo{
            index: 2,
            player_color_side: 2,
            enemy_wave_time: 2.,
            enemy_wave_amount: 1,
            enemy_speed: 292.
        },
        LevelInfo{
            index: 3,
            player_color_side: 3,
            enemy_wave_time: 2.,
            enemy_wave_amount: 1,
            enemy_speed: 292.
        },
        LevelInfo{
            index: 4,
            player_color_side: 3,
            enemy_wave_time: 1.6,
            enemy_wave_amount: 1,
            enemy_speed: 292.
        },
        LevelInfo{
            index: 5,
            player_color_side: 3,
            enemy_wave_time: 2.,
            enemy_wave_amount: 2,
            enemy_speed: 292.
        },
        LevelInfo{
            index: 6,
            player_color_side: 4,
            enemy_wave_time: 2.2,
            enemy_wave_amount: 1,
            enemy_speed: 292.
        },
        LevelInfo{
            index: 7,
            player_color_side: 4,
            enemy_wave_time: 2.5,
            enemy_wave_amount: 2,
            enemy_speed: 256.
        },
        LevelInfo{
            index: 8,
            player_color_side: 4,
            enemy_wave_time: 2.,
            enemy_wave_amount: 2,
            enemy_speed: 288.,
        },
        LevelInfo{
            index: 9,
            player_color_side: 4,
            enemy_wave_time: 1.7,
            enemy_wave_amount: 2,
            enemy_speed: 300.
        },
        LevelInfo{
            index: 10,
            player_color_side: 5,
            enemy_wave_time: 2.,
            enemy_wave_amount: 1,
            enemy_speed: 280.
        },
        LevelInfo{
            index: 11,
            player_color_side: 5,
            enemy_wave_time: 1.8,
            enemy_wave_amount: 2,
            enemy_speed: 260.
        },
        LevelInfo{
            index: 12,
            player_color_side: 5,
            enemy_wave_time: 1.6,
            enemy_wave_amount: 1,
            enemy_speed: 300.
        },
        LevelInfo{
            index: 13,
            player_color_side: 6,
            enemy_wave_time: 2.4,
            enemy_wave_amount: 1,
            enemy_speed: 256.
        },
        LevelInfo{
            index: 14,
            player_color_side: 6,
            enemy_wave_time: 3.,
            enemy_wave_amount: 2,
            enemy_speed: 256.
        },
        LevelInfo{
            index: 15,
            player_color_side: 6,
            enemy_wave_time: 2.,
            enemy_wave_amount: 2,
            enemy_speed: 290.
        },
    ];

    for level in levels {
        commands.spawn(level);
    }
}

fn game_info_update(
    mut commands: Commands,
    mut game_info: ResMut<GameInfo>,
    mut enemy_spawner: ResMut<EnemySpawn>,
    mut level_text_q: Query<&mut Text2d, With<LevelText>>,
    level_q: Query<&LevelInfo>,
){
    if game_info.enemy_wave_current == 0{
        if game_info.end {
            commands.spawn((
                Text2d::new("Congrats :)"),
                TextFont::default(),
        ));
        }

        game_info.level += 1;

        for level in level_q.iter() {
            if level.index == game_info.level{
                game_info.player_color_side = level.player_color_side;
                game_info.enemy_speed = level.enemy_speed;
                game_info.enemy_wave_time = level.enemy_wave_time;
                game_info.enemy_wave_amount = level.enemy_wave_amount;
            }
        }

        if game_info.level == 15 {
            game_info.end = true;
        }

        enemy_spawner.timer.reset();
    
        game_info.enemy_wave_current = game_info.enemy_wave_total as u32 * game_info.enemy_wave_amount as u32;

        game_info.enemy_wave_spawned = 0;

        game_info.wave_ended = true;

        let level = game_info.level;

        info!("Switching to level {level}");

        level_text_q.single_mut().0 = format!("Level {}", game_info.level);
    }
}