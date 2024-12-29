use std::str::FromStr;

use lib::helper;

fn main() {
    let part1 = part1();
    
    println!("Part 1: {}", part1);
}

fn part1() -> usize {
    let input_path = "/day10/inputs/part1.txt";
    
    let str_content = helper::parse_file_by_path(input_path).unwrap();
    let (nodes, grid) = parse_file_content(str_content);
    let mut count = 0;
    
    for node in nodes {
        let found = node.find_all(grid.clone());
        
        println!("{:?}", node);
        println!("Found: {}", found);
        count += found;
    }
    
    count
}

fn parse_file_content(content: String) -> (Vec<Node>, Grid) {
    let grid = Grid::from_str(content.as_str()).unwrap();
    let nodes = grid.get_starting_indexes().iter().map(|index| {
        let value = grid.grid[index.0][index.1];
        Node::new(value, index.clone())
    }).collect::<Vec<Node>>();
    
    (nodes, grid)
}

#[derive(Clone, Debug)]
pub struct Node {
    value: usize,
    pos: (usize, usize)
}

impl From<(usize, usize, Grid)> for Node {
    fn from((x, y, grid): (usize, usize, Grid)) -> Self {
        let value = grid.grid[x][y];
        Node::new(value, (x, y))
    }
}

impl Node {
    pub fn new(value: usize, (x,y): (usize, usize)) -> Node {
        Self {
            value,
            pos: (x, y)
        }
    }
    
    pub fn find_all(&self, grid: Grid) -> usize {
        let count = 0;
        let nodes = self.find(grid.clone());
        
        if nodes.len() == 0 {
            println!("No nodes left in node {:?}", self.clone());
            return 0;
        }

        for node in nodes {
            let value = if node.value == 9 {
                1
            } else { 0 };
            
            println!("Node value {:?}", node);
            return node.find_all(grid) + value
        }
        
        count
    }
    pub fn find(&self, grid: Grid) -> Vec<Node> {
        let mut indexes = Vec::new();
        
        let (x, y) = (self.pos.0, self.pos.1);
        let x = &x;
        let y = &y;
        
        let height = grid.grid.len();
        let width = grid.grid[0].len();
        let grid_value = grid.grid[*x][*y];
        
        match (x, y) {
            (0, 0) => {
                let right_value = grid.grid[*x][*y + 1];
                let below_value = grid.grid[*x + 1][*y];
                
                if right_value == grid_value + 1 {
                    indexes.push((*x, y + 1));
                }
                
                if below_value == grid_value + 1 {
                    indexes.push((x + 1, *y));
                }
            }
            (0, _) => {
                let left_value = grid.grid[*x][*y - 1];
                let right_value = grid.grid[*x][*y + 1];
                let below_value = grid.grid[*x + 1][*y];
                
                if left_value == grid_value + 1 {
                    indexes.push((*x, y - 1));
                }
                
                if right_value == grid_value + 1 {
                    indexes.push((*x, y + 1));
                }
                
                if below_value == grid_value + 1 {
                    indexes.push((x + 1, *y));
                }
            }
            (_, 0) => {
                let right_value = grid.grid[*x][*y + 1];
                let below_value = grid.grid[*x + 1][*y];
                let left_value = grid.grid[*x][*y - 1];
                
                if right_value == grid_value + 1 {
                    indexes.push((*x, y + 1));
                }
                
                if below_value == grid_value + 1 {
                    indexes.push((x + 1, *y));
                }
                
                if left_value == grid_value + 1 {
                    indexes.push((*x, y - 1));
                }
            }
            (_, height) => {
                let below_value = grid.grid[*x + 1][*y];
                let left_value = grid.grid[*x][*y - 1];
                let above_value = grid.grid[*x - 1][*y];
                
                if below_value == grid_value + 1 {
                    indexes.push((x + 1, *y));
                }
                
                if left_value == grid_value + 1 {
                    indexes.push((*x, y - 1));
                }
                
                if above_value == grid_value + 1 {
                    indexes.push((x - 1, *y));
                }
            }
            (width, _) => {
                let right_value = grid.grid[*x][*y + 1];
                let left_value = grid.grid[*x][*y - 1];
                let above_value = grid.grid[*x - 1][*y];
                
                if right_value == grid_value + 1 {
                    indexes.push((*x, y + 1));
                }
                
                if left_value == grid_value + 1 {
                    indexes.push((*x, y - 1));
                }
                
                if above_value == grid_value + 1 {
                    indexes.push((x - 1, *y));
                }
            }
            (width, height) => {
                let left_value = grid.grid[*x][*y - 1];
                let above_value = grid.grid[*x - 1][*y];
                
                if left_value == grid_value + 1 {
                    indexes.push((*x, y - 1));
                }
                
                if above_value == grid_value + 1 {
                    indexes.push((x - 1, *y));
                }
            }
            (_, _) => {
                let right_value = grid.grid[*x][*y + 1];
                let below_value = grid.grid[*x + 1][*y];
                let left_value = grid.grid[*x][*y - 1];
                let above_value = grid.grid[*x - 1][*y];
                
                if right_value == grid_value + 1 {
                    indexes.push((*x, y + 1));
                }
                
                if below_value == grid_value + 1 {
                    indexes.push((x + 1, *y));
                }
                
                if left_value == grid_value + 1 {
                    indexes.push((*x, y - 1));
                }
                
                if above_value == grid_value + 1 {
                    indexes.push((x - 1, *y));
                }
            }
        }

        
        indexes.iter().map(|index| Node::new(grid.grid[index.0][index.1], index.clone())).collect()
    }
}

#[derive(Clone, Debug)]
pub struct Grid {
    grid: Vec<Vec<usize>>,
}

impl FromStr for Grid {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|line| {
                line
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();
        
        Ok(Grid { grid })
    }
}

impl Grid {
    pub fn new(grid: Vec<Vec<usize>>) -> Grid {
        Grid { grid }
    }
    
    pub fn get_starting_indexes(&self) -> Vec<(usize, usize)> {
        let default_value = 0;
        let mut indexes = Vec::new();
        
        for (index_row, row) in self.grid.iter().enumerate() {
            for (index_col, col) in row.iter().enumerate() {
                if *col == default_value {
                    indexes.push((index_row, index_col));
                }
            }
        }
        
        indexes
    }
}
