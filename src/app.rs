use crate::{Grid, grid::Tile};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Brush {
    Paint(Tile),
}

impl Default for Brush {
    fn default() -> Self {
        Self::Paint(Tile::Wall)
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct TemplateApp {
    grid: Grid,

    #[serde(skip)]
    brush: Brush,
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_else(|| Self {
                grid: Grid::new(100, 100),
                brush: Brush::default(),
            })
        } else {
            Self {
                grid: Grid::new(100, 100),
                brush: Brush::default(),
            }
        };

        if app.grid.rows == 0 || app.grid.cols == 0 {
            app.grid = Grid::new(100, 100);
        }

        app
    }
}

impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Brush");

            ui.selectable_value(&mut self.brush, Brush::Paint(Tile::Wall), "Wall");
            ui.selectable_value(&mut self.brush, Brush::Paint(Tile::Ground), "Ground");
            ui.selectable_value(&mut self.brush, Brush::Paint(Tile::Water), "Water");
            ui.selectable_value(&mut self.brush, Brush::Paint(Tile::Path), "Path");
            ui.selectable_value(&mut self.brush, Brush::Paint(Tile::Road), "Road");

            ui.separator();
            ui.label("Tip: click + drag to paint");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) =
                ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());
            let rect = response.rect;

            if (response.clicked() || response.dragged()) && response.hovered() {
                if let Some(pos) = response.interact_pointer_pos() {
                    if let Some((row, col)) = self.grid.pos_to_cell(rect, pos) {
                        let Brush::Paint(tile) = self.brush;
                        self.grid.set_tile(row, col, tile);
                    }
                }
            }

            self.grid.draw(rect, &painter);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
