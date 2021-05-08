use bevy::prelude::*;

pub(super) struct Materials {
    pub(super) stop_material: Handle<ColorMaterial>,
    pub(super) tram_material: Handle<ColorMaterial>,
}

/// Segment of track in a single direction
pub(crate) struct TramLine {
    /// One end of the TramLine segment, connecting entities with TramStop or
    /// TramLine components. Empty means end of line with no loop.
    pub(super) from: Vec<Entity>,
    /// Other end of the TramLine segment, connecting entities with TramStop or
    /// TramLine components. Empty means end of line with no loop.
    pub(super) to: Vec<Entity>,
}

/// Route of TramStops which Trams can run on
pub(crate) struct TramRoute {
    /// Entities with TramStop components which this TramRoute consists of
    pub(super) stops: Vec<Entity>,
}

/// When visited by a TramRoute, Trams can stop here
pub(crate) struct TramStop {
    /// Entities with TramLine component which come into this TramStop. Empty
    /// means end of line with no loop.
    pub(super) incoming: Vec<Entity>,
    /// Entities with TramLine component which go out of this TramStop. Empty
    /// means end of line with no loop.
    pub(super) outgoing: Vec<Entity>,
}

#[derive(Bundle)]
pub(super) struct TramStopBundle {
    pub(super) stop: TramStop,

    #[bundle]
    sprite: SpriteBundle,
}

impl TramStopBundle {
    pub(super) fn new(x: f32, y: f32, materials: &Res<Materials>) -> Self {
        TramStopBundle {
            stop: TramStop {
                incoming: vec![],
                outgoing: vec![],
            },
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
    /// Entity with TramRoute component which this Tram is currently following
    pub(super) route: Entity,
    /// Entity with TramLine or TramStop component which this Tram is on/at
    pub(super) location: Entity,
    /// Progress along TramLine segment if applicable
    pub(super) progress: Option<f64>,
}

#[derive(Bundle)]
pub(super) struct TramBundle {
    pub(super) tram: Tram,

    #[bundle]
    sprite: SpriteBundle,
}

impl TramBundle {
    pub(super) fn new(route: Entity, location: Entity, materials: &Res<Materials>) -> Self {
        TramBundle {
            tram: Tram {
                route,
                location,
                progress: None,
            },
            sprite: SpriteBundle {
                material: materials.tram_material.clone(),
                sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                ..Default::default()
            },
        }
    }
}
