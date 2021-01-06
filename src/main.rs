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

impl Data {
    pub fn move_up(&mut self) {
        if self.y > 1 {
            self.y -= 2;
        }
    }
    pub fn move_down(&mut self) {
        if self.y < 16 {
            self.y += 2;
        }
    }
    pub fn move_left(&mut self) {
        if self.x > 4 {
            self.x -= 4;
        }
    }
    pub fn move_right(&mut self) {
        if self.x < 34 {
            self.x += 4;
        }
    }
    pub fn set_number(&mut self, n: u8) {
        if n == 0 {
            plot(self.x, self.y, " ");
        } else {
            let x = self.x as usize / 4;
            let y = self.y as usize / 2;
            self.puzzle[y][x] = n;
            if n == self.solution[y][x] {
                plot(self.x, self.y, n);
            } else {
                plot(self.x, self.y, n.to_string().red());
            }
        }
    }
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
    plot(55, 2, "Welcome to Sudoku");
    plot(46, 3, "Press Enter to generate a new board");
    plot(40, 5, "Use the Arrow keys or H J K L to move the cursor");
    plot(41, 6, "Special keys can be used to place numbers too:");
    plot(50, 7, "Y-P = 1-5 | N-. = 6-9 | ; = 0");
    stdout().execute(cursor::Show)?;
    stdout().execute(cursor::MoveTo(data.x, data.y))?;
    stdout().flush()?;

    loop {
        match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Backspace => {}
                KeyCode::Enter => {
                    let (puzzle, solution) = gen_puzzle();
                    data.puzzle = puzzle;
                    data.solution = solution;
                    draw_numbers(puzzle);
                }
                KeyCode::Left => data.move_left(),
                KeyCode::Right => data.move_right(),
                KeyCode::Up => data.move_up(),
                KeyCode::Down => data.move_down(),
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
                        data.set_number(k.to_string().parse::<u8>().unwrap());
                    } else {
                        match k {
                            'y' => data.set_number(1),
                            'u' => data.set_number(2),
                            'i' => data.set_number(3),
                            'o' => data.set_number(4),
                            'p' => data.set_number(5),
                            'h' => data.move_left(),
                            'j' => data.move_down(),
                            'k' => data.move_up(),
                            'l' => data.move_right(),
                            ';' => data.set_number(0),
                            'n' => data.set_number(6),
                            'm' => data.set_number(7),
                            ',' => data.set_number(8),
                            '.' => data.set_number(9),
                            _ => {}
                        }
                    }
                }
                KeyCode::Null => {}
                KeyCode::Esc => {
                    exit(0);
                }
            },
            Event::Resize(_width, _height) => {}
            Event::Mouse(_event) => {}
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