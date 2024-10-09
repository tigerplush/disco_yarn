use bevy::prelude::*;
use bevy_yarnspinner::prelude::YarnSpinnerPlugin;

mod option_selection;
mod setup;
mod updating;

#[derive(Default)]
pub struct DiscoElysiumYarnSpinnerDialogueViewPlugin;

impl Plugin for DiscoElysiumYarnSpinnerDialogueViewPlugin {
    fn build(&self, app: &mut App) {
        assert!(
            app.is_plugin_added::<YarnSpinnerPlugin>(),
            "YarnSpinnerPlugin must be added before DiscoElysiumYarnSpinnerDialogueViewPlugin"
        );

        app.add_plugins(option_selection::plugin)
            .add_plugins(setup::plugin)
            .add_plugins(updating::plugin);
    }
}
