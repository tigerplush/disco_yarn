use bevy::{color::palettes::css, prelude::*, utils::HashMap, window::PrimaryWindow};
use bevy_yarnspinner::{
    events::DialogueCompleteEvent,
    prelude::{DialogueOption, DialogueRunner, OptionId, YarnSpinnerSystemSet},
};

use super::{setup::UiDialogueList, updating::write_line};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            create_options.run_if(resource_added::<OptionSelection>),
            select_option.run_if(
                resource_exists::<OptionSelection>.and_then(any_with_component::<PrimaryWindow>),
            ),
            despawn_options,
        )
            .chain()
            .after(YarnSpinnerSystemSet),
    )
    .add_event::<HasSelectedOptionEvent>();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Event)]
struct HasSelectedOptionEvent;

#[derive(Component)]
struct UiOptions;

#[derive(Component)]
struct OptionButton(OptionId, String);

#[derive(Resource)]
pub(crate) struct OptionSelection {
    options: Vec<DialogueOption>,
}

#[derive(Component)]
struct OptionLabel;

impl OptionSelection {
    pub fn from_option_set<'a>(options: impl IntoIterator<Item = &'a DialogueOption>) -> Self {
        let options = options
            .into_iter()
            .filter(|o| o.is_available)
            .cloned()
            .collect();
        Self { options }
    }
}

fn create_options(
    option_selection: Res<OptionSelection>,
    query: Query<Entity, With<UiDialogueList>>,
    mut commands: Commands,
) {
    commands
        .entity(query.single())
        .with_children(|ui_dialogue_list| {
            ui_dialogue_list
                .spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_self: AlignSelf::Stretch,
                            ..default()
                        },
                        ..default()
                    },
                    UiOptions,
                ))
                .with_children(|ui_options| {
                    for (i, option) in option_selection.options.iter().enumerate() {
                        ui_options
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        align_self: AlignSelf::Stretch,
                                        ..default()
                                    },
                                    ..default()
                                },
                                OptionButton(option.id, option.line.text_without_character_name()),
                            ))
                            .with_children(|button| {
                                let sections = [
                                    TextSection {
                                        value: format!("{}: ", i + 1),
                                        ..default()
                                    },
                                    TextSection {
                                        value: option.line.text.clone(),
                                        ..default()
                                    },
                                ];

                                button.spawn((TextBundle::from_sections(sections), OptionLabel));
                            });
                    }
                });
        });
}

fn select_option(
    keys: Res<ButtonInput<KeyCode>>,
    mut buttons: Query<
        (&Interaction, &OptionButton, &Children),
        (With<Button>, Changed<Interaction>),
    >,
    mut dialogue_runners: Query<&mut DialogueRunner>,
    mut text: Query<&mut Text, With<OptionLabel>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut selected_option_event: EventWriter<HasSelectedOptionEvent>,
    mut commands: Commands,
    option_selection: Res<OptionSelection>,
    query: Query<Entity, With<UiDialogueList>>,
) {
    let mut selection = None;

    let key_to_option: HashMap<_, _> = NUMBER_KEYS
        .into_iter()
        .zip(NUMPAD_KEYS)
        .zip(option_selection.options.iter().map(|option| option.id))
        .collect();

    for ((num_key, numpad_key), option) in key_to_option {
        if keys.just_pressed(num_key) || keys.just_pressed(numpad_key) {
            selection = Some(option);
            break;
        }
    }

    let mut window = windows.single_mut();
    for (interaction, button, _children) in buttons.iter_mut() {
        let (color, icon) = match *interaction {
            Interaction::Pressed if selection.is_none() => {
                selection = Some(button.0);
                write_line(
                    &mut commands,
                    query.single(),
                    Some("YOU"),
                    button.1.as_str(),
                );
                (css::TOMATO.into(), CursorIcon::Default)
            }
            Interaction::Hovered => (Color::WHITE, CursorIcon::Pointer),
            _ => (css::TOMATO.into(), CursorIcon::Default),
        };
        window.cursor.icon = icon;
        let text_entity = _children.iter().find(|&e| text.contains(*e)).unwrap();
        let mut text = text.get_mut(*text_entity).unwrap();
        text.sections[1].style.color = color;
    }

    let has_selected_id = selection.is_some();
    if let Some(id) = selection {
        for mut dialogue_runner in dialogue_runners.iter_mut() {
            dialogue_runner.select_option(id).unwrap();
        }
    }

    if has_selected_id {
        selected_option_event.send(HasSelectedOptionEvent);
    }
}

fn despawn_options(
    mut has_selected_option_event: EventReader<HasSelectedOptionEvent>,
    mut dialogue_complete_event: EventReader<DialogueCompleteEvent>,
    mut commands: Commands,
    query: Query<Entity, With<UiOptions>>,
) {
    let should_despawn =
        !has_selected_option_event.is_empty() || !dialogue_complete_event.is_empty();
    if !should_despawn {
        return;
    }
    has_selected_option_event.clear();
    dialogue_complete_event.clear();

    commands.remove_resource::<OptionSelection>();
    if !query.is_empty() {
        commands.entity(query.single()).despawn_recursive();
    }
}

const NUMBER_KEYS: [KeyCode; 9] = [
    KeyCode::Digit1,
    KeyCode::Digit2,
    KeyCode::Digit3,
    KeyCode::Digit4,
    KeyCode::Digit5,
    KeyCode::Digit6,
    KeyCode::Digit7,
    KeyCode::Digit8,
    KeyCode::Digit9,
];

const NUMPAD_KEYS: [KeyCode; 9] = [
    KeyCode::Numpad1,
    KeyCode::Numpad2,
    KeyCode::Numpad3,
    KeyCode::Numpad4,
    KeyCode::Numpad5,
    KeyCode::Numpad6,
    KeyCode::Numpad7,
    KeyCode::Numpad8,
    KeyCode::Numpad9,
];
