extern crate d19;
use std::fs::File;
use std::io::prelude::*;

fn create_map(file:&str) -> Vec<Vec<char>>{
    let mut f = File::open(file).expect("file not found");

    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let lines = contents.split("\n");
    let mut map:Vec<Vec<char>> = Vec::new();
    for line in lines {
        map.push(line.chars().collect());
    }
    map
}

fn display_map(map: &Vec<Vec<char>>) {
    for line in map {
        let line_str:String =  line.into_iter().collect();
        println!("{}", line_str);
    }
}

enum Directions {
    Down, Up, Left, Right
}

impl Directions {
    fn get_possible_potions(&self, pos:&Position) -> Vec<Position>{
        let positions = match *self {
            Directions::Down => vec![Position{c:pos.c-1, l:pos.l}, Position{c:pos.c+1, l:pos.l}, Position{c:pos.c, l:pos.l+1}],
            Directions::Up => vec![Position{c:pos.c-1, l:pos.l}, Position{c:pos.c+1, l:pos.l}, Position{c:pos.c, l:pos.l-1}],
            Directions::Left => vec![Position{c:pos.c-1, l:pos.l}, Position{c:pos.c, l:pos.l-1}, Position{c:pos.c, l:pos.l+1}],
            Directions::Right => vec![Position{c:pos.c+1, l:pos.l}, Position{c:pos.c, l:pos.l-1}, Position{c:pos.c, l:pos.l+1}],
        };

        return positions;
    }
}

#[derive(Debug)]
struct Position { l: usize, c: usize }

fn is_valid (map: &Vec<Vec<char>>, pos: &Position) -> bool {
    return pos.l >= 0 && pos.c >= 0 && pos.l < map.len() && pos.c < map[0].len();
}

fn main() {
    let map = create_map("d19.txt");
    // display_map(&map);
    let start_column = map[0].iter().position(|x| *x == '|').unwrap();
    let mut pos:Position = Position { l:0, c:start_column };
    let mut direction = Directions::Down;
    let mut tracks:Vec<char> = Vec::new();
    let mut nb_steps = 0;
    while is_valid(&map, &pos) && map[pos.l][pos.c] != ' ' {
        nb_steps += 1;
        if map[pos.l][pos.c] != '|' && map[pos.l][pos.c] != '-' && map[pos.l][pos.c] != '+' {
            tracks.push(map[pos.l][pos.c]);
        }

        if map[pos.l][pos.c] == '+' {
            let positions = direction.get_possible_potions(&pos);
            let valids = positions.iter()
                                .filter(|x| is_valid(&map, &x) && map[x.l][x.c] != ' ')
                                .collect::<Vec<_>>();
            let new_position = valids[0];   // We assume the map is valid
            if new_position.l > pos.l { direction = Directions::Down; }
            else if new_position.l < pos.l { direction = Directions::Up; }
            else if new_position.c < pos.c { direction = Directions::Left; }
            else if new_position.c > pos.c { direction = Directions::Right; }
        }

        match direction {
            Directions::Down => { pos.l += 1; }
            Directions::Up => { pos.l -= 1; }
            Directions::Left => { pos.c -= 1; }
            Directions::Right => { pos.c += 1; }
        }
    }

    println!("{:?}", start_column);
    let path:String =  tracks.into_iter().collect();
    println!("{}", path);
    println!("{}", nb_steps);
}