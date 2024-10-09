use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}

#[derive(Component)]
pub struct UiRootNode;

#[derive(Component, Default)]
pub struct UiDialogueList {
    pub position: f32,
}

fn setup(mut commands: Commands) {
    commands
        .spawn((
            Name::from("root"),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::End,
                    ..default()
                },
                // visibility: Visibility::Hidden,
                ..default()
            },
            UiRootNode,
        ))
        .with_children(|root_node| {
            // this is the black bar where all dialogue is shown consecutively
            root_node
                .spawn((
                    Name::from("scroll_view"),
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            width: Val::Percent(33.0),
                            ..default()
                        },
                        background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                ))
                .with_children(|scroll_view| {
                    scroll_view
                        .spawn((
                            Name::from("list_w_hidden_overflow"),
                            NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Column,
                                    align_self: AlignSelf::Stretch,
                                    height: Val::Percent(100.),
                                    overflow: Overflow::clip_y(),
                                    ..default()
                                },
                                ..default()
                            },
                        ))
                        .with_children(|list_w_hidden_overflow| {
                            list_w_hidden_overflow.spawn((
                                Name::from("moving_panel"),
                                NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        ..default()
                                    },
                                    ..default()
                                },
                                UiDialogueList::default(),
                            ));
                        });
                });
        });
}
