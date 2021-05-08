use bevy::prelude::{Plugin as BevyPlugin, *};

mod setup;
mod types;

pub(super) struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup::setup_materials.system())
            .add_startup_stage(
                crate::constants::SETUP_STAGE,
                SystemStage::single_threaded().with_system(setup::setup_lines.system()),
            );
    }
}
