use bevy::prelude::*;
use bevy::render::pass::ClearColor;

mod constants;
mod trams;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "heltram".to_string(),
            width: 500.0,
            height: 500.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_plugin(trams::Plugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
