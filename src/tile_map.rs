use macroquad::prelude::*;

pub const TILE_SIZE: f32 = 32.0;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TileType {
    Floor,
    Wall,
}

impl TileType {
    pub fn is_walkable(self) -> bool {
        matches!(self, TileType::Floor)
    }

    fn color(self) -> Color {
        match self {
            TileType::Floor => Color::from_rgba(60, 60, 80, 255),
            TileType::Wall => Color::from_rgba(100, 80, 60, 255),
        }
    }
}

pub struct TileMap {
    tiles: Vec<Vec<TileType>>,
    pub width: usize,
    pub height: usize,
}

impl TileMap {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![vec![TileType::Floor; width]; height];
        Self {
            tiles,
            width,
            height,
        }
    }

    pub fn create_test_level() -> Self {
        let width = 20;
        let height = 15;
        let mut map = Self::new(width, height);

        // Add walls around the border
        for x in 0..width {
            map.set_tile(x, 0, TileType::Wall);
            map.set_tile(x, height - 1, TileType::Wall);
        }
        for y in 0..height {
            map.set_tile(0, y, TileType::Wall);
            map.set_tile(width - 1, y, TileType::Wall);
        }

        // Add some interior walls
        for x in 5..10 {
            map.set_tile(x, 5, TileType::Wall);
        }
        for y in 8..12 {
            map.set_tile(12, y, TileType::Wall);
        }

        map
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<TileType> {
        self.tiles.get(y).and_then(|row| row.get(x)).copied()
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: TileType) {
        if y < self.height && x < self.width {
            self.tiles[y][x] = tile;
        }
    }

    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 {
            return false;
        }
        self.get_tile(x as usize, y as usize)
            .map(|t| t.is_walkable())
            .unwrap_or(false)
    }

    pub fn draw(&self) {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                draw_rectangle(
                    x as f32 * TILE_SIZE,
                    y as f32 * TILE_SIZE,
                    TILE_SIZE,
                    TILE_SIZE,
                    tile.color(),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_walkability() {
        assert!(TileType::Floor.is_walkable());
        assert!(!TileType::Wall.is_walkable());
    }

    #[test]
    fn test_map_boundaries() {
        let map = TileMap::create_test_level();
        // Corners should be walls
        assert!(!map.is_walkable(0, 0));
        assert!(!map.is_walkable(19, 0));
        assert!(!map.is_walkable(0, 14));
        assert!(!map.is_walkable(19, 14));
        // Interior should be walkable (except where we placed walls)
        assert!(map.is_walkable(1, 1));
        assert!(map.is_walkable(2, 2));
    }

    #[test]
    fn test_out_of_bounds() {
        let map = TileMap::new(10, 10);
        assert!(!map.is_walkable(-1, 0));
        assert!(!map.is_walkable(0, -1));
        assert!(!map.is_walkable(100, 0));
        assert!(!map.is_walkable(0, 100));
    }
}
