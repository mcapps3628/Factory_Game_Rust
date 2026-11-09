use bevy::prelude::*;
use bevy::ui_widgets::{Activate, Button};

use super::{dimmed, ModePanel};
use crate::mode::PlayerMode;
use crate::tile::{SelectedTile, TileType};

/// On each tile button: which tile it selects.
#[derive(Component)]
pub struct TileButton(pub TileType);

pub fn spawn_build_menu(mut commands: Commands) {
    commands
        .spawn((
            ModePanel(PlayerMode::Build), // only visible in Build mode
            Interaction::default(),       // so the empty panel area counts as "over UI"
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                left: Val::Px(10.0),
                column_gap: Val::Px(8.0),
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
        ))
        .with_children(|bar| {
            for tile_type in TileType::ALL {
                let def = tile_type.def();

                bar.spawn((
                    Button,
                    Interaction::default(),
                    TileButton(tile_type),
                    Node {
                        width: Val::Px(100.0),
                        height: Val::Px(64.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        row_gap: Val::Px(2.0),
                        ..default()
                    },
                    BackgroundColor(def.color),
                ))
                    // The observer asks for its own resource; it can't borrow from this system
                    .observe(move |_: On<Activate>, mut selected: ResMut<SelectedTile>| {
                        selected.0 = tile_type;
                    })
                    .with_children(|button| {
                        button.spawn((
                            Text::new(def.name),
                            TextColor(Color::WHITE),
                            TextFont {
                                font_size: FontSize::Px(16.0),
                                ..default()
                            },
                        ));
                        button.spawn((
                            Text::new(format!("${}", def.cost)),
                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            TextFont {
                                font_size: FontSize::Px(12.0),
                                ..default()
                            },
                        ));
                    });
            }
        });
}

/// Selected tile = full color, others = darkened.
pub fn highlight_tiles(
    selected: Res<SelectedTile>,
    mut buttons: Query<(&TileButton, &mut BackgroundColor)>,
) {
    if !selected.is_changed() {
        return;
    }
    for (button, mut bg) in &mut buttons {
        let color = button.0.def().color;
        bg.0 = if button.0 == selected.0 {
            color
        } else {
            dimmed(color)
        };
    }
}