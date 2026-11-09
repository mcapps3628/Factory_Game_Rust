use bevy::prelude::*;
use bevy::window::{CursorIcon, PrimaryWindow, SystemCursorIcon};

use crate::tile::{Direction, PlaceDirection, SelectedTile, TileType};

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerMode {
    #[default]
    Build,
    Normal,
    Inventory,
}

pub struct ModePlugin;

impl Plugin for ModePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayerMode>()
            .add_systems(Update, change_mode);
    }
}

fn change_mode(
    keys: Res<ButtonInput<KeyCode>>,
    current: Res<State<PlayerMode>>,
    mut next: ResMut<NextState<PlayerMode>>,
) {
    let mode = *current.get();
    let toggle = |target: PlayerMode| {
        if mode == target {
            PlayerMode::Normal
        } else {
            target
        }
    };

    if keys.just_pressed(KeyCode::KeyB) {
        next.set(toggle(PlayerMode::Build));
    }
    if keys.just_pressed(KeyCode::KeyE) {
        next.set(toggle(PlayerMode::Inventory));
    }
    if keys.just_pressed(KeyCode::Escape) {
        next.set(PlayerMode::Normal);
    }
}

pub fn cursor_not_over_ui(interactions: Query<&Interaction>) -> bool {
    interactions.iter().all(|i| *i == Interaction::None)
}

/// Cursor handling

pub fn update_cursor(
    mut commands: Commands,
    window: Single<Entity, With<PrimaryWindow>>,
    state: Res<State<PlayerMode>>,
    selected: Res<SelectedTile>,
    direction: Res<PlaceDirection>,
) {
    // Only do work when something that affects the icon changed
    if !(state.is_changed() || selected.is_changed() || direction.is_changed()) {
        return;
    }

    let icon = match state.get() {
        PlayerMode::Build if selected.0 == TileType::Conveyor => direction_cursor(direction.0),
        PlayerMode::Build => SystemCursorIcon::Pointer,
        PlayerMode::Inventory => SystemCursorIcon::Pointer,
        PlayerMode::Normal => SystemCursorIcon::Default,
    };

    commands.entity(*window).insert(CursorIcon::from(icon));
}

fn direction_cursor(d: Direction) -> SystemCursorIcon {
    match d {
        Direction::North => SystemCursorIcon::NResize,
        Direction::East => SystemCursorIcon::EResize,
        Direction::South => SystemCursorIcon::SResize,
        Direction::West => SystemCursorIcon::WResize,
    }
}
