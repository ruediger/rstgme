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

    pub fn create_random(width: usize, height: usize) -> Self {
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

        // Add random wall clusters
        let num_clusters = (width * height) / 50;
        for _ in 0..num_clusters {
            let cx = rand::gen_range(3, width - 3);
            let cy = rand::gen_range(3, height - 3);
            let cluster_size = rand::gen_range(2, 6);

            // Create a small cluster of walls
            for _ in 0..cluster_size {
                let ox = rand::gen_range(0, 3) as i32 - 1;
                let oy = rand::gen_range(0, 3) as i32 - 1;
                let wx = (cx as i32 + ox) as usize;
                let wy = (cy as i32 + oy) as usize;
                if wx > 1 && wx < width - 2 && wy > 1 && wy < height - 2 {
                    map.set_tile(wx, wy, TileType::Wall);
                }
            }
        }

        // Add some horizontal wall segments
        let num_h_walls = height / 8;
        for _ in 0..num_h_walls {
            let y = rand::gen_range(3, height - 3);
            let x_start = rand::gen_range(2, width / 2);
            let length = rand::gen_range(4, width / 3);
            for x in x_start..(x_start + length).min(width - 2) {
                map.set_tile(x, y, TileType::Wall);
            }
            // Leave a gap for passage
            let gap = rand::gen_range(x_start, (x_start + length).min(width - 2));
            map.set_tile(gap, y, TileType::Floor);
        }

        // Add some vertical wall segments
        let num_v_walls = width / 8;
        for _ in 0..num_v_walls {
            let x = rand::gen_range(3, width - 3);
            let y_start = rand::gen_range(2, height / 2);
            let length = rand::gen_range(4, height / 3);
            for y in y_start..(y_start + length).min(height - 2) {
                map.set_tile(x, y, TileType::Wall);
            }
            // Leave a gap for passage
            let gap = rand::gen_range(y_start, (y_start + length).min(height - 2));
            map.set_tile(x, gap, TileType::Floor);
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

    pub fn draw(&self, camera_x: f32, camera_y: f32) {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                draw_rectangle(
                    x as f32 * TILE_SIZE - camera_x,
                    y as f32 * TILE_SIZE - camera_y,
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
        let map = TileMap::create_random(20, 15);
        // Corners should be walls (border)
        assert!(!map.is_walkable(0, 0));
        assert!(!map.is_walkable(19, 0));
        assert!(!map.is_walkable(0, 14));
        assert!(!map.is_walkable(19, 14));
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
