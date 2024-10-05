use bevy::{
    asset::{AssetServer, Handle},
    color::{palettes::css, Color},
    input::ButtonInput,
    prelude::{
        BuildChildren, ButtonBundle, Changed, Children, Commands, Component, DespawnRecursiveExt,
        Entity, ImageBundle, KeyCode, NodeBundle, Query, Res, ResMut, StateScoped, TextBundle,
        Visibility,
    },
    text::{Font, Text, TextStyle},
    ui::{
        AlignItems, BackgroundColor, BorderColor, BorderRadius, Display, FlexDirection,
        FocusPolicy, Interaction, JustifyContent, Overflow, PositionType, RepeatedGridTrack, Style,
        UiImage, UiRect, Val, ZIndex,
    },
};

use crate::{input::keyboard::GameAction, KeyMap};

use super::{MenuButtonAction, MenuState, ScrollingList, NORMAL_BUTTON};

#[derive(Debug, Component, PartialEq, Eq)]
pub struct ClearButton(GameAction, Entity);

#[derive(Component, Debug, PartialEq, Eq)]
pub struct EditControlButton(GameAction);

#[derive(Component)]
pub struct ActionRecorder {
    pub action: GameAction,
    pub entity: Entity,
}

pub fn controls_menu_setup(mut commands: Commands, assets: Res<AssetServer>, key_map: Res<KeyMap>) {
    let font: Handle<Font> = assets.load("fonts/gohu.ttf");
    let trash_icon = assets.load("./trash.png");

    commands
        .spawn((
            StateScoped(MenuState::SettingsControls),
            NodeBundle {
                style: Style {
                    padding: UiRect::horizontal(Val::Vw(15.)),
                    top: Val::Px(0.),
                    display: Display::Flex,
                    width: Val::Vw(100.),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|root| {
            let placeholder = root
                .spawn((
                    ButtonBundle {
                        z_index: ZIndex::Global(3),
                        style: Style {
                            position_type: PositionType::Absolute,
                            top: Val::Px(10.),
                            left: Val::Px(10.),
                            padding: UiRect::all(Val::Px(5.)),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    MenuButtonAction::BackToSettings,
                ))
                .with_children(|btn| {
                    btn.spawn(TextBundle {
                        text: Text::from_section(
                            "Back",
                            TextStyle {
                                font: font.clone(),
                                font_size: 21.,
                                color: Color::WHITE,
                            },
                        ),
                        ..Default::default()
                    });
                })
                .id();

            root.spawn(NodeBundle {
                style: Style {
                    overflow: Overflow::clip_y(),
                    height: Val::Vh(100.),
                    width: Val::Vw(60.),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|wrapper| {
                wrapper
                    .spawn((
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                width: Val::Percent(100.),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        ScrollingList { position: 0. },
                    ))
                    .with_children(|list| {
                        list.spawn(TextBundle {
                            text: Text::from_section(
                                "Keyboard Controls",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 36.,
                                    color: Color::WHITE,
                                },
                            ),
                            style: Style {
                                margin: UiRect::vertical(Val::Px(20.)),
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                        for (action, keys) in &key_map.map {
                            list.spawn((
                                ButtonBundle {
                                    border_color: BorderColor(Color::srgb(0.3, 0.3, 0.3)),
                                    style: Style {
                                        display: Display::Grid,
                                        width: Val::Percent(100.),
                                        height: Val::Auto,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        grid_template_columns: vec![
                                            RepeatedGridTrack::flex(2, 1.),
                                            RepeatedGridTrack::px(1, 40.),
                                        ],
                                        border: UiRect::bottom(Val::Px(2.5)),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                EditControlButton(*action),
                            ))
                            .with_children(|line| {
                                line.spawn(TextBundle {
                                    text: Text::from_section(
                                        format!("{:?}", action),
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 24.,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    style: Style {
                                        margin: UiRect::all(Val::Px(10.)),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                });

                                let mut component = line.spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::RowReverse,
                                        column_gap: Val::Px(15.),
                                        margin: UiRect::horizontal(Val::Px(10.)),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                });

                                let id = component.id();

                                update_input_component(
                                    &mut component.commands(),
                                    id,
                                    keys,
                                    &assets,
                                );

                                line.spawn((
                                    ButtonBundle {
                                        border_radius: BorderRadius::all(Val::Percent(25.)),
                                        focus_policy: FocusPolicy::Pass,
                                        style: Style {
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            width: Val::Percent(80.),
                                            padding: UiRect::all(Val::Px(5.)),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                    ClearButton(*action, id),
                                ))
                                .with_children(|btn| {
                                    btn.spawn(ImageBundle {
                                        image: UiImage::new(trash_icon.clone()),
                                        style: Style {
                                            width: Val::Percent(100.),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    });
                                });
                            });
                        }
                    });
            });

            root.spawn((
                NodeBundle {
                    visibility: Visibility::Hidden,
                    focus_policy: FocusPolicy::Block,
                    z_index: ZIndex::Global(2),
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Vw(100.),
                        height: Val::Vh(100.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ActionRecorder {
                    action: GameAction::Escape,
                    entity: placeholder,
                },
            ))
            .with_children(|wrapper| {
                wrapper
                    .spawn(NodeBundle {
                        background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                        border_color: BorderColor(Color::Srgba(css::BLUE_VIOLET)),
                        style: Style {
                            border: UiRect::all(Val::Px(2.5)),
                            min_width: Val::Vw(50.),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            padding: UiRect::all(Val::Px(10.)),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|dialog| {
                        dialog.spawn(TextBundle {
                            text: Text::from_section(
                                "Press any key...",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 21.,
                                    color: Color::WHITE,
                                },
                            ),
                            style: Style {
                                margin: UiRect::all(Val::Px(25.)),
                                width: Val::Auto,
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                    });
            });
        });
}

pub fn update_input_component(
    commands: &mut Commands,
    entity: Entity,
    binds: &Vec<KeyCode>,
    assets: &AssetServer,
) {
    commands.entity(entity).despawn_descendants();
    let font: Handle<Font> = assets.load("fonts/gohu.ttf");

    // List all possible binds, and add them as text elements
    for key in binds {
        let child = commands
            .spawn(NodeBundle {
                background_color: BackgroundColor(Color::Srgba(css::BLUE_VIOLET)),
                border_radius: BorderRadius::all(Val::Px(10.)),
                style: Style {
                    padding: UiRect::horizontal(Val::Px(10.)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|k| {
                k.spawn(TextBundle {
                    text: Text::from_section(
                        {
                            // Formats keybinds
                            let mut output = format!("{:?}", key).replace("Key", "");
                            if output.starts_with("Arrow") {
                                if output.ends_with("Left") {
                                    output = "←".into()
                                }
                                if output.ends_with("Right") {
                                    output = "→".into()
                                }
                                if output.ends_with("Up") {
                                    output = "↑".into()
                                }
                                if output.ends_with("Down") {
                                    output = "↓".into()
                                }
                            }
                            output
                        },
                        TextStyle {
                            font: font.clone(),
                            font_size: 21.,
                            color: Color::WHITE,
                        },
                    ),
                    ..Default::default()
                });
            })
            .id();

        commands.entity(entity).add_child(child);
    }
}

pub fn controls_update_system(
    queries: (
        Query<(&Interaction, &EditControlButton, &Children, &mut Style), Changed<Interaction>>,
        Query<(&Interaction, &ClearButton, &mut BackgroundColor), Changed<Interaction>>,
        Query<(&mut ActionRecorder, &mut Visibility)>,
    ),
    mut commands: Commands,
    resources: (Res<AssetServer>, Res<ButtonInput<KeyCode>>, ResMut<KeyMap>),
) {
    let (mut edit_query, mut clear_query, mut visibility_query) = queries;
    let (assets, input, mut key_map) = resources;

    if visibility_query.is_empty() {
        return;
    }

    let (mut recorder, mut vis) = visibility_query.single_mut();

    if *vis == Visibility::Visible {
        if let Some(btn) = input.get_just_pressed().next() {
            *vis = Visibility::Hidden;
            key_map.map.get_mut(&recorder.action).unwrap().push(*btn);
            update_input_component(
                &mut commands,
                recorder.entity,
                key_map.map.get(&recorder.action).unwrap(),
                &assets,
            );
            return;
        }
    }

    for (interaction, btn_action, children, mut style) in edit_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                let kbd_action = btn_action.0;
                // Check if "Clear" button received an event. If so, ignores input
                if clear_query.get(children[2]).is_ok() {
                    continue;
                }
                // Open the "add input" dialog
                *vis = Visibility::Visible;
                recorder.action = kbd_action;
                recorder.entity = children[1];
            }
            Interaction::Hovered => {
                // Show "Clear" button
                style.grid_template_columns.pop();
                style
                    .grid_template_columns
                    .push(RepeatedGridTrack::px(1, 40.));
            }
            Interaction::None => {
                // Hide "clear button"
                style.grid_template_columns.pop();
                style
                    .grid_template_columns
                    .push(RepeatedGridTrack::px(1, 10.));
            }
        }
    }

    for (interaction, clear, mut bg) in clear_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // Clear all binds for this action
                key_map.map.insert(clear.0, Vec::new());
                // Update visual element
                update_input_component(
                    &mut commands,
                    clear.1,
                    key_map.map.get(&clear.0).unwrap(),
                    &assets,
                );
            }
            Interaction::Hovered => {
                bg.0 = Color::Srgba(css::RED);
            }
            Interaction::None => {
                bg.0 = NORMAL_BUTTON;
            }
        }
    }
}