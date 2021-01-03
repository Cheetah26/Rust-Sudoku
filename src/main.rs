use crossterm::event::{read, Event, KeyCode};
use crossterm::{
    cursor,
    style::{self},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::fmt::Display;
use std::io::{stdout, Write};
use std::process::exit;

struct Data {
    x: u16,
    y: u16,
}

fn main() -> Result<()> {
    let mut data = Data { x: 2, y: 1};
    stdout().execute(terminal::Clear(terminal::ClearType::All))?;
    draw_grid();
    draw_numbers([[0; 9]; 9]);
    stdout().execute(cursor::Show);
    stdout().execute(cursor::MoveTo(data.x, data.y));
    stdout().flush();

    loop {
        match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Backspace => {}
                KeyCode::Left => {
                    if data.x > 4 {
                        data.x -= 4;
                    }
                }
                KeyCode::Enter => {}
                KeyCode::Right => {
                    if data.x < 34 {
                        data.x += 4;
                    }
                }
                KeyCode::Up => {
                    if data.y > 1 {
                        data.y -= 2;
                    }
                }
                KeyCode::Down => {
                    if data.y < 16 {
                        data.y += 2;
                    }
                }
                KeyCode::Home => {}
                KeyCode::End => {}
                KeyCode::PageUp => {}
                KeyCode::PageDown => {}
                KeyCode::Tab => {}
                KeyCode::BackTab => {}
                KeyCode::Delete => {}
                KeyCode::Insert => {}
                KeyCode::F(_) => {}
                KeyCode::Char(k) => {
                    if k.is_digit(10) {
                        plot(data.x, data.y, k);
                    }
                }
                KeyCode::Null => {}
                KeyCode::Esc => {
                    exit(0);
                }
            },
            Event::Resize(_width, _height) => {}
            _ => (),
        }

        // stdout().execute(terminal::Clear(terminal::ClearType::All))?;
        // draw_grid();
        // draw_numbers([[0; 9]; 9]);
        stdout().execute(cursor::MoveTo(data.x, data.y));
        stdout().flush();
    }
}

fn draw_grid() {
    plot(0, 0, "┏");
    for x in 1..36 {
        if x % 4 == 0 {
            plot(x, 0, if x % 12 == 0 { "┳" } else { "┯" });
        } else {
            plot(x, 0, "━");
        }
    }
    for y in 1..18 {
        if y % 2 == 0 {
            plot(0, y, if y % 3 == 0 { "┣" } else { "┠" });
        } else {
            plot(0, y, "┃");
        }
    }
    plot(0, 18, "┗");

    for x in 1..36 {
        if x % 4 == 0 {
            for y in 1..18 {
                if y % 2 == 0 {
                    plot(
                        x,
                        y,
                        if x % 3 == 0 {
                            if y % 3 == 0 {
                                "╋"
                            } else {
                                "╂"
                            }
                        } else if y % 3 == 0 {
                            "┿"
                        } else {
                            "┼"
                        },
                    );
                } else {
                    plot(x, y, if x % 3 == 0 { "┃" } else { "│" });
                }
            }
        } else {
            for y in 1..18 {
                if y % 2 == 0 {
                    plot(x, y, if y % 3 == 0 { "━" } else { "─" })
                }
            }
        }
    }
    plot(36, 0, "┓");
    for y in 1..18 {
        if y % 2 == 0 {
            plot(36, y, if y % 3 == 0 { "┫" } else { "┨" });
        } else {
            plot(36, y, "┃");
        }
    }
    for x in 1..36 {
        if x % 4 == 0 {
            plot(x, 18, if x % 12 == 0 { "┻" } else { "┷" });
        } else {
            plot(x, 18, "━");
        }
    }
    plot(36, 18, "┛");
}

fn draw_numbers(nums: [[u8; 9]; 9]) {
    for x in 1..10 {
        for y in 1..10 {
            plot(x * 4 - 2, y * 2 - 1, nums[x as usize - 1][y as usize - 1]);
        }
    }
}

fn plot<T: Display + Clone>(x: u16, y: u16, s: T) {
    stdout()
        .queue(cursor::MoveTo(x, y))
        .unwrap()
        .queue(style::Print(s))
        .unwrap();
}
