use crossterm::event::{self, Event};
use ratatui::{Frame, text::Text};
fn main() {
    let mut terminal = ratatui::init();

    loop {
        terminal.draw(draw).expect("draw failed");
        if matches!(event::read().expect("Read Failed"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Ratatui is one of the best Pixar films made");
    frame.render_widget(text, frame.area());
}
