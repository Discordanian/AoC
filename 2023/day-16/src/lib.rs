use std::collections::HashSet;

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    East = 0,
    South = 1,
    West = 2,
    North = 3,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub struct Ray {
    pos: (usize, usize),
    dir: Direction,
}

impl Ray {
    fn next_ray(&self, matrix: &Vec<Vec<char>>) -> Vec<Ray> {
        let mut retval = vec![];
        let (x,y) = self.pos;
        let elem = matrix[y][x];
        let row_count = matrix.len();
        let col_count = matrix[0].len();

        let ns = self.dir == Direction::North || self.dir == Direction::South;
        println!("At {:?} moving {:?}", self.pos, self.dir);

        if elem == '.' {
            match self.dir {
                Direction::North => retval.extend(self.north()),
                Direction::East => retval.extend(self.east(col_count)),
                Direction::South => retval.extend(self.south(row_count)),
                Direction::West => retval.extend(self.west()),
            }
        }
        if elem == '-' && ns {
            retval.extend(self.west());
            retval.extend(self.east(col_count));
        }
        if elem == '|' && !ns {
            retval.extend(self.north());
            retval.extend(self.south(row_count));
        }
        if elem == '/' {
            match self.dir {
                Direction::North => retval.extend(self.east(col_count)),
                Direction::East => retval.extend(self.north()),
                Direction::South => retval.extend(self.west()),
                Direction::West => retval.extend(self.south(row_count)),
            }
        }
        if elem == '\\' {
            match self.dir {
                Direction::North => retval.extend(self.west()),
                Direction::East => retval.extend(self.south(row_count)),
                Direction::South => retval.extend(self.east(col_count)),
                Direction::West => retval.extend(self.north()),
            }
        }


        dbg!(&retval);
        retval

    }
    fn north(&self) -> Vec<Ray> {
        if self.pos.1 > 0 {
            return vec![Ray {pos: (self.pos.0, self.pos.1 - 1), dir: Direction::North}];
        } 
        vec![]
    }
    fn west(&self) -> Vec<Ray> {
        if self.pos.0 > 0 {
            return vec![Ray {pos: (self.pos.0 -1, self.pos.1), dir: Direction::West}];
        } 
        vec![]
    }
    fn east(&self,wall:usize) -> Vec<Ray> {
        if self.pos.0 < wall - 1 {
            return vec![Ray {pos: (self.pos.0 + 1, self.pos.1), dir: Direction::East}];
        } 
        vec![]
    }
    fn south(&self,wall:usize) -> Vec<Ray> {
        if self.pos.1 < wall - 1 {
            return vec![Ray {pos: (self.pos.0, self.pos.1 + 1), dir: Direction::South}];
        } 
        vec![]
    }
}

pub fn process_part1(input: &str) -> u32 {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut rays: Vec<Ray> = vec![Ray {pos: (0,0),dir: Direction::East}];
    // let mut seen: Vec<(usize, usize)> = Vec::new();
    let mut past_rays: Vec<Ray> = Vec::new();

    // past_rays.push(rays[0]);


    while !rays.is_empty() {
        println!("Working through {} rays", rays.len());
        let ray = rays.pop().unwrap();
        if past_rays.contains(&ray) {
            println!("Skipping this one.  Seen it before");
            continue;
        }
        past_rays.push(ray);
        rays.extend(ray.next_ray(&matrix));
    }

    dbg!(&past_rays);
    past_rays.iter().map(|r| (r.pos.0, r.pos.1)).collect::<HashSet<(usize, usize)>>().len() as u32
}

pub fn process_part2(input: &str) -> u32 {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    matrix[1].len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 46);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 51);
    }
}
