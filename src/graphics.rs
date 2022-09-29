use bevy::prelude::*;

use crate::{resources::{Settings, TimeNow, Wrapper}, SCALE_Y};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, setup)
            .add_system(update);
    }
}

#[derive(Component)]
struct TimeText;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    boundaries: Res<Wrapper>,
) {
    let font = asset_server.load("FiraSans-Bold.ttf");
    commands.spawn_bundle(Text2dBundle {
        transform: Transform {
            translation: Vec3::new(0., (boundaries.0.dimensions()[1] + 1_f32) / 2_f32 * SCALE_Y, 0_f32),
            ..Default::default()
        },
        text: Text::from_section(
            "Time:",
            TextStyle {
                color: Color::BLACK,
                font,
                ..Default::default()
            },
        ),
        ..Default::default()
    }).insert(TimeText);
}

fn update(
    settings: Res<Settings>,
    time: Res<TimeNow>,
    mut query: Query<&mut Text, With<TimeText>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = format!("Time: {:.4} ns", time.0 * settings.tau);
    }
}