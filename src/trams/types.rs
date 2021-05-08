use bevy::prelude::*;

pub(super) struct Materials {
    pub(super) stop_material: Handle<ColorMaterial>,
    pub(super) tram_material: Handle<ColorMaterial>,
}

pub(crate) struct TramLine {
    pub(super) stops: Vec<Entity>,
}

pub(crate) struct TramStop;

#[derive(Bundle)]
pub(super) struct StopBundle {
    pub(super) stop: TramStop,

    #[bundle]
    sprite: SpriteBundle,
}

impl StopBundle {
    pub(super) fn new(x: f32, y: f32, materials: &Res<Materials>) -> Self {
        StopBundle {
            stop: TramStop {},
            sprite: SpriteBundle {
                material: materials.stop_material.clone(),
                sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                transform: Transform {
                    translation: Vec3::new(x, y, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

pub(crate) struct Tram {
    pub(super) line: Entity,
}

#[derive(Bundle)]
pub(super) struct TramBundle {
    pub(super) tram: Tram,

    #[bundle]
    sprite: SpriteBundle,
}

impl TramBundle {
    pub(super) fn new(line: Entity, x: f32, y: f32, materials: &Res<Materials>) -> Self {
        TramBundle {
            tram: Tram { line },
            sprite: SpriteBundle {
                material: materials.tram_material.clone(),
                sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                transform: Transform {
                    translation: Vec3::new(x, y, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}
