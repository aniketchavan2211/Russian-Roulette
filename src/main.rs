mod app;
mod config;
mod game;
mod ui;

use app::App;

fn main() -> Result<(), std::io::Error> {
    App::run()
}
