use std::collections::HashMap;

fn main() {
    let part1 = part_1();
    let part2 = part_2();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn part_1() -> u32 {
    let input1_path = "/day5/inputs/part1.txt";
    let file_content = lib::helper::parse_file_by_path(input1_path).unwrap();

    let lines = parse_file_content(file_content);
    let mut ordered_list = Vec::new();

    for list in lines.1 {
        if lines.0.is_list_ordered(list.clone()) {
            ordered_list.push(list);
            continue
        }
    }

    let sum = ordered_list.iter().map(|f| get_middle_element(f.clone())).sum();

    sum
}

fn part_2() -> u32 {
    let input1_path = "/day5/inputs/part2.txt";
    let file_content = lib::helper::parse_file_by_path(input1_path).unwrap();

    let (rules, lists) = parse_file_content(file_content);
    let mut unordered = Vec::new();

    for list in lists {
        if !rules.is_list_ordered(list.clone()) {
            unordered.push(list);
            continue
        }
    }

    let fixed_lists: Vec<Vec<u32>> = unordered.iter().map(|f| rules.fix_list(f.clone())).collect();
    println!("Fixed {:?}", fixed_lists);

    let sum = fixed_lists.iter().map(|f| get_middle_element(f.clone())).sum();

    sum
}

fn parse_file_content(file_content: String) -> (PageOrderingRules, Vec<Vec<u32>>) {
    let mut rules = PageOrderingRules::new();
    let mut values = Vec::new();

    let mut iter = file_content.split("\n").into_iter();
    while let Some(text) = iter.next() {
        if (text.contains("|")) {
            let (left, right) = text.split_once("|").unwrap();
            let left = left.parse::<u32>().unwrap();
            let right = right.parse::<u32>().unwrap();

            rules.add_rule(left, right);
        } else {
            if text.contains(",") {
                let l = text.split(",").map(|v| v.parse::<u32>().unwrap()).collect();
                values.push(l)
            }
        }
    }

    (rules, values)

}

fn get_middle_element(list: Vec<u32>) -> u32 {
    let middle = list.len() / 2;
    list[middle]
}

#[derive(Debug)]
struct PageOrderingRules(pub HashMap<u32, Vec<u32>>);

impl PageOrderingRules {
    fn new() -> PageOrderingRules {
        PageOrderingRules(HashMap::new())
    }

    fn add_rule(&mut self, page: u32, rule: u32) {
        if let Some(r) = self.0.get_mut(&page) {
            r.push(rule);
        } else {
            self.0.insert(page, vec![rule]);
        }
    }

    fn is_list_ordered(&self, list: Vec<u32>) -> bool {
        for (index, key) in list.iter().enumerate() {
            let ordering_rule = self.0.get(key);

            if let Some(ordering_rule) = ordering_rule {
                let previous = &list[..index];

                if !ordering_rule.iter().all(|rule| !previous.contains(rule)) {
                    return false;
                }
            }
        }

        true
    }

    fn fix_list(&self, list: Vec<u32>) -> Vec<u32> {
        if self.is_list_ordered(list.clone()) {
            println!("List is already ordered {:?}", list);
            list
        } else {
            let mut new_list = list.clone();

            for (index, key) in list.iter().enumerate() {
                let ordering_rule = self.0.get(key);

                if let Some(ordering_rule) = ordering_rule {
                    let previous = &list[..index];

                    if !ordering_rule.iter().all(|rule| !previous.contains(rule)) {
                        for rule in previous {
                            if !ordering_rule.contains(rule) {
                                continue
                            }
                            
                            let breaking_rule_index = new_list.iter().position(|r| r == rule);
                            if breaking_rule_index.is_none() { continue; }

                            let breaking_rule_index = breaking_rule_index.unwrap();

                            // swap two indexes in list
                            new_list.swap(breaking_rule_index, index);

                            return self.fix_list(new_list.clone());
                        }
                    }
                }
            }

            new_list
        }
    }
}