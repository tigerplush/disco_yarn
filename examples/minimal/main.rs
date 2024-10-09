use bevy::prelude::*;
use bevy_yarnspinner::prelude::{YarnProject, YarnSpinnerPlugin};
use disco_yarn::DiscoElysiumYarnSpinnerDialogueViewPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            YarnSpinnerPlugin::new(),
            DiscoElysiumYarnSpinnerDialogueViewPlugin::default(),
        ))
        .add_systems(Startup, setup_camera)
        .add_systems(
            Update,
            spawn_dialogue_runner.run_if(resource_added::<YarnProject>),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    let mut dialogue_runner = project.create_dialogue_runner();
    dialogue_runner.start_node("HelloWorld");
    commands.spawn(dialogue_runner);
}
