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
/*
impl Eq for Coord {

}
*/

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
