use std::time::{Duration, Instant};
use eframe::{egui, Frame};
use eframe::egui::Context;
use rand::{rng, RngExt};

pub struct GameOfLife {
    rows: usize,
    cols: usize,
    field: Vec<u8>,
}

impl GameOfLife {
    const DIRECTIONS: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),  (0, 1),
        (1, -1),  (1, 0), (1, 1),
    ];

    fn idx(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    pub fn generate_game(rows: usize, cols: usize, alive_chance: f64) -> Self {
        let mut rng = rand::rng();
        let field = (0..rows*cols)
            .map(|_| if rng.random_bool(alive_chance) { 1u8 } else { 0u8 })
            .collect::<Vec<u8>>();
        Self { rows, cols, field }
    }

    fn alive_neighbors(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;
        for (dx, dy) in Self::DIRECTIONS {
            let new_row = row as isize + dy;
            let new_col = col as isize + dx;
            if new_row >= 0 && new_row < self.rows as isize &&
                new_col >= 0 && new_col < self.cols as isize
            {
                let idx = self.idx(new_row as usize, new_col as usize);
                count += self.field[idx];
            }
        }
        count
    }

    pub fn update(&mut self) {
        let mut new_field = self.field.clone();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let idx = self.idx(row, col);
                let neighbors = self.alive_neighbors(row, col);

                new_field[idx] = match (self.field[idx], neighbors) {
                    (1, 2) | (1, 3) => 1,
                    (0, 3) => 1,
                    _ => 0,
                };
            }
        }

        self.field = new_field;
    }
}
pub struct GameOfLifeApp {
    game: GameOfLife,
    running: bool,
    last_update: Instant,
}

impl GameOfLifeApp {
    pub fn new() -> Self {
        Self {
            game: GameOfLife::generate_game(30, 40, 0.21),
            running: false,
            last_update: Instant::now()
        }
    }

    fn draw_grid(&mut self, ui: &mut egui::Ui) {
        let cell_size = 15.0;
        let grid_size = egui::Vec2::new(self.game.cols as f32 * cell_size, self.game.rows as f32 * cell_size);

        let (response, painter) = ui.allocate_painter(grid_size, egui::Sense::click());
        let origin = response.rect.left_top();

        for row in 0..self.game.rows {
            for col in 0..self.game.cols {
                let idx = self.game.idx(row, col);
                if self.game.field[idx] == 1 {
                    let rect = egui::Rect::from_min_size(
                        egui::pos2(origin.x + col as f32 * cell_size, origin.y + row as f32 * cell_size),
                        egui::vec2(cell_size, cell_size),
                    );
                    painter.rect_filled(rect, 0.0, egui::Color32::WHITE);
                }
            }
        }

        for i in 0..=self.game.rows {
            let y = origin.y + i as f32 * cell_size;
            painter.line_segment([egui::pos2(origin.x, y), egui::pos2(origin.x + grid_size.x, y)], (1.0, egui::Color32::GRAY));
        }
        for j in 0..=self.game.cols {
            let x = origin.x + j as f32 * cell_size;
            painter.line_segment([egui::pos2(x, origin.y), egui::pos2(x, origin.y + grid_size.y)], (1.0, egui::Color32::GRAY));
        }

        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                let col = ((pos.x - origin.x) / cell_size) as usize;
                let row = ((pos.y - origin.y) / cell_size) as usize;
                if row < self.game.rows && col < self.game.cols {
                    let idx = self.game.idx(row, col);
                    self.game.field[idx] ^= 1;
                }
            }
        }
    }
}

impl GameOfLifeApp {
    pub fn update(&mut self, ctx: &Context, _frame: &mut Frame) -> bool {
        let mut go_back = false;

        if self.running && self.last_update.elapsed() > Duration::from_millis(200){
            self.game.update();
            self.last_update = Instant::now();
        }

        ctx.request_repaint();

        egui::TopBottomPanel::top("Controls").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("⬅ Back").clicked() {
                    go_back = true;
                }
                if ui.button("Play").clicked() {
                    self.running = true;
                }
                if ui.button("Stop").clicked() {
                    self.running = false;
                }
                if ui.button("Next").clicked(){
                    self.game.update();
                }
                if ui.button("Generate Random").clicked() {
                    self.game = GameOfLife::generate_game(
                        rand::rng().random_range(10..60),
                        rand::rng().random_range(10..60),
                        rand::rng().random(),
                    );
                }
                if ui.button("Generate Blank").clicked(){
                    self.game = GameOfLife::generate_game(
                        rand::rng().random_range(10..60),
                        rand::rng().random_range(10..60),
                        0.0,
                    );
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_grid(ui);
        });

        go_back
    }
}