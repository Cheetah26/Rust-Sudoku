use crossterm::event::{read, Event, KeyCode};
use crossterm::style::Colorize;
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
    puzzle: [[u8; 9]; 9],
    solution: [[u8; 9]; 9],
}

fn main() -> Result<()> {
    let mut data = Data {
        x: 2,
        y: 1,
        puzzle: [[0; 9]; 9],
        solution: [[0; 9]; 9],
    };
    stdout().execute(terminal::Clear(terminal::ClearType::All))?;
    draw_grid();
    draw_numbers([[0; 9]; 9]);
    plot(49, 2, "Welcome to Sudoku");
    plot(40, 3, "Press Enter to generate a new board");
    stdout().execute(cursor::Show)?;
    stdout().execute(cursor::MoveTo(data.x, data.y))?;
    stdout().flush()?;

    loop {
        match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Backspace => {}
                KeyCode::Left => {
                    if data.x > 4 {
                        data.x -= 4;
                    }
                }
                KeyCode::Enter => {
                    let (puzzle, solution) = gen_puzzle();
                    data.puzzle = puzzle;
                    data.solution = solution;
                    draw_numbers(puzzle);
                }
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
                        if k == '0' {
                            plot(data.x, data.y, " ");
                        } else {
                            let x = data.x as usize / 4;
                            let y = data.y as usize / 2;
                            data.puzzle[y][x] = k.to_string().parse::<u8>().unwrap();
                            if data.puzzle[y][x] == data.solution[y][x] {
                                plot(data.x, data.y, k);
                            } else {
                                plot(data.x, data.y, k.red());
                            }
                        }
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
        stdout().execute(cursor::MoveTo(data.x, data.y))?;
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

fn draw_numbers(nums: [[u8; 9]; 9]) {
    for x in 1..10 {
        for y in 1..10 {
            let num = nums[x as usize - 1][y as usize - 1];
            plot(
                x * 4 - 2,
                y * 2 - 1,
                if num != 0 {
                    num.to_string()
                } else {
                    " ".to_string()
                },
            );
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

fn gen_puzzle() -> ([[u8; 9]; 9], [[u8; 9]; 9]) {
    let game = sudokugen::generate(3);
    let mut puzzle = [[0; 9]; 9];
    let mut solution = [[0; 9]; 9];
    game.board()
        .to_string()
        .replace("\n", "")
        .replace(" ", "")
        .chars()
        .enumerate()
        .for_each(|(i, c)| {
            if c != '.' {
                puzzle[i / 9][i % 9] = c.to_string().parse::<u8>().unwrap();
            }
        });
    game.solution()
        .to_string()
        .replace("\n", "")
        .replace(" ", "")
        .chars()
        .enumerate()
        .for_each(|(i, c)| {
            if c != '.' {
                solution[i % 9][i / 9] = c.to_string().parse::<u8>().unwrap();
            }
        });
    (puzzle, solution)
}