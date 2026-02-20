use crate::screen_wrapper::RandomApp;

mod game_of_life;
pub mod app_screen;
mod screen_wrapper;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native("My Game of Life App", options, Box::new(|cc| Ok(Box::new(RandomApp::new(cc)))))
}
