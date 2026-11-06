use bevy::prelude::*;

mod economy;
mod item;
mod tile;
mod ui;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, tile::TilePlugin, ui::UiPlugin))
        .insert_resource(economy::Money(500))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
