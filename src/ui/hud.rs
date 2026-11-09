use bevy::prelude::*;

use crate::economy::Money;
use crate::mode::PlayerMode;

#[derive(Component)]
pub struct MoneyText;

#[derive(Component)]
pub struct ModeText;

pub fn spawn_hud(mut commands: Commands) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0),
            ..default()
        })
        .with_children(|hud| {
            hud.spawn((
                MoneyText,
                Text::new(""),
                TextColor(Color::WHITE),
                TextFont {
                    font_size: FontSize::Px(20.0),
                    ..default()
                },
            ));
            hud.spawn((
                ModeText,
                Text::new(""),
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                TextFont {
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
            ));
        });
}

pub fn update_hud(
    money: Res<Money>,
    state: Res<State<PlayerMode>>,
    mut money_text: Single<&mut Text, (With<MoneyText>, Without<ModeText>)>,
    mut mode_text: Single<&mut Text, With<ModeText>>,
) {
    if money.is_changed() {
        money_text.0 = format!("Money: ${}", money.0);
    }
    if state.is_changed() {
        mode_text.0 = format!(
            "Mode: {:?}   [B] build   [E] inventory   [Esc] cancel",
            state.get()
        );
    }
}