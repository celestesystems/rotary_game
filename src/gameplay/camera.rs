use bevy::prelude::*;

#[derive(Component)]
pub struct CameraGameplay;

pub struct CameraGameplayPlugin;

impl Plugin for CameraGameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_spawn);
    }
}

fn camera_spawn(
    mut commands: Commands,
){
    commands.spawn((Camera2d::default(), CameraGameplay));
}