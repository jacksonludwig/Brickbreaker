use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::Context;
use ggez::GameResult;

pub struct State {
    pos_x: f32,
}

impl State {
    pub fn new() -> GameResult<State> {
        let state = State { pos_x: 0.0 };
        Ok(state)
    }
}

impl event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(0.0, 0.0),
            100.0,
            2.0,
            graphics::WHITE,
        )?;

        graphics::draw(ctx, &circle, (na::Point2::new(self.pos_x, 380.0_f32),))?;

        graphics::present(ctx)?;
        println!("FPS: {}", ggez::timer::fps(ctx));
        Ok(())
    }
}
