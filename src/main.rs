use ggez::{conf, event, Context, GameResult};
use std::path;

struct Game {}

impl event::EventHandler for Game {
    fn update( &mut self, _context: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw( &mut self, _context: &mut Context) -> GameResult {
        Ok(())
    }
}


pub fn main() -> GameResult {
    let context_builder = ggez::ContextBuilder::new("rust_sokoban","sokoban" )
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0,600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = &mut context_builder.build()?;
    let game = &mut Game{};
    event::run(context, event_loop,game)
}
