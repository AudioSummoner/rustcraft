use crate::{
    ui::{assets::load_background_image, style::NORMAL_BUTTON},
    TEXT_COLOR,
};
use bevy::{
    asset::AssetServer,
    color::palettes::css::CRIMSON,
    prelude::{
        BuildChildren, Button, ButtonBundle, Changed, Commands, Component, Entity, NodeBundle,
        Query, Res, ResMut, Resource, StateScoped, TextBundle, With,
    },
    text::TextStyle,
    ui::{
        AlignItems, BackgroundColor, FlexDirection, Interaction, JustifyContent, Style, UiImage,
        UiRect, Val,
    },
};

use super::{MenuButtonAction, MenuState, SelectedOption};

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
    Low,
    Medium,
    High,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(pub u32);

pub fn settings_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background_image = load_background_image(&asset_server);

    let button_style = Style {
        width: Val::Px(400.0),
        height: Val::Px(60.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };

    let button_text_style = TextStyle {
        font_size: 33.0,
        color: TEXT_COLOR,
        ..Default::default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            UiImage::new(background_image), // Set the background image
            StateScoped(MenuState::Settings),
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    for (action, text) in [
                        (MenuButtonAction::SettingsDisplay, "Display"),
                        (MenuButtonAction::SettingsSound, "Sound"),
                        (MenuButtonAction::SettingsControls, "Controls"),
                        (MenuButtonAction::BackToMainMenu, "Back"),
                    ] {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    ..Default::default()
                                },
                                action,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    text,
                                    button_text_style.clone(),
                                ));
                            });
                    }
                });
        });
}

pub fn display_settings_menu_setup(mut commands: Commands, display_quality: Res<DisplayQuality>) {
    let button_style = Style {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };
    let button_text_style = TextStyle {
        font_size: 33.0,
        color: TEXT_COLOR,
        ..Default::default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            StateScoped(MenuState::SettingsDisplay),
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: CRIMSON.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Create a new `NodeBundle`, this time not setting its `flex_direction`. It will
                    // use the default value, `FlexDirection::Row`, from left to right.
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: CRIMSON.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // Display a label for the current setting
                            parent.spawn(TextBundle::from_section(
                                "Display Quality",
                                button_text_style.clone(),
                            ));
                            // Display a button for each possible value
                            for quality_setting in [
                                DisplayQuality::Low,
                                DisplayQuality::Medium,
                                DisplayQuality::High,
                            ] {
                                let mut entity = parent.spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(150.0),
                                            height: Val::Px(65.0),
                                            ..button_style.clone()
                                        },
                                        background_color: NORMAL_BUTTON.into(),
                                        ..Default::default()
                                    },
                                    quality_setting,
                                ));
                                entity.with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        format!("{quality_setting:?}"),
                                        button_text_style.clone(),
                                    ));
                                });
                                if *display_quality == quality_setting {
                                    entity.insert(SelectedOption);
                                }
                            }
                        });
                    // Display the back button to return to the settings screen
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                background_color: NORMAL_BUTTON.into(),
                                ..Default::default()
                            },
                            MenuButtonAction::BackToSettings,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Back", button_text_style));
                        });
                });
        });
}

pub fn sound_settings_menu_setup(mut commands: Commands, volume: Res<Volume>) {
    let button_style = Style {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };
    let button_text_style = TextStyle {
        font_size: 33.0,
        color: TEXT_COLOR,
        ..Default::default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            StateScoped(MenuState::SettingsSound),
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: CRIMSON.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: CRIMSON.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Volume",
                                button_text_style.clone(),
                            ));
                            for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
                                let mut entity = parent.spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(30.0),
                                            height: Val::Px(65.0),
                                            ..button_style.clone()
                                        },
                                        background_color: NORMAL_BUTTON.into(),
                                        ..Default::default()
                                    },
                                    Volume(volume_setting),
                                ));
                                if *volume == Volume(volume_setting) {
                                    entity.insert(SelectedOption);
                                }
                            }
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                background_color: NORMAL_BUTTON.into(),
                                ..Default::default()
                            },
                            MenuButtonAction::BackToSettings,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Back", button_text_style));
                        });
                });
        });
}

// This system updates the settings when a new value for a setting is selected, and marks
// the button as the one currently selected
pub fn setting_button<T: Resource + Component + PartialEq + Copy>(
    interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
    mut selected_query: Query<(Entity, &mut BackgroundColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<T>,
) {
    for (interaction, button_setting, entity) in &interaction_query {
        if *interaction == Interaction::Pressed && *setting != *button_setting {
            let (previous_button, mut previous_button_color) = selected_query.single_mut();
            *previous_button_color = NORMAL_BUTTON.into();
            commands.entity(previous_button).remove::<SelectedOption>();
            commands.entity(entity).insert(SelectedOption);
            *setting = *button_setting;
        }
    }
}
