use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u32 {
    process_input("/day19/inputs/part1.txt", |design, patterns| {
        if Design::is_possible(design.clone(), patterns) {
            1
        } else {
            0
        }
    })
}

fn part2() -> usize {
    process_input("/day19/inputs/part2.txt", |design, patterns| {
        Design::count_matching(&design.0, patterns)
    })
}

fn process_input<T, F>(file_path: &str, mut process_design: F) -> T
where
    T: Default + std::ops::AddAssign,
    F: FnMut(&Design, &Vec<TowelPattern>) -> T,
{
    let data = lib::helper::parse_file_by_path(file_path).expect("Failed to parse input file");
    let (mut patterns, designs) = parse_file_content(data);

    patterns.sort_by(|a, b| b.cmp(a));

    let mut result = T::default();
    for design in designs {
        result += process_design(&design, &patterns);
    }

    result
}

fn parse_file_content(content: String) -> (Vec<TowelPattern>, Vec<Design>) {
    let mut lines = content.lines();
    let patterns = lines
        .next()
        .expect("Missing patterns line")
        .split(',')
        .map(|p| TowelPattern::new(p.trim().to_string()))
        .collect();

    let designs = lines
        .skip(1)
        .map(|line| Design::new(line.to_string()))
        .collect();

    (patterns, designs)
}

#[derive(PartialEq, Eq, Debug, Clone, PartialOrd, Ord)]
pub struct TowelPattern(String);

impl TowelPattern {
    pub fn new(pattern: String) -> Self {
        TowelPattern(pattern)
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Design(String);

impl Design {
    pub fn new(design: String) -> Self {
        Design(design)
    }

    pub fn count_matching(design: &str, patterns: &[TowelPattern]) -> usize {
        let mut memo = HashMap::new();
        Self::count_matching_memo(design, patterns, &mut memo)
    }

    fn count_matching_memo(
        design: &str,
        patterns: &[TowelPattern],
        memo: &mut HashMap<String, usize>,
    ) -> usize {
        if design.is_empty() {
            return 1;
        }
        if let Some(&count) = memo.get(design) {
            return count;
        }

        let mut count = 0;
        for pattern in patterns {
            if design.starts_with(&pattern.0) {
                count += Self::count_matching_memo(&design[pattern.size()..], patterns, memo);
            }
        }

        memo.insert(design.to_string(), count);
        count
    }

    pub fn is_possible(design: Design, patterns: &[TowelPattern]) -> bool {
        let line = design.0;
        if line.is_empty() {
            return true;
        }

        for pattern in patterns {
            if line.starts_with(&pattern.0) {
                let remaining = &line[pattern.size()..];
                if Self::is_possible(Design::new(remaining.to_string()), patterns) {
                    return true;
                }
            }
        }

        false
    }
}
