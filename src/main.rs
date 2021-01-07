mod game;

use crossterm::event::{read, Event, KeyCode};
use crossterm::{
    cursor,
    style::{self},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::fmt::Display;
use std::io::{stdout, Write};
use std::process::exit;

fn main() -> Result<()> {
    stdout().execute(terminal::Clear(terminal::ClearType::All))?;
    draw_grid();
    let mut game = game::Game::new();
    game.draw_numbers();
    plot(55, 2, "Welcome to Sudoku");
    plot(46, 3, "Press Enter to generate a new board");
    plot(40, 5, "Use the Arrow keys or H J K L to move the cursor");
    plot(41, 6, "Special keys can be used to place numbers too:");
    plot(50, 7, "Y-P = 1-5 | N-. = 6-9 | ; = 0");

    plot(40, 9, "Toggles:"); //   Key   |  Function  |   State");
    plot(50, 10, "Q: Show Wrong Numbers");
    stdout().execute(cursor::Show)?;
    stdout().execute(cursor::MoveTo(game.x, game.y))?;
    stdout().flush()?;

    loop {
        match read()? {
            Event::Key(event) => {
                stdout().execute(cursor::Hide)?;
                match event.code {
                    KeyCode::Backspace => {}
                    KeyCode::Enter => {
                        game = game::Game::new();
                    }
                    KeyCode::Left => game.move_left(),
                    KeyCode::Right => game.move_right(),
                    KeyCode::Up => game.move_up(),
                    KeyCode::Down => game.move_down(),
                    KeyCode::Home => {}
                    KeyCode::End => {}
                    KeyCode::PageUp => {}
                    KeyCode::PageDown => {}
                    KeyCode::Tab => {
                        game.test_complete();
                    }
                    KeyCode::BackTab => game.set_number(0),
                    KeyCode::Delete => game.set_number(0),
                    KeyCode::Insert => {}
                    KeyCode::F(_) => {}
                    KeyCode::Char(k) => {
                        if k.is_digit(10) {
                            game.set_number(k.to_string().parse::<u8>().unwrap());
                        } else {
                            match k {
                                'y' => game.set_number(1),
                                'u' => game.set_number(2),
                                'i' => game.set_number(3),
                                'o' => game.set_number(4),
                                'p' => game.set_number(5),
                                'h' => game.move_left(),
                                'j' => game.move_down(),
                                'k' => game.move_up(),
                                'l' => game.move_right(),
                                ';' => game.set_number(0),
                                'n' => game.set_number(6),
                                'm' => game.set_number(7),
                                ',' => game.set_number(8),
                                '.' => game.set_number(9),
                                'a' => game.toggle_show_wrong(),
                                _ => {}
                            }
                        }
                    }
                    KeyCode::Null => {}
                    KeyCode::Esc => {
                        stdout().execute(terminal::Clear(terminal::ClearType::All))?;
                        exit(0);
                    }
                }
            }
            Event::Resize(_width, _height) => {}
            Event::Mouse(_event) => {}
        }
        stdout().execute(cursor::MoveTo(game.x, game.y)).unwrap();
        stdout().execute(cursor::Show)?;
        stdout().flush()?;
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

fn plot<T: Display + Clone>(x: u16, y: u16, s: T) {
    stdout()
        .queue(cursor::MoveTo(x, y))
        .unwrap()
        .queue(style::Print(s))
        .unwrap();
}
