fn main() {
    let part1 = part_1();
    let part2 = part_2();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn part_2() -> u32 {
    let input1_path = "/day3/inputs/part2.txt";
    let file_content = lib::helper::parse_file_by_path(input1_path).unwrap();

    let lines = parse_file_content(file_content);

    let one_liner_instruction = lines.join("\n");
    parse_instruction(one_liner_instruction)
}

pub fn parse_instruction(input: String ) -> u32 {
    let mut sum = 0;
    let mut enable_mul = true;
    let regex = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\)").unwrap();
    for captures in regex.captures_iter(&input) {
        match captures.get(0).unwrap().as_str() {
            "do()" => enable_mul = true,
            "don't()" => enable_mul = false,
            _ if enable_mul => {
                let n1_parsed = captures[1].parse::<u32>().unwrap();
                let n2_parsed = captures[2].parse::<u32>().unwrap();
                sum += n1_parsed * n2_parsed;
            }
            _ => (),
        }
    }
    sum
}

fn part_1() -> u32 {
    let input1_path = "/day3/inputs/part1.txt";
    let file_content = lib::helper::parse_file_by_path(input1_path).unwrap();

    let lines = parse_file_content(file_content);
    let mut total: u32 = 0;

    for line in lines {
        total += scan_line(line);
    }

    total
}

fn parse_file_content(content: String) -> Vec<String> {
    content.split("\n")
        .map(|line| line.to_string())
        .collect()
}

fn scan_line(line: String) -> u32 {
    let regex = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut total: u32 = 0;

    for cap in regex.captures_iter(&line) {
        let x = cap[1].parse::<u32>().unwrap();
        let y = cap[2].parse::<u32>().unwrap();

        total += x * y;
    }

    total
}


