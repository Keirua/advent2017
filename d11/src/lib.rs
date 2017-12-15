/*
https://www.redblobgames.com/grids/hexagons/
Using Axial coordinates
*/
#[derive(Debug)]
pub struct HexPosition {

    q:i32,
    r:i32
}

impl HexPosition {
    pub fn new() -> HexPosition {
        return HexPosition{
            q:0, r:0
        }
    }

    pub fn apply_movements(&mut self, s:&str) -> i32 {
        let moves = s.split(",");
        let mut max_moves = 0;
        for movement in moves {
            match movement {
                "n" => { self.r -= 1 }
                "s" => { self.r += 1 }
                "nw" => { self.q -= 1 }
                "se" => { self.q += 1 }
                
                "ne" => { self.r -= 1; self.q += 1 }
                "sw" => { self.r += 1; self.q -= 1 }
                _ => { panic!("nope!"); }
            }
            let curr_nb_moves = self.nb_movements();
            if max_moves < curr_nb_moves {
                max_moves = curr_nb_moves;
            }
        }

        max_moves
    }

    pub fn nb_movements(&self) -> i32 {
        return (self.q.abs() + (self.q + self.r).abs() + self.r.abs()) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut p = HexPosition::new();
        p.apply_movements("ne,ne,ne");
        assert_eq!(3, p.nb_movements());

        let mut p2 = HexPosition::new();
        p2.apply_movements("ne,ne,sw,sw");
        assert_eq!(0, p2.nb_movements());
        //
        let mut p3 = HexPosition::new();
        p3.apply_movements("ne,ne,s,s");
        assert_eq!(2, p3.nb_movements());

        let mut p4 = HexPosition::new();
        p4.apply_movements("se,sw,se,sw,sw");
        assert_eq!(3, p4.nb_movements());
    }
}
