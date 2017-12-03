#![allow(unused_variables, dead_code)]

use std::fmt;

#[derive(Debug,Copy)]
pub struct Coord{
    x:i32,
    y:i32
}

impl Coord {
    pub fn new(x:i32, y:i32) -> Coord {
        return Coord{
            x:x,y:y
        };
    }

    pub fn manhattan(&self, other:&Coord) -> i32 {
        return (other.x - self.x).abs() + (other.y - self.y).abs();
    }
}

impl Clone for Coord {
    fn clone(&self) -> Coord { *self }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Coord) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug,Copy)]
enum Orientation {
    East, North, West, South
}

impl Clone for Orientation {
    fn clone(&self) -> Orientation { *self }
}

impl Orientation {
    pub fn next (self) -> Orientation {
        match self {
            Orientation::East => Orientation::North,
            Orientation::North => Orientation::West,
            Orientation::West => Orientation::South,
            Orientation::South => Orientation::East,
        }
    }

    pub fn direction(self) -> Coord {
        match self {
            Orientation::East => Coord::new(1,0),
            Orientation::North => Coord::new(0,1),
            Orientation::West => Coord::new(-1,0),
            Orientation::South => Coord::new(0,-1),
        }
    }
}

pub fn generate_coords(n:usize) -> Vec<Coord> {
    let mut v: Vec<Coord> = Vec::new();
    let mut current_orientation = Orientation::East;
    v.push(Coord::new(0,0));    // we fill coordinates 0
    v.push(Coord::new(0,0));

    let mut nb_iterations = 0;
    let mut curr_max_len = 1;
    let mut curr_len = 0;

    for _ in 1..n {
        let last_pos = v[v.len()-1].clone();
        let direction = current_orientation.direction().clone();
        let new_pos = Coord::new(
                last_pos.x + direction.x,
                last_pos.y + direction.y
            );
        
        curr_len += 1;
        if curr_len == curr_max_len{
            // once we finish an arc, we start a new one in another direction
            curr_len = 0;
            current_orientation = current_orientation.next();

            // Once we have done 2 iterations with said length, we increment the length
            nb_iterations += 1;
            if nb_iterations == 2 {
                nb_iterations = 0;
                curr_max_len += 1;
            }
        }

        v.push(new_pos);
    }

    return v;
}

pub struct Grid {
    s:usize,
    grid: Vec<Vec<i32>>
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.s {
            for j in 0..self.s {
                let _ = write!(f, "{} ", self.grid[i][j]);
            }
            let _ = write!(f, "\n");
        }
        write!(f, "")
    }
}

impl Grid {
    pub fn generate_sum_grid(n:usize) -> Grid  {
        let mut g = Grid::initialize(n);
        let coords = generate_coords(n);
        g.grid[g.s/2 as usize ][g.s/2 as usize] = 1;
        for i in 1..n {
            let grid_coord = g.to_grid_coord(coords[i].clone());
            let value = g.get_grid_value(grid_coord);
            g.grid[grid_coord.x as usize ][grid_coord.y as usize] = value;
            if value > n as i32 {
                println!("{}", value);
                break;
            }
        }
        g
    }

    pub fn to_grid_coord(&self, c:Coord) -> Coord {
        Coord::new (
            c.x + (self.s/2) as i32,
            c.y + (self.s/2) as i32
        )
    }
    
    fn get_grid_value(&self, c:Coord) -> i32 {
        let mut r:i32 = 0;
        for i in -1..2 {
            for j in -1..2 {
                let xoff:i32 = c.x+i;
                let yoff:i32 = c.y+j;
                if (xoff >=0 && yoff >=0) && (xoff < self.s as i32 && yoff < self.s as i32) {
                    r += self.grid[xoff as usize][yoff as usize];
                }
            }
        }
        return r;
    }

    fn initialize(n:usize) -> Grid {
        let s = ((n as f64).sqrt() as usize) +1;
        let mut v = Vec::new();
        for _ in 0..s {
            let mut line = Vec::new();
            for _ in 0..s {
                line.push(0);
            }
            v.push(line);
        };
        Grid {
            grid: v,
            s:s
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let coords = generate_coords(25);
        assert_eq!(Coord::new(0,0), coords[1]);
        assert_eq!(Coord::new(1,0), coords[2]);
        assert_eq!(Coord::new(2,2), coords[13]);
        assert_eq!(Coord::new(-1,2), coords[16]);
        assert_eq!(Coord::new(0,-2), coords[23]);
        assert_eq!(Coord::new(-2,-2), coords[21]);
    }
}
