use crate::plot;
use crossterm::style::Colorize;

pub struct Game {
    pub x: u16,
    pub y: u16,
    puzzle: [[u8; 9]; 9],
    solution: [[u8; 9]; 9],
    show_wrong: bool,
}

impl Game {
    pub fn new() -> Self {
        let mut data = Game {
            x: 2,
            y: 1,
            puzzle: [[0; 9]; 9],
            solution: [[0; 9]; 9],
            show_wrong: true,
        };
        data.new_puzzle();
        data.draw_numbers();
        data.toggle_show_wrong();
        data
    }
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
        let x = self.x as usize / 4;
        let y = self.y as usize / 2;
        self.puzzle[y][x] = n;
        self.draw_numbers();
    }
    fn new_puzzle(&mut self) {
        let game = sudokugen::generate(3);
        game.board()
            .to_string()
            .replace("\n", "")
            .replace(" ", "")
            .chars()
            .enumerate()
            .for_each(|(i, c)| {
                if c != '.' {
                    self.puzzle[i / 9][i % 9] = c.to_string().parse::<u8>().unwrap();
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
                    self.solution[i / 9][i % 9] = c.to_string().parse::<u8>().unwrap();
                }
            });
    }
    pub fn draw_numbers(&self) {
        for (y, row) in self.puzzle.iter().enumerate() {
            for (x, num) in row.iter().enumerate() {
                let px = (x as u16 + 1) * 4 - 2;
                let py = (y as u16 + 1) * 2 - 1;
                plot(px, py, " ");
                plot(
                    px,
                    py,
                    if self.puzzle == self.solution {
                        num.to_string().green()
                    } else if num == &0 {
                        " ".to_string().white()
                    } else if self.show_wrong && self.puzzle[y][x] != self.solution[y][x] {
                        num.to_string().red()
                    } else {
                        num.to_string().white()
                    },
                )
            }
        }
    }
    pub fn toggle_show_wrong(&mut self) {
        self.show_wrong = !self.show_wrong;
        if self.show_wrong {
            // plot(69, 10, "on ".to_string().green());
            plot(80, 10, "on ".to_string().green());
        } else {
            plot(80, 10, "off".to_string().red());
        }
        self.draw_numbers();
    }
    pub fn test_complete(&mut self) {
        self.puzzle = self.solution;
        self.puzzle[self.y as usize / 2][self.x as usize / 4] = 0;
        self.draw_numbers();
    }
}
