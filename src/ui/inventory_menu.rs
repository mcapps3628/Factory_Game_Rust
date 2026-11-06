use bevy::prelude::*;
use bevy::ui_widgets::Button;
use crate::player_inventory::{PlayerInventory, SelectedItem};
use crate::item::ItemType;

#[derive(Component)]
pub struct InventoryButton(pub ItemType);

pub fn spawn_inventory_menu(mut commands: Commands) {
    // Toolbar, bottom-right. One button per tile type, generated from the definitions.
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                right: Val::Px(10.0),
                column_gap: Val::Px(8.0),
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        ))
        .with_children(|bar| {
            for item_type in ItemType::ALL {
                let def = item_type.def();
                bar.spawn((
                    Button,
                    InventoryButton(item_type),
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

pub fn handle_inventory_buttons(
    query: Query<(&Interaction, &InventoryButton), Changed<Interaction>>,
    mut selected: ResMut<SelectedItem>,
) {
    for (interaction, button) in &query {
        if *interaction == Interaction::Pressed {
            selected.0 = button.0;
        }
    }
}

pub fn highlight_selected(
    selected: Res<SelectedItem>,
    mut query: Query<(&InventoryButton, &mut BackgroundColor)>,
) {
    if !selected.is_changed() {
        return;
    }
    for (button, mut bg) in &mut query {
        let alpha = if button.0 == selected.0 { 1.0 } else { 0.4 };
        bg.0 = button.0.def().color.with_alpha(alpha);
    }
}