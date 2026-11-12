use bevy::prelude::*;

pub const GRID_SIZE: IVec2 = IVec2::new(16, 16);
pub const TILE_SIZE: f32 = 48.0;

#[derive(Resource)]
pub struct Grid {
    cells: Vec<Option<Entity>>, // private: only reachable through the methods below
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            cells: vec![None; (GRID_SIZE.x * GRID_SIZE.y) as usize],
        }
    }
}

impl Grid {
    pub fn in_bounds(pos: IVec2) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < GRID_SIZE.x && pos.y < GRID_SIZE.y
    }

    fn index(pos: IVec2) -> Option<usize> {
        Self::in_bounds(pos).then(|| (pos.y * GRID_SIZE.x + pos.x) as usize)
    }

    pub fn get(&self, pos: IVec2) -> Option<Entity> {
        Self::index(pos).and_then(|i| self.cells[i])
    }

    pub fn set(&mut self, pos: IVec2, entity: Entity) {
        if let Some(i) = Self::index(pos) {
            self.cells[i] = Some(entity);
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self, pos: IVec2) {
        if let Some(i) = Self::index(pos) {
            self.cells[i] = None;
        }
    }
}

pub fn grid_to_world(pos: IVec2) -> Vec2 {
    (pos.as_vec2() - GRID_SIZE.as_vec2() / 2.0 + 0.5) * TILE_SIZE
}

pub fn world_to_grid(world: Vec2) -> Option<IVec2> {
    let pos = (world / TILE_SIZE + GRID_SIZE.as_vec2() / 2.0)
        .floor()
        .as_ivec2();
    Grid::in_bounds(pos).then_some(pos)
}

// Window cursor -> grid cell. Shared by every tool that needs the hovered cell.
pub fn cursor_grid_pos(
    window: &Window,
    camera: &Camera,
    cam_t: &GlobalTransform,
) -> Option<IVec2> {
    let cursor = window.cursor_position()?;
    let world = camera.viewport_to_world_2d(cam_t, cursor).ok()?;
    world_to_grid(world)
}

// Draws a checkerboard so you can see the cells.
pub fn spawn_grid_background(mut commands: Commands) {
    for y in 0..GRID_SIZE.y {
        for x in 0..GRID_SIZE.x {
            let pos = grid_to_world(IVec2::new(x, y));
            let shade = if (x + y) % 2 == 0 { 0.16 } else { 0.20 };
            commands.spawn((
                Sprite::from_color(
                    Color::srgb(shade, shade, shade),
                    Vec2::splat(TILE_SIZE - 2.0),
                ),
                Transform::from_translation(pos.extend(-1.0)),
            ));
        }
    }
}

