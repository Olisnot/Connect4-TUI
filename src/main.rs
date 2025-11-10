use std::io::stdout;

use color_eyre::Result;
use crossterm::{
    ExecutableCommand,
    event::{DisableMouseCapture, EnableMouseCapture},
};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    style::{Color, Style},
    symbols::Marker,
    widgets::{
        Block, Padding, Widget,
        canvas::{Canvas, Line, Rectangle},
    },
};

fn main() -> Result<()> {
    color_eyre::install()?;
    stdout().execute(EnableMouseCapture)?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    stdout().execute(DisableMouseCapture)?;
    app_result
}

#[derive(Clone, Copy)]
enum Chips {
    Yellow,
    Red,
    Empty,
}

struct App {
    exit: bool,
    x: f64,
    y: f64,
    marker: Marker,
    color: Color,
    placements: [[Chips; 6]; 7],
}

impl App {
    const fn new() -> Self {
        Self {
            exit: false,
            x: 0.0,
            y: 0.0,
            marker: Marker::Dot,
            color: Color::Yellow,
            placements: [[Chips::Empty; 6]; 7],
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            match event::read()? {
                Event::Key(key) => self.handle_key_press(key),
                Event::Mouse(_) => (),
                _ => (),
            }
        }
        Ok(())
    }

    fn handle_key_press(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('c') => self.color = Color::Green,
            KeyCode::Down | KeyCode::Char('j') => self.y += 1.0,
            KeyCode::Up | KeyCode::Char('k') => self.y -= 1.0,
            KeyCode::Right | KeyCode::Char('l') => self.x += 1.0,
            KeyCode::Left | KeyCode::Char('h') => self.x -= 1.0,
            _ => {}
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self.c4_canvas(), frame.area());
    }

    fn c4_canvas(&self) -> impl Widget {
        Canvas::default()
            .block(
                Block::bordered()
                    .padding(Padding::new(110, 110, 10, 10))
                    .border_style(Style::new().fg(self.color))
                    .title("Connect4"),
            )
            .marker(self.marker)
            .x_bounds([0.0, 7.0])
            .y_bounds([0.0, 6.0])
            .paint(|ctx| {
                ctx.draw(&Rectangle {
                    x: 0.0,
                    y: 1.0,
                    width: 7.0,
                    height: 6.0,
                    color: self.color,
                });

                for x in 0..=7 {
                    ctx.draw(&Line {
                        x1: x as f64,
                        y1: 0.0,
                        x2: x as f64,
                        y2: 6.0,
                        color: self.color,
                    });
                }

                for y in 0..=6 {
                    ctx.draw(&Line {
                        x1: 0.0,
                        y1: y as f64,
                        x2: 7.0,
                        y2: y as f64,
                        color: self.color,
                    });
                }
            })
    }
}
