use crate::App;
use ratatui::{
    style::Color,
    widgets::{
        Block, Widget,
        canvas::{Canvas, Map},
    },
};

impl App {
    fn map_canvas(&self) -> impl Widget + '_ {
        Canvas::default()
            .block(Block::bordered().title("World"))
            .marker(self.marker)
            .paint(|ctx| {
                ctx.draw(&Map {
                    color: Color::Green,
                    resolution: MapResolution::High,
                });
                ctx.print(self.x, -self.y, "You are here".yellow());
            })
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0])
    }
}
