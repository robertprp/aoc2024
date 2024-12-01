fn main() {
    let part1 = part_1();
    let part2 = part_2();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn parse_file_content (file_content: String) -> (Vec<u32>, Vec<u32>) {
    let (mut left_list, mut right_list) = (vec![], vec![]);
    let lines = file_content.split("\n").collect::<Vec<_>>();

    for line in lines {
        let left_n = line.split_once("   ").unwrap().0.parse::<u32>().unwrap();
        left_list.push(left_n);

        let right_n = line.split_once("   ").unwrap().1.parse::<u32>().unwrap();
        right_list.push(right_n);
    }

    (left_list, right_list)
}

fn part_1() -> u32 {
    let input1_path = "/day1/inputs/part1.txt";
    let file_content = lib::helper::parse_file_by_path(input1_path).unwrap();

    let (mut left_list, mut right_list) = parse_file_content(file_content);
   
    left_list.sort_unstable();
    right_list.sort_unstable();
    
    left_list.iter()
        .zip(right_list.iter())
        .map(|(v1, v2)| (*v1).abs_diff(*v2))
        .sum()
}

fn part_2 () -> u32 {
    let input1_path = "/day1/inputs/part2.txt";
    let file_content = lib::helper::parse_file_by_path(input1_path).unwrap();
    
    let (left_list, right_list) = parse_file_content(file_content);

    right_list
        .into_iter()
        .map(|value| left_list.iter().filter(|&x| x == &value).count() as u32 * value)
        .sum()
}

