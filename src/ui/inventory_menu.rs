use bevy::prelude::*;
use bevy::ui_widgets::{Activate, Button};

use super::ModePanel;
use crate::item::ItemType;
use crate::mode::PlayerMode;
use crate::player_inventory::{PlayerInventory, SelectedItem};

/// Item Name Text
#[derive(Component)]
pub struct ItemButton(pub ItemType);

/// Item Count Text
#[derive(Component)]
pub struct ItemCountText(pub ItemType);

const ITEM_NORMAL: Color = Color::srgb(0.18, 0.18, 0.22);
const ITEM_SELECTED: Color = Color::srgb(0.30, 0.45, 0.30);

pub fn spawn_inventory_menu(mut commands: Commands) {
    commands
        .spawn((
            ModePanel(PlayerMode::Inventory), // only visible in Inventory mode
            Interaction::default(),
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                left: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
        ))
        .with_children(|panel| {
            panel.spawn((
                Text::new("Inventory"),
                TextColor(Color::WHITE),
                TextFont {
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
            ));

            // One row per item type and counts are updated by update_counts
            for item in ItemType::ALL {
                panel
                    .spawn((
                        Button,
                        Interaction::default(),
                        ItemButton(item),
                        Node {
                            width: Val::Px(200.0),
                            height: Val::Px(34.0),
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            padding: UiRect::horizontal(Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor(ITEM_NORMAL),
                    ))
                    .observe(move |_: On<Activate>, mut selected: ResMut<SelectedItem>| {
                        selected.0 = item;
                    })
                    .with_children(|row| {
                        row.spawn((
                            Text::new(format!("{}", item.def().name)),
                            TextColor(Color::WHITE),
                            TextFont {
                                font_size: FontSize::Px(16.0),
                                ..default()
                            },
                        ));
                        row.spawn((
                            ItemCountText(item),
                            Text::new("0"),
                            TextColor(Color::WHITE),
                            TextFont {
                                font_size: FontSize::Px(16.0),
                                ..default()
                            },
                        ));
                    });
            }

            panel.spawn((
                Text::new("Left-click: Insert item (Shift = all)\nRight-click: Collect\nS: Sell (Shift = all)"),
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                TextFont {
                    font_size: FontSize::Px(12.0),
                    ..default()
                },
            ));
        });
}

pub fn update_counts(
    player: Res<PlayerInventory>,
    mut texts: Query<(&ItemCountText, &mut Text)>,
) {
    if !player.is_changed() {
        return;
    }
    for (count, mut text) in &mut texts {
        let n = player.items.get(&count.0).copied().unwrap_or(0);
        text.0 = n.to_string();
    }
}

pub fn highlight_items(
    selected: Res<SelectedItem>,
    mut buttons: Query<(&ItemButton, &mut BackgroundColor)>,
) {
    if !selected.is_changed() {
        return;
    }
    for (button, mut bg) in &mut buttons {
        bg.0 = if button.0 == selected.0 {
            ITEM_SELECTED
        } else {
            ITEM_NORMAL
        };
    }
}