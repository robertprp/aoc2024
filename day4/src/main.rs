fn main() {
    let part1 = part_1();
    let part2 = part_2();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn part_1() -> u32 {
    let input1_path = "/day4/inputs/part1.txt";
    let file_content = lib::helper::parse_file_by_path(input1_path).unwrap();

    let lines = parse_file_content(file_content);
    let chars = lines.iter().map(|line| line.chars().collect()).collect();

    let count = count_word(&chars, "XMAS");

    count as u32
}

fn part_2() -> u32 {
    let input_path = "/day4/inputs/part2.txt";
    let file_content = lib::helper::parse_file_by_path(input_path).unwrap();

    let lines = parse_file_content(file_content);
    let chars = lines.iter().map(|line| line.chars().collect()).collect();

    let count = count_xmas(&chars);

    count as u32
}

fn parse_file_content(content: String) -> Vec<String> {
    content.split("\n")
        .map(|line| line.to_string())
        .collect()
}

fn count_word(grid: &Vec<Vec<char>>, word: &str) -> usize {
    let directions = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let rows = grid.len();
    let cols = grid[0].len();
    let word_len = word.len();
    let word_chars: Vec<char> = word.chars().collect();
    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            for &(dr, dc) in &directions {
                let mut found = true;
                for i in 0..word_len {
                    let nr = r as isize + dr * i as isize;
                    let nc = c as isize + dc * i as isize;

                    if nr < 0 || nr >= rows as isize || nc < 0 || nc >= cols as isize {
                        found = false;
                        break;
                    }

                    if grid[nr as usize][nc as usize] != word_chars[i] {
                        found = false;
                        break;
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_xmas(grid: &Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for r in 1..(rows - 1) {
        for c in 1..(cols - 1) {
            if grid[r][c] != 'A' { continue; }
            
            let top_left_char = grid[r - 1][c - 1];
            let top_right_char = grid[r - 1][c + 1];
            let bottom_left_char = grid[r + 1][c - 1];
            let bottom_right_char = grid[r + 1][c + 1];

            match (
                top_left_char,
                top_right_char,
                bottom_left_char,
                bottom_right_char
            ) {
                ('M', 'S', 'M', 'S') |
                ('M', 'M', 'S', 'S') |
                ('S', 'M', 'S', 'M') |
                ('S', 'S', 'M', 'M') => count += 1,
                _ => {}
            }
        }
    }

    count
}