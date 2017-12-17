#![allow(unused_variables,unused_mut,dead_code)]
extern crate regex;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
pub enum Instruction {
    Spin(usize),
    Exchange(usize,usize),
    Partner(char,char)
}

pub fn extract_program (s:&str) -> Vec<Instruction> {
    let mut instruction:Vec<Instruction> = Vec::new();
    let re_spin = Regex::new(r"^s(\d+)$").unwrap();
    let re_exchange = Regex::new(r"^x(\d+)/(\d+)$").unwrap();
    let re_partner = Regex::new(r"^p([a-p])/([a-p])$").unwrap();

    for current_instr in s.split(",") {
        if re_spin.is_match (current_instr) {
            let caps = re_spin.captures(current_instr).unwrap();
            let x:usize = usize::from_str(caps.get(1).map_or("", |m| m.as_str())).unwrap();
            instruction.push(Instruction::Spin(x));
        }
        else if re_exchange.is_match (current_instr) {
            let caps = re_exchange.captures(current_instr).unwrap();
            let a:usize = usize::from_str(caps.get(1).map_or("", |m| m.as_str())).unwrap();
            let b:usize = usize::from_str(caps.get(2).map_or("", |m| m.as_str())).unwrap();
            instruction.push(Instruction::Exchange(a,b));
        }
        else if re_partner.is_match (current_instr) {
            let caps = re_partner.captures(current_instr).unwrap();
            let a:char = char::from_str(caps.get(1).map_or("", |m| m.as_str())).unwrap();
            let b:char = char::from_str(caps.get(2).map_or("", |m| m.as_str())).unwrap();
            instruction.push(Instruction::Partner(a,b));
        }
    }

    instruction
}

fn spin(v: &mut Vec<char>, x: usize) {
    let mut w:Vec<char> = Vec::new();
    for i in 0..x {
        w.push(v[v.len()-x+i]);
    }

    for i in 0..v.len()-x {
        w.push(v[i]);
    }
    v.clear();
    for i in 0..w.len() {
        v.push(w[i]);
    }
}

fn exchange(v: &mut Vec<char>, a: usize, b: usize) {
    let z = v[a];
    v[a] = v[b];
    v[b] = z;
}
fn partner(v: &mut Vec<char>, a: char, b: char) {
    let x = v.iter().position(|&x| x == a).unwrap();
    let y = v.iter().position(|&y| y == b).unwrap();
    exchange(v, x, y);
}

pub fn create_program(nb_programs: u8) -> Vec<char>{
    let mut v:Vec<char> = Vec::new();
    for i in 0..nb_programs {
        v.push(('a' as u8 + i) as char);
    }
    v
}

pub fn run (v: &mut Vec<char>, instructions: &Vec<Instruction>) {
    for instr in instructions {
        match *instr {
            Instruction::Spin(x) => spin(v, x),
            Instruction::Exchange(a,b) => exchange(v, a, b),
            Instruction::Partner(a,b) => partner(v, a, b),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
