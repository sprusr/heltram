use bevy::prelude::*;

static GAME_SETUP: &str = "game_setup";

struct Tram;

struct Materials {
    tram_material: Handle<ColorMaterial>,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage(GAME_SETUP, SystemStage::single(spawn_tram.system()))
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        tram_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    });
}

fn spawn_tram(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.tram_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(Tram);
}
