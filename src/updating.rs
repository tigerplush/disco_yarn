use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};
use bevy_yarnspinner::{
    events::{DialogueStartEvent, PresentLineEvent, PresentOptionsEvent},
    prelude::{DialogueRunner, YarnSpinnerSystemSet},
};

use crate::option_selection::OptionSelection;

use super::setup::{UiDialogueList, UiRootNode};

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            show_dialogue.run_if(on_event::<DialogueStartEvent>()),
            present_line.run_if(on_event::<PresentLineEvent>()),
            present_options.run_if(on_event::<PresentOptionsEvent>()),
            continue_dialogue,
            mouse_scroll,
            scroll_to_newest,
        )
            .chain()
            .after(YarnSpinnerSystemSet),
    );
}

fn show_dialogue(mut visibility: Query<&mut Visibility, With<UiRootNode>>) {
    *visibility.single_mut() = Visibility::Inherited;
}

fn present_line(
    mut line_events: EventReader<PresentLineEvent>,
    query: Query<Entity, With<UiDialogueList>>,
    mut commands: Commands,
) {
    for event in line_events.read() {
        write_line(
            &mut commands,
            query.single(),
            event.line.character_name(),
            event.line.text_without_character_name().as_str(),
        );
    }
}

pub fn write_line(commands: &mut Commands, entity: Entity, speaker: Option<&str>, line: &str) {
    let mut text = String::new();
    if let Some(name) = speaker {
        text = format!("{} - ", name.to_uppercase());
    }
    text.push_str(line);
    commands.entity(entity).with_children(|ui_dialogue_list| {
        ui_dialogue_list.spawn(
            TextBundle::from_section(text, TextStyle { ..default() }).with_style(style::standard()),
        );
    });
}

fn present_options(mut commands: Commands, mut events: EventReader<PresentOptionsEvent>) {
    for event in events.read() {
        let option_selection = OptionSelection::from_option_set(&event.options);
        commands.insert_resource(option_selection);
    }
}

fn continue_dialogue(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut dialogue_runners: Query<&mut DialogueRunner>,
    option_selection: Option<Res<OptionSelection>>,
) {
    let explicit_continue = keys.just_pressed(KeyCode::Space)
        || keys.just_pressed(KeyCode::Enter)
        || mouse_buttons.just_pressed(MouseButton::Left)
        || touches.any_just_pressed();

    if explicit_continue && option_selection.is_none() {
        for mut dialogue_runner in dialogue_runners.iter_mut() {
            if !dialogue_runner.is_waiting_for_option_selection() && dialogue_runner.is_running() {
                dialogue_runner.continue_in_next_update();
            }
        }
    }
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut UiDialogueList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (mut dialogue_list, mut style, parent, list_node) in query_list.iter_mut() {
            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);
            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            dialogue_list.position += dy;
            dialogue_list.position = dialogue_list.position.clamp(-max_scroll, 0.);
            style.top = Val::Px(dialogue_list.position);
        }
    }
}

fn scroll_to_newest(
    mut query_list: Query<(&mut UiDialogueList, &mut Style, &Parent, &Node), Changed<Node>>,
    query_node: Query<&Node>,
) {
    for (mut dialogue_list, mut style, parent, list_node) in query_list.iter_mut() {
        let items_height = list_node.size().y;
        let container_height = query_node.get(parent.get()).unwrap().size().y;

        let max_scroll = (items_height - container_height).max(0.);

        dialogue_list.position = -max_scroll;
        style.top = Val::Px(dialogue_list.position);
    }
}

mod style {
    use bevy::prelude::*;

    pub fn standard() -> Style {
        Style {
            padding: UiRect {
                top: Val::Percent(1.0),
                bottom: Val::Percent(1.0),
                ..default()
            },
            margin: UiRect {
                left: Val::Percent(5.0),
                right: Val::Percent(5.0),
                ..default()
            },
            ..default()
        }
    }
}
