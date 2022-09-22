use bevy::prelude::*;
mod atom;
mod graphics;
mod resources;

const N_MOL: &str = "N_MOL";
const DENSITY: &str = "DENSITY";
const DELTA_T: &str = "DELTA_T";
const SCALE_X: f32 = 50.;
const SCALE_Y: f32 = 50.;
const SCALE_Z: f32 = 0.;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .add_plugin(atom::AtomPlugin)
        .add_plugin(graphics::GraphicsPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    resources::init(&mut commands, &asset_server);
}
