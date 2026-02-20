use eframe::{CreationContext, Frame};
use eframe::egui::Context;
use crate::app_screen::AppScreen;
use crate::game_of_life::main::GameOfLifeApp;

pub struct RandomApp {
    screen: AppScreen
}

impl RandomApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            screen: AppScreen::StartMenu
        }
    }
}

impl eframe::App for RandomApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        match &mut self.screen {

            AppScreen::StartMenu => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(100.0);
                        ui.heading("Game Launcher");
                        ui.add_space(50.0);

                        let size = egui::vec2(250.0, 120.0);

                        if ui.add_sized(size, egui::Button::new("Game of Life")).clicked() {
                            self.screen = AppScreen::GameOfLife(GameOfLifeApp::new());
                        }
                    });
                });
            }

            AppScreen::GameOfLife(game) => {
                if game.update(ctx, frame) {
                    self.screen = AppScreen::StartMenu;
                }
            }
        }
    }
}