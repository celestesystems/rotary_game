use bevy::prelude::*;
use gameplay::{camera::CameraGameplayPlugin, enemy::EnemyPlugin, player::PlayerPlugin, GameplayPlugin};

pub mod gameplay;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((CameraGameplayPlugin, PlayerPlugin, EnemyPlugin, GameplayPlugin))
        .run();
}
