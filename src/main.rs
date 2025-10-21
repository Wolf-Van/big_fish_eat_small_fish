mod app;
mod game;
mod ui;
mod input;
mod render;
mod enemy;
mod database;

fn main() {
    app::run_ui().unwrap();
}
