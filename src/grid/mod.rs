#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Tile {
    Wall,
    Ground,
    Water,
    Path,
    Road,
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Ground
    }
}

impl Tile {
    pub fn cost(self) -> Option<u32> {
        match self {
            Tile::Wall => None,
            Tile::Ground => Some(10),
            Tile::Path => Some(8),
            Tile::Road => Some(6),
            Tile::Water => Some(30),
        }
    }

    pub fn color(self) -> egui::Color32 {
        match self {
            Tile::Wall => egui::Color32::from_rgb(80, 86, 96),
            Tile::Ground => egui::Color32::from_rgb(28, 32, 38),
            Tile::Water => egui::Color32::from_rgb(45, 85, 150),
            Tile::Path => egui::Color32::from_rgb(90, 75, 50),
            Tile::Road => egui::Color32::from_rgb(70, 70, 70),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default, Debug, Clone, Copy)]
pub struct Cell {
    tile: Tile,
}

impl Cell {
    pub fn tile(&self) -> Tile {
        self.tile
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Grid {
    pub rows: usize,
    pub cols: usize,
    matrix: Vec<Cell>,
}

impl Grid {
    const BG_COLOR: egui::Color32 = egui::Color32::from_rgb(16, 18, 22);
    const GRID_LINE: egui::Color32 = egui::Color32::from_rgb(40, 44, 52);

    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            matrix: vec![Cell::default(); rows.saturating_mul(cols)],
        }
    }

    #[inline]
    fn index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&Cell> {
        if row < self.rows && col < self.cols {
            Some(&self.matrix[self.index(row, col)])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
        if row < self.rows && col < self.cols {
            let i = self.index(row, col);
            Some(&mut self.matrix[i])
        } else {
            None
        }
    }

    pub fn set_tile(&mut self, row: usize, col: usize, tile: Tile) {
        if let Some(cell) = self.get_mut(row, col) {
            cell.tile = tile;
        }
    }

    pub fn tile(&self, row: usize, col: usize) -> Option<Tile> {
        self.get(row, col).map(|cell| cell.tile)
    }

    fn layout(&self, rect: egui::Rect) -> Option<(f32, egui::Pos2)> {
        if self.rows == 0 || self.cols == 0 {
            return None;
        }
        let cell_side = (rect.width() / self.cols as f32).min(rect.height() / self.rows as f32);
        if cell_side <= 0.0 {
            return None;
        }
        let grid_w = self.cols as f32 * cell_side;
        let grid_h = self.rows as f32 * cell_side;
        let origin = rect.min
            + egui::vec2(
                (rect.width() - grid_w) * 0.5,
                (rect.height() - grid_h) * 0.5,
            );
        Some((cell_side, origin))
    }

    pub fn cell_rect(&self, rect: egui::Rect, row: usize, col: usize) -> Option<egui::Rect> {
        if row >= self.rows || col >= self.cols {
            return None;
        }
        let (cell_side, origin) = self.layout(rect)?;
        let min = origin + egui::vec2(col as f32 * cell_side, row as f32 * cell_side);
        Some(egui::Rect::from_min_size(
            min,
            egui::vec2(cell_side, cell_side),
        ))
    }

    pub fn pos_to_cell(&self, rect: egui::Rect, pos: egui::Pos2) -> Option<(usize, usize)> {
        let (cell_side, origin) = self.layout(rect)?;
        let local = pos - origin;
        if local.x < 0.0 || local.y < 0.0 {
            return None;
        }
        let c = (local.x / cell_side).floor() as isize;
        let r = (local.y / cell_side).floor() as isize;
        if r >= 0 && c >= 0 {
            let (r, c) = (r as usize, c as usize);
            if r < self.rows && c < self.cols {
                return Some((r, c));
            }
        }
        None
    }

    pub fn draw(&self, rect: egui::Rect, painter: &egui::Painter) {
        if self.rows == 0 || self.cols == 0 {
            return;
        }

        painter.rect_filled(rect, 0.0, Self::BG_COLOR);

        let Some((cell_side, origin)) = self.layout(rect) else {
            return;
        };

        // Draw tiles
        for row in 0..self.rows {
            for col in 0..self.cols {
                let i = self.index(row, col);
                let tile = self.matrix[i].tile;
                let min = origin + egui::vec2(col as f32 * cell_side, row as f32 * cell_side);
                let cell_rect = egui::Rect::from_min_size(min, egui::vec2(cell_side, cell_side));
                painter.rect_filled(cell_rect, 0.0, tile.color());
            }
        }

        let stroke = egui::Stroke::new(1.0, Self::GRID_LINE);

        let w = self.cols as f32 * cell_side;
        let h = self.rows as f32 * cell_side;

        for r in 0..=self.rows {
            let y = origin.y + r as f32 * cell_side;
            painter.line_segment(
                [egui::pos2(origin.x, y), egui::pos2(origin.x + w, y)],
                stroke,
            );
        }
        for c in 0..=self.cols {
            let x = origin.x + c as f32 * cell_side;
            painter.line_segment(
                [egui::pos2(x, origin.y), egui::pos2(x, origin.y + h)],
                stroke,
            );
        }
    }
}

