//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::dev_tools::states::log_transitions;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app
        // print state transitions in dev builds
        .add_systems(Update, log_transitions::<Screen>)
        // add floating debug inspector window
        .add_plugins(WorldInspectorPlugin::new());
}
