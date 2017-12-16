extern crate regex;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
pub enum Instruction {
    Spin(i32),
    Exchange(i32,i32),
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
            let x:i32 = i32::from_str(caps.get(1).map_or("", |m| m.as_str())).unwrap();
            instruction.push(Instruction::Spin(x));
        }
        else if re_exchange.is_match (current_instr) {
            let caps = re_exchange.captures(current_instr).unwrap();
            let a:i32 = i32::from_str(caps.get(1).map_or("", |m| m.as_str())).unwrap();
            let b:i32 = i32::from_str(caps.get(2).map_or("", |m| m.as_str())).unwrap();
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
