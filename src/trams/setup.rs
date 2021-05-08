use bevy::prelude::*;

pub(super) fn setup_materials(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(super::types::Materials {
        stop_material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
        tram_material: materials.add(Color::rgb(0.1, 0.7, 0.1).into()),
    });
}

pub(super) fn setup_lines(mut commands: Commands, materials: Res<super::types::Materials>) {
    let stop1 = commands
        .spawn_bundle(super::types::TramStopBundle::new(50.0, 10.0, &materials))
        .id();
    let stop2 = commands
        .spawn_bundle(super::types::TramStopBundle::new(20.0, -30.0, &materials))
        .id();
    let route = commands
        .spawn()
        .insert(super::types::TramRoute {
            stops: vec![stop1, stop2],
        })
        .id();
}
