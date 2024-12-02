fn main() {
    let part1 = part1();
    let part2 = part2();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn part1() -> u32 {
    let input1_path = "/day2/inputs/part1.txt";

    let file_content = lib::helper::parse_file_by_path(input1_path).unwrap();

    let lines = parse_file_content(file_content);
    let mut count_safe: u32 = 0;

    for line in lines {
        if is_safe(line.clone()) {
            count_safe += 1;
        }
    }

    count_safe
}

fn parse_file_content(content: String) -> Vec<Vec<u32>> {
    content.split("\n")
        .map(|line| line.split_whitespace()
            .map(|n| n.parse::<u32>().unwrap()).collect())
        .collect()
}

fn is_safe(list: Vec<u32>) -> bool {
    let all_increasing = list.windows(2).all(|w| w[0] <= w[1]);
    let all_decreasing = list.windows(2).all(|w| w[0] >= w[1]);

    if all_increasing {
        list.windows(2).all(|w| {
            let diff = w[1] - w[0];
            diff >= 1 && diff <= 3
        })
    } else if all_decreasing {
        list.windows(2).all(|w| {
            let diff = w[0] - w[1];
            diff >= 1 && diff <= 3
        })
    } else {
        false
    }
}

fn part2() -> u32 {
    let input1_path = "/day2/inputs/part2.txt";

    let file_content = lib::helper::parse_file_by_path(input1_path).unwrap();

    let lines = parse_file_content(file_content);
    let mut count_safe: u32 = 0;

    for line in lines {
        if is_safe(line.clone()) {
            count_safe += 1;
        } else if is_safe_by_remove_one_element(line.clone()) {
            count_safe += 1;
        }
    }

    count_safe
}

fn is_safe_by_remove_one_element(list: Vec<u32>) -> bool {
    for i in 0..list.len() {
        let mut modified_list = list.clone();
        modified_list.remove(i);
        if is_safe(modified_list.clone()) {
            return true;
        }
    }

    false
}