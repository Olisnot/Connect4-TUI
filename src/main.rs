use std::io::stdout;

use color_eyre::Result;
use crossterm::{
    ExecutableCommand,
    event::{DisableMouseCapture, EnableMouseCapture},
};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    prelude::*,
    style::{Color, Style},
    symbols::Marker,
    widgets::{
        Block, Widget,
        canvas::{Canvas, Circle, Line, Rectangle},
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

struct App {
    exit: bool,
    marker: Marker,
    color: Color,
    chip_circle: Circle,
    placements: [[Option<Circle>; 6]; 7],
}

impl App {
    fn new() -> Self {
        let grid: [[Option<Circle>; 6]; 7] = std::array::from_fn(|_| std::array::from_fn(|_| None));

        Self {
            exit: false,
            marker: Marker::HalfBlock,
            color: Color::Yellow,
            chip_circle: Circle {
                x: 0.5,
                y: 6.5,
                radius: 0.2,
                color: Color::Yellow,
            },
            placements: grid,
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
            KeyCode::Char('c') => self.color = Color::Red,
            KeyCode::Right => {
                if self.chip_circle.x < 6.5 {
                    self.chip_circle.x += 1.0;
                }
            }
            KeyCode::Left => {
                if self.chip_circle.x > 0.5 {
                    self.chip_circle.x -= 1.0;
                }
            }
            KeyCode::Enter => self.add_to_placements(),
            _ => {}
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let block = Block::bordered()
            .border_style(Style::new().fg(self.color))
            .title("Connect4");
        frame.render_widget(block.clone(), frame.area());
        let visual_ratio = 7.0 / 6.0;
        let cell_ratio = visual_ratio / 0.18;
        let center_frame = self.aspect_fit_center(block.inner(frame.area()), 7, 6, cell_ratio);
        frame.render_widget(self.c4_canvas(), center_frame);
    }

    fn c4_canvas(&self) -> impl Widget {
        const COLS: f64 = 7.0;
        const ROWS: f64 = 6.0;

        let x_margin = 9.0;
        let y_margin = 2.0;
        Canvas::default()
            .marker(self.marker)
            .x_bounds([-x_margin, COLS + x_margin])
            .y_bounds([-y_margin, ROWS + y_margin])
            .paint(move |ctx| {
                ctx.draw(&Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: COLS,
                    height: ROWS,
                    color: self.color,
                });

                for x in 0..=COLS as i32 {
                    ctx.draw(&Line {
                        x1: x as f64,
                        y1: 0.0,
                        x2: x as f64,
                        y2: 6.0,
                        color: self.color,
                    });
                }

                for y in 0..=ROWS as i32 {
                    ctx.draw(&Line {
                        x1: 0.0,
                        y1: y as f64,
                        x2: 7.0,
                        y2: y as f64,
                        color: self.color,
                    });
                }

                ctx.draw(&self.chip_circle);

                for col in self.placements.iter() {
                    for chip in col.iter().flatten() {
                        ctx.draw(chip);
                    }
                }
            })
    }

    fn aspect_fit_center(&self, inner: Rect, cols: u16, rows: u16, ratio_w_over_h: f64) -> Rect {
        if inner.width == 0 || inner.height == 0 {
            return Rect::new(inner.x, inner.y, 0, 0);
        }

        let avail_w = inner.width as f64;
        let avail_h = inner.height as f64;

        let area_ratio = avail_w / avail_h;

        // First fit ideal rect (may not be divisible by cols/rows)
        let (raw_w, raw_h) = if area_ratio > ratio_w_over_h {
            // limited by height
            let h = avail_h;
            let w = ratio_w_over_h * h;
            (w, h)
        } else {
            // limited by width
            let w = avail_w;
            let h = w / ratio_w_over_h;
            (w, h)
        };

        // Snap so each logical cell is same size in characters
        let snapped_w = (raw_w.floor() as u16 / cols) * cols;
        let snapped_h = (raw_h.floor() as u16 / rows) * rows;

        if snapped_w == 0 || snapped_h == 0 {
            // Degenerate case: just give up and return empty
            return Rect::new(inner.x, inner.y, 0, 0);
        }

        // Re-center snapped rect inside `inner`
        let x = inner.x + (inner.width.saturating_sub(snapped_w)) / 2;
        let y = inner.y + (inner.height.saturating_sub(snapped_h)) / 2;

        Rect::new(x, y, snapped_w, snapped_h)
    }

    fn add_to_placements(&mut self) {
        let selected_col = self.chip_circle.x as usize;
        for (i, chip) in self.placements[selected_col].iter_mut().enumerate() {
            if chip.is_none() {
                match i {
                    0 => self.chip_circle.y = 0.5,
                    1 => self.chip_circle.y = 1.5,
                    2 => self.chip_circle.y = 2.5,
                    3 => self.chip_circle.y = 3.5,
                    4 => self.chip_circle.y = 4.5,
                    5 => self.chip_circle.y = 5.5,
                    _ => break,
                }

                chip.replace(self.chip_circle.clone());
                match self.color {
                    Color::Yellow => self.color = Color::Red,
                    Color::Red => self.color = Color::Yellow,
                    _ => {}
                }

                self.chip_circle = Circle {
                    x: 0.5,
                    y: 6.5,
                    radius: 0.2,
                    color: self.color,
                };
                break;
            }
        }
    }
}
