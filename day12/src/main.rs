use std::{collections::HashSet, str::FromStr};

fn main() {
    // let part1 = part1();
    let part2 = part2();
    
    // println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn part1() -> u32 {
    let input1_path = "/day12/inputs/part1.txt";

    let file_content = lib::helper::parse_file_by_path(input1_path).unwrap();

    let grid = Grid::from_str(file_content.as_str()).unwrap();

    let regions = grid.regions();
    
    let mut total_price = 0;
    for region in regions {
        let price = region.price();
        
        total_price += price;
    }
    
    total_price
}


fn part2() -> u32 {
    let input2_path = "/day12/inputs/part2.txt";

    let file_content = lib::helper::parse_file_by_path(input2_path).unwrap();

    let grid = Grid::from_str(file_content.as_str()).unwrap();

    let regions = grid.regions();
    
    let mut total_price = 0;
    for region in regions {
        let sides = region.sides();
        println!("A region of {:?} plants with price {:?} * {:?} = {:?}.", region.id, region.area(), sides, region.price_by_side());
        let price = region.price_by_side();
        
        total_price += price;
    }
    
    total_price
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Spot(usize, usize, char);

#[derive(Clone, Debug)]
struct Region {
    id: char,
    spots: Vec<Spot>,
}

impl Spot {
    pub fn is_same_vertical_side(&self, spot: &Spot) -> bool {
        self.2 == spot.2 && self.1 == spot.1
    }
    
    pub fn is_same_horizontal_side(&self, spot: &Spot) -> bool {
        self.2 == spot.2 && self.0 == spot.0
    }
}

#[derive(Clone, Debug)]
struct Grid {
    grid: Vec<Vec<char>>,
}

impl Region {
    pub fn new(spots: Vec<Spot>, id: char) -> Self {
        Self {
            id,
            spots
        }
    }
    
    pub fn uniques(&self) -> Vec<Spot> {
        let mut uniques = Vec::new();
        
        for spot in &self.spots {
            if !uniques.contains(spot) {
                uniques.push(spot.clone());
            }
        }
        
        uniques
    }
    
    pub fn area(&self) -> u32 {
        self.uniques().len() as u32
    }
    
    pub fn perimeter(&self) -> u32 {
        let mut perimeter = 0;
        
        for spot in &self.uniques() {
            let Spot(row, col, _) = spot;
            let neighbors = [
                (row.wrapping_sub(1), *col),
                (*row, col.wrapping_sub(1)),
                (*row + 1, *col),
                (*row, col + 1),
            ];
            
            for (n_row, n_col) in neighbors {
                if !self.uniques().iter().any(|s| s.0 == n_row && s.1 == n_col) {
                    perimeter += 1;
                }
            }
        }
        
        perimeter
    }
    
    pub fn sides(&self) -> u32 {
            let mut horizontal_sides = 0;
            let mut vertical_sides = 0;
    
            let mut horizontal_indexes = Vec::new();
            let mut vertical_indexes = Vec::new();
            
            for spot in &self.uniques() {
                let Spot(row, col, _) = *spot;
    
                if !horizontal_indexes.contains(&row) {
                    horizontal_sides += 1;
                    horizontal_indexes.push(row);
                } 
                
                if !vertical_indexes.contains(&col) {
                    vertical_sides += 1;
                    vertical_indexes.push(col);
                }
            }
    
            horizontal_sides * 2 + vertical_sides * 2
        }
    
    pub fn price_by_side(&self) -> u32 {
        self.sides() * self.area()
    }
    
    pub fn price(&self) -> u32 {
        self.area() * self.perimeter()
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s.lines().map(|line| {
            line.chars().collect()
        }).collect();

        Ok(Grid {
            grid
        })
    }
}

impl Grid {
    pub fn bfs(&self, start: Spot) -> Vec<Spot> {
        let mut queue = Vec::new();
        let mut visited = Vec::new();
        
        queue.push(start);
        
        while let Some(spot) = queue.pop() {
            visited.push(spot.clone());
            
            let neighbors = self.get_neighbors(spot.clone());
            
            for neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    queue.push(neighbor);
                }
            }
        }
        
        visited
    }
    
    pub fn get_neighbors(&self, spot: Spot) -> Vec<Spot> {
        let mut neighbors = Vec::new();
        
        let Spot(row, col, char) = spot;
        
        if row > 0 && self.grid[row - 1][col] == char {
            neighbors.push(Spot(row - 1, col, char));
        }
        
        if col > 0 && self.grid[row][col - 1] == char {
            neighbors.push(Spot(row, col - 1, char));
        }
        
        if row < self.grid.len() - 1 && self.grid[row + 1][col] == char {
            neighbors.push(Spot(row + 1, col, char));
        }
        
        if col < self.grid[0].len() - 1 && self.grid[row][col + 1] == char {
            neighbors.push(Spot(row, col + 1, char));
        }
        
        neighbors
    }
    
    
    pub fn regions(&self) -> Vec<Region> {
        let mut regions = Vec::new();
        let mut grid = self.clone();
        
        // We must have found
        while let Some(spot) = grid.get_random_available_spot() {
            let spots_visited = grid.bfs(spot.clone());
            
            let char_value = grid.grid[spot.0][spot.1];
            let region = Region::new(spots_visited.clone(), char_value);
            
            regions.push(region);
            
            for spot_visited in spots_visited.clone() {
                grid.grid[spot_visited.0][spot_visited.1] = '.';
            }
        }
        
        regions
    }
    
    pub fn get_random_available_spot(&self) -> Option<Spot> {
        for (index_row, row) in self.grid.iter().enumerate() {
            for (index_col, _col) in row.iter().enumerate() {
                let id = self.grid[index_row][index_col];
                if id != '.' {
                    let spot = Spot(index_row, index_col, id);
                    return Some(spot);
                }
            }
        }
        
        return None
    }
}
