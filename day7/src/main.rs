use std::os::unix::raw::pid_t;
use std::sync::Arc;
use itertools::Itertools;
use regex::Regex;
use tokio;
use tokio::main;

#[main]
async fn main() {
    // let part1 = part_1();
    let part2 = part_2().await;

    // println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn part_1() -> i64 {
    let input = "/day7/inputs/part1.txt";

    let file_content = lib::helper::parse_file_by_path(input).unwrap();

    let equations = parse_file_content(file_content);

    let mut counter = 0;
    let results = equations.iter().filter(|eq| {
        let evaluation = eq.evaluate();

        counter += 1;
        evaluation
    }).collect::<Vec<_>>();

    results.iter().map(|eq| eq.result).sum()
}

async fn part_2() -> i64 {
    let input = "/day7/inputs/part2.txt";

    let file_content = lib::helper::parse_file_by_path(input).unwrap();

    let equations = parse_file_content(file_content);

    let counter = Arc::new(tokio::sync::Mutex::new(0)); // Shared counter
    let tasks = equations.iter().map(|eq| {
        let eq_clone = eq.clone();
        let counter_clone = Arc::clone(&counter);
        tokio::spawn(async move {
            let evaluation = eq_clone.evaluate_part2().await;

            let mut counter_guard = counter_clone.lock().await;
            *counter_guard += 1;
            println!("Evaluation no: {}", *counter_guard);

            if evaluation {
                Some(eq_clone)
            } else {
                None
            }
        })
    });

    let results = futures::future::join_all(tasks).await;

    results
        .into_iter()
        .filter_map(|res| res.ok()) // Ignore failed tasks
        .map(|eq| eq.unwrap_or(CalibrationEquations{ equations: Vec::new(), result: 0 }).result)
        .sum()
}

fn parse_file_content(file_content: String) -> Vec<CalibrationEquations> {
    let lines = file_content.split("\n").collect::<Vec<_>>();

    let mut calibration_equations: Vec<CalibrationEquations> = Vec::new();
    for line in lines {
        let parts = line.split(":").map(|s| s.to_string()).collect::<Vec<String>>();
        let result = parts[0].parse::<i64>().unwrap();
        let equations = parts[1].trim().split(" ").map(|s| s.parse::<i64>().unwrap()).collect();

        calibration_equations.push(CalibrationEquations {
            result,
            equations
        });
    }

    calibration_equations
}

#[derive(Debug, Clone)]
struct CalibrationEquations {
    result: i64,
    equations: Vec<i64>
}

impl CalibrationEquations {
    fn evaluate(&self) -> bool {
        let combinations = self.generate_combinations();

        for expression in combinations {
            let evaluation = self.evaluate_expression(expression.clone());

            if evaluation == self.result {
                return true;
            }
        }

        false
    }

    async fn evaluate_part2(&self) -> bool {
        let combinations = self.generate_combinations_extra();
        let merged_eq = self.merge_equations(combinations);
        let cal_result = self.result.clone();
        let mut tasks = Vec::new();
        
        for expression in merged_eq {
            let task_result = tokio::spawn(async move {
               Self::evaluate_expression_extra(expression) == cal_result
            });
            
            tasks.push(task_result);
        }

        let results = futures::future::join_all(tasks).await;
        results.into_iter().any(|res| res.unwrap_or(false))
    }
    
    fn merge_equations(&self, input: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();

        for expr in input {
            if expr.contains('|') {
                let mut merged = String::new();
                let mut temp = String::new();
                let mut should_merge = true;

                for (i, part) in expr.split('|').enumerate() {
                    if part.contains('*') || part.contains('+') {
                        if !temp.is_empty() {
                            merged.push_str(&temp);
                            temp.clear();
                        }

                        if i > 0 {
                            merged.push('|');
                        }
                        merged.push_str(part);
                        should_merge = false;
                    } else if should_merge {
                        temp.push_str(part);
                    } else {
                        if !temp.is_empty() {
                            merged.push_str(&temp);
                            temp.clear();
                        }

                        merged.push('|');
                        merged.push_str(part);
                    }
                }

                if !temp.is_empty() {
                    merged.push_str(&temp);
                }

                result.push(merged);
            } else {
                result.push(expr);
            }
        }

        result
    }


    fn generate_combinations_extra(&self) -> Vec<String> {
        let mut results = Vec::new();

        let combinations = self.generate_operator_combinations(self.equations.len() - 1);

        for combination in combinations {
            let mut equation = String::new();

            for (index, eq) in self.equations.iter().enumerate() {
                equation.push_str(&eq.to_string());

                if index >= combination.len() {
                    continue;
                }

                equation.push_str(&combination[index].to_string());
            }

            results.push(equation);
        }

        results

    }

    fn evaluate_expression_extra(expression: String) -> i64 {
        let mut expr = expression;

        if (!expr.contains('+') && !expr.contains('*') && expr.contains('|')) {
            return expr.split('|').map(|s| s.to_string()).join("").parse::<i64>().unwrap()
        }

        if !expr.contains('+') && !expr.contains('*') {
            return expr.parse::<i64>().unwrap();
        }

        let regex_mult_add_concat = Regex::new(r"(\d+)\s*(\+|\*|\|)\s*(\d+)").unwrap();

        while let Some(captures) = regex_mult_add_concat.captures(&expr) {
            let left = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let right = captures.get(3).unwrap().as_str().parse::<i64>().unwrap();
            let operator = captures.get(2).unwrap().as_str();

            let result = match operator {
                "+" => left + right,
                "*" => left * right,
                "|" => {
                    // Concatenate left and right as digits
                    let concatenated = format!("{}{}", left, right);
                    concatenated.parse::<i64>().unwrap()
                }
                _ => panic!("Invalid operator"),
            };

            expr = regex_mult_add_concat
                .replace(&expr, |_: &regex::Captures| result.to_string())
                .to_string();
        }

        expr.parse::<i64>().unwrap()
    }

    fn evaluate_expression(&self, expression: String) -> i64 {
        let mut expr = expression;
        let regex_mult_or_add = Regex::new(r"(\d+)\s*(\+|\*)\s*(\d+)").unwrap();

        while let Some(captures) = regex_mult_or_add.captures(&expr) {
            let left = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let right = captures.get(3).unwrap().as_str().parse::<i64>().unwrap();
            let operator = captures.get(2).unwrap().as_str();

            let result = match operator {
                "+" => left + right,
                "*" => left * right,
                _ => panic!("Invalid operator"),
            };

            expr = regex_mult_or_add
                .replace(&expr, |_: &regex::Captures| result.to_string())
                .to_string();
        }

        expr.parse::<i64>().unwrap()
    }

    fn generate_combinations(&self) -> Vec<String> {
        let mut results = Vec::new();

        let combinations = self.generate_operator_combinations(self.equations.len() - 1);

        for combination in combinations {
            let mut equation = String::new();

            for (index, eq) in self.equations.iter().enumerate() {
                equation.push_str(&eq.to_string());

                if index >= combination.len() {
                    continue;
                }

                equation.push_str(&combination[index].to_string());
            }

            results.push(equation);
        }

        results
    }

    fn generate_operator_combinations(&self, n: usize) -> Vec<Vec<char>> {
        let mut results = Vec::new();
        let operators = vec!['+', '*', '|'];

        fn backtrack(
            n: usize,
            path: &mut Vec<char>,
            operators: &Vec<char>,
            results: &mut Vec<Vec<char>>,
        ) {
            if path.len() == n {
                results.push(path.clone());
                return;
            }
            for &op in operators {
                path.push(op);
                backtrack(n, path, operators, results);
                path.pop();
            }
        }

        let mut path = Vec::new();
        backtrack(n, &mut path, &operators, &mut results);
        results
    }
}



