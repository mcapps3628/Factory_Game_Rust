mod build_menu;
mod hud;
mod inventory_menu;

use bevy::prelude::*;
use crate::mode;
use crate::mode::PlayerMode;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                hud::spawn_hud,
                build_menu::spawn_build_menu,
                inventory_menu::spawn_inventory_menu,
            ),
        )
            .add_systems(
                Update,
                (
                    sync_panels,
                    hud::update_hud,
                    build_menu::highlight_tiles,
                    inventory_menu::update_counts,
                    inventory_menu::highlight_items,
                    mode::update_cursor,
                ),
            );
    }
}

/// Put this on a panel's root node. The panel is shown only while the
/// player is in that mode, and hidden otherwise.
#[derive(Component)]
pub struct ModePanel(pub PlayerMode);

/// Display::None removes a node from layout AND interaction, which is why
/// it's used instead of Visibility::Hidden.
fn sync_panels(
    state: Res<State<PlayerMode>>,
    mut panels: Query<(&ModePanel, &mut Node)>,
) {
    if !state.is_changed() {
        return;
    }
    for (panel, mut node) in &mut panels {
        node.display = if panel.0 == *state.get() {
            Display::Flex
        } else {
            Display::None
        };
    }
}

/// Darkens a color; used for "not selected" buttons.
fn dimmed(color: Color) -> Color {
    let c = color.to_srgba();
    Color::srgb(c.red * 0.45, c.green * 0.45, c.blue * 0.45)
}