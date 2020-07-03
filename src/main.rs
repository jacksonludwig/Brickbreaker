mod game;

use game::State;
use ggez::{event, GameResult};

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut State::new()?;
    event::run(ctx, event_loop, state)
}
