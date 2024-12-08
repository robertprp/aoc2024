use std::collections::{ HashSet};

fn main() {
    let part1 = part_1();
    let part2 = part_2();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn part_1() -> u32 {
    let input1_path = "/day6/inputs/part1.txt";
    let file_content = lib::helper::parse_file_by_path(input1_path).unwrap();

    let lines = parse_file_content(file_content);

    let mut guard = GuardGrid::new(lines);

    while let Some(_movement) = guard.make_movement() { }

    let unique_positions = guard.unique_positions.len() as u32;

    unique_positions + 1
}

struct GuardGrid {
    guard_current_position: (u32, u32),
    grid: Vec<Vec<char>>,
    unique_positions: HashSet<(u32, u32)>
}

impl GuardGrid {
    fn new(grid: Vec<String>) -> GuardGrid {
        let char_grid = grid.iter().map(
            |row| {
                row.chars().map( |char| char ).collect()
            }
        ).collect::<Vec<Vec<char>>>();

        let mut guard_current_position = (0, 0);

        for (index_row, row) in char_grid.iter().enumerate() {
            for (index_col, _col) in row.iter().enumerate() {
                if char_grid[index_row][index_col] == '^' {
                    guard_current_position = (index_row as u32, index_col as u32);
                }
            }
        }

        GuardGrid {
            guard_current_position,
            grid: char_grid,
            unique_positions: HashSet::new()
        }
    }

    fn print_grid(&self) {
        for row in &self.grid {
            println!("{:?}", row);
        }
    }

    fn is_out_of_bounds(&self, guard_char: char) -> bool {
        match guard_char {
            '^' => self.guard_current_position.0 == 0,
            'v' => self.guard_current_position.0 == self.grid.len() as u32 - 1,
            '<' => self.guard_current_position.1 == 0,
            '>' => self.guard_current_position.1 == self.grid[0].len() as u32 - 1,
            _ => false
        }
    }

    fn make_movement(&mut self) -> Option<bool> {
        let current_position = self.guard_current_position;
        self.unique_positions.insert(self.guard_current_position);

        let guard = self.grid[current_position.0 as usize][current_position.1 as usize];

        let (guard_row, guard_col) = self.guard_current_position.clone();

        if self.is_out_of_bounds(guard) { return None; }

        match guard {
            '^' => {
                if self.grid[guard_row as usize - 1][guard_col as usize] == '#' {
                    self.grid[current_position.0 as usize][current_position.1 as usize] = '>';
                } else {
                    self.grid[guard_row as usize - 1][guard_col as usize] = '^';
                    self.grid[guard_row as usize][guard_col as usize] = '.';
                    self.guard_current_position = (guard_row - 1, guard_col);
                }
            },
            'v' => {
                if self.grid[guard_row as usize + 1][guard_col as usize] == '#' {
                    self.grid[current_position.0 as usize][current_position.1 as usize] = '<';
                } else {
                    self.grid[guard_row as usize + 1][guard_col as usize] = 'v';
                    self.grid[guard_row as usize][guard_col as usize] = '.';
                    self.guard_current_position = (guard_row + 1, guard_col);
                }
            },
            '<' => {
                if self.grid[guard_row as usize][guard_col as usize - 1] == '#' {
                    self.grid[current_position.0 as usize][current_position.1 as usize] = '^';
                } else {
                    self.grid[guard_row as usize][guard_col as usize - 1] = '<';
                    self.grid[guard_row as usize][guard_col as usize] = '.';
                    self.guard_current_position = (guard_row, guard_col - 1);
                }
            },
            '>' => {
                if self.grid[guard_row as usize][guard_col as usize + 1] == '#' {
                    self.grid[current_position.0 as usize][current_position.1 as usize] = 'v';
                } else {
                    self.grid[guard_row as usize][guard_col as usize + 1] = '>';
                    self.grid[guard_row as usize][guard_col as usize] = '.';
                    self.guard_current_position = (guard_row, guard_col + 1);
                }
            },
            _ => {
                println!("Unknown char {}", guard);
            }
        }

        Some(true)
    }
}

fn part_2() -> u32 {
    let input1_path = "/day6/inputs/part2.txt";
    let file_content = lib::helper::parse_file_by_path(input1_path).unwrap();

    let lines = parse_file_content(file_content);

    // let mut guard = Guard::new(lines);

    // while let Some(_movement) = guard.make_movement() { }

    // let unique_positions = guard.unique_positions.len() as u32;

    // unique_positions + 1
    0
}

fn parse_file_content(file_content: String) -> Vec<String> {
    file_content.split("\n")
        .map(|line| line.to_string())
        .collect()
}
