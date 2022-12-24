use std::{cmp::max, collections::HashMap};

mod parser {
    use crate::Grid;
    use nom::{
        bytes::complete::take,
        combinator::{all_consuming, map_res},
        multi::many1,
        Parser,
    };
    use std::str::FromStr;

    fn row_parser<'a>() -> impl Parser<&'a str, Vec<usize>, ()> {
        all_consuming(many1(map_res(take(1usize), |s| {
            Ok::<usize, ()>(usize::from_str(s).expect("Could not parse string to usize"))
        })))
    }

    pub fn parse<T>(lines: T) -> Grid
    where
        T: Iterator<Item = String>,
    {
        let mut lines = lines.peekable();
        let columns = lines.peek().unwrap().len();
        let mut grid = vec![];

        for line in lines {
            let (_, mut row) = row_parser()
                .parse(line.as_str())
                .expect("Could not parse line str to Vec<usize>");

            grid.append(&mut row);
        }

        Grid::from((grid, columns))
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn iter_all() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }
}

pub struct GridPoint {
    val: usize,
    // Highest trees seen in up, down, left, right
    highest: HashMap<Direction, usize>,
    visible: bool,
}

// impl PartialOrd for GridPoint {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(match self.val.checked_sub(other.val) {
//             Some(0) => std::cmp::Ordering::Equal,
//             Some(_) => std::cmp::Ordering::Greater,
//             None => std::cmp::Ordering::Less,
//         })
//     }
// }

pub struct Grid {
    columns: usize,
    grid: Vec<GridPoint>,
}

impl From<(Vec<usize>, usize)> for Grid {
    fn from((grid, columns): (Vec<usize>, usize)) -> Self {
        let grid = Grid {
            columns,
            grid: grid
                .iter()
                .map(|x| GridPoint {
                    val: *x,
                    highest: HashMap::new(),
                    visible: false,
                })
                .collect(),
        };

        grid
    }
}

impl Grid {
    fn valid_compare_index(&self, ind: usize, dir: &Direction) -> bool {
        ind < self.grid.len()
            && match *dir {
                Direction::Up | Direction::Down => true,
                // comparison point is back on the right side
                Direction::Left => ind % self.columns != self.columns - 1,
                // comparison point is wrapped around to the left side
                Direction::Right => ind % self.columns != 0,
            }
    }

    fn get_compare_index(&self, ind: usize, dir: &Direction) -> Option<usize> {
        // Get index of point to compare against
        let compare_ind = match dir {
            Direction::Up => ind.checked_sub(self.columns),
            Direction::Down => ind.checked_add(self.columns),
            Direction::Left => ind.checked_sub(1),
            Direction::Right => ind.checked_add(1),
        };

        compare_ind.and_then(|i| {
            if self.valid_compare_index(i, dir) {
                compare_ind
            } else {
                None
            }
        })
    }

    fn fill_highest_one(&mut self, ind: usize, dir: &Direction) {
        // Get index of point to compare against
        let compare_ind = self.get_compare_index(ind, dir);

        if let Some(compare_ind) = compare_ind {
            // Recurse to guarantee highest is properly filled in for this direction
            self.fill_highest_one(compare_ind, dir);

            // Set visible and highest for this point in the grid
            let new_highest = max(
                self.grid[compare_ind].val,
                self.grid[compare_ind].highest[dir],
            );
            self.grid[ind]
                .highest
                .entry(dir.clone())
                .or_insert(new_highest);
            self.grid[ind].visible |= self.grid[ind].val > self.grid[ind].highest[dir];
        } else {
            // If the comparison point is off the grid, this is on an edge, so set visible
            self.grid[ind].visible = true;
            self.grid[ind].highest.entry(dir.clone()).or_default();
        }
    }

    fn fill_highest(&mut self) {
        for i in 0..self.columns {
            self.fill_highest_one(i, &Direction::Down);
        }
        for i in self.grid.len() - self.columns..self.grid.len() {
            self.fill_highest_one(i, &Direction::Up);
        }
        for i in (0..self.grid.len()).step_by(self.columns) {
            self.fill_highest_one(i, &Direction::Right);
        }
        for i in (self.columns - 1..self.grid.len()).step_by(self.columns) {
            self.fill_highest_one(i, &Direction::Left);
        }
    }

    fn count_visible(&self) -> usize {
        self.grid.iter().filter(|p| p.visible).count()
    }

    fn highest_scenic_score(&self) -> usize {
        let mut best = 0;
        for ind in 0..self.grid.len() {
            println!("----- index {} -----", ind);
            // Calculate and collect distance in all directions
            let mut view_distances = vec![];
            for dir in Direction::iter_all() {
                println!("Calculating view distance for direction {:?}", dir);
                // Move compare index along and increment
                let mut moving_ind = ind;
                let mut _view_dist = 0;
                while let Some(compare_ind) = self.get_compare_index(moving_ind, &dir) {
                    _view_dist += 1;
                    if self.grid[compare_ind].val >= self.grid[ind].val {
                        break;
                    }
                    moving_ind = compare_ind;
                }
                println!("View distance for direction {:?} is {}", dir, _view_dist);
                view_distances.push(_view_dist);
            }
            best = max(best, view_distances.iter().product());
        }
        best
    }
}

pub fn run1<T>(lines: T)
where
    T: Iterator<Item = String>,
{
    let mut grid = parser::parse(lines);
    grid.fill_highest();
    println!("number of visible trees is: {}", grid.count_visible());
}

pub fn run2<T>(lines: T)
where
    T: Iterator<Item = String>,
{
    let grid = parser::parse(lines);
    println!("highest scenic score is {}", grid.highest_scenic_score());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
