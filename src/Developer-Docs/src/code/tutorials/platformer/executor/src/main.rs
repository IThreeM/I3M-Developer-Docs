//! Executor with your game connected to it as a plugin.
use i3m::engine::executor::Executor;
use platformer::Game;

fn main() {
    let mut executor = Executor::new();
    executor.add_plugin(Game::default());
    executor.run()
}
