use bevy::prelude::*;
use bevy::ui_widgets::Button;
use crate::economy::Money;
use crate::tile::{SelectedTile, TileType};

#[derive(Component)]
pub struct BuildButton(pub TileType);

#[derive(Component)]
pub struct MoneyText;

pub fn spawn_build_menu(mut commands: Commands) {
    // Money label, top-left
    commands.spawn((
        Text::new(""),
        TextColor(Color::WHITE),
        MoneyText,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));

    // Toolbar, bottom-left. One button per tile type, generated from the definitions.
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                left: Val::Px(10.0),
                column_gap: Val::Px(8.0),
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        ))
        .with_children(|bar| {
            for tile_type in TileType::ALL {
                let def = tile_type.def();
                bar.spawn((
                    Button,
                    BuildButton(tile_type),
                    Node {
                        width: Val::Px(90.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(def.color),
                ))
                    .with_children(|button| {
                        button.spawn((
                            Text::new(format!("{}\n${}", def.name, def.cost)),
                            TextColor(Color::WHITE),
                        ));
                    });
            }
        });
}

pub fn handle_build_buttons(
    query: Query<(&Interaction, &BuildButton), Changed<Interaction>>,
    mut selected: ResMut<SelectedTile>,
) {
    for (interaction, button) in &query {
        if *interaction == Interaction::Pressed {
            selected.0 = button.0;
        }
    }
}

pub fn highlight_selected(
    selected: Res<SelectedTile>,
    mut query: Query<(&BuildButton, &mut BackgroundColor)>,
) {
    if !selected.is_changed() {
        return;
    }
    for (button, mut bg) in &mut query {
        let alpha = if button.0 == selected.0 { 1.0 } else { 0.4 };
        bg.0 = button.0.def().color.with_alpha(alpha);
    }
}

pub fn update_money_text(money: Res<Money>, mut text: Single<&mut Text, With<MoneyText>>) {
    if money.is_changed() {
        text.0 = format!("Money: ${}", money.0);
    }
}