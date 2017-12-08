/*
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
*/

extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
#[derive(Debug)]
pub enum Modifier {
    Inc,
    Dec
}
#[derive(Debug)]
pub enum Conditionnal {
    GreaterThan,
    LowerThan,
    GreaterOrEqualThan,
    LowerOrEqualThan,
    Equal,
    NotEqual,
}

#[derive(Debug)]
pub struct Operation<'a> {
    reg_name:&'a str,
    action: Modifier,
    modification_value:i32,
    register_conditionnal: &'a str,
    conditionnal: Conditionnal,
    condition_value:i32
}

pub fn extract_operation_list(code:&str) -> (Vec<Operation>, HashMap<&str, i32>){
    let mut operations:Vec<Operation> = Vec::new();
    let mut registers:HashMap<&str, i32> = HashMap::new();
    let lines = code.split("\n");
    let re = Regex::new(r"^([A-Za-z]+) (inc|dec) (-{0,1}\d+) if ([A-Za-z]+) (>|<|>=|<=|==|!=) (-{0,1}\d+)").unwrap();

    for line in lines {
        let caps = re.captures(line).unwrap();
        let reg_name = caps.get(1).map_or("", |m| m.as_str());
        let action = match caps.get(2).map_or("", |m| m.as_str()) {
            "inc" => Modifier::Inc,
            "dec" => Modifier::Dec,
            _ => { panic!("Invalid modifier"); }
        };
        let mod_v = i32::from_str(caps.get(3).map_or("", |m| m.as_str())).unwrap();
        let register_conditionnal = caps.get(4).map_or("", |m| m.as_str());
        let conditionnal = match caps.get(5).map_or("", |m| m.as_str()) {
            ">" => Conditionnal::GreaterThan,
            "<" => Conditionnal::LowerThan,
            ">=" => Conditionnal::GreaterOrEqualThan,
            "<=" => Conditionnal::LowerOrEqualThan,
            "==" => Conditionnal::Equal,
            "!=" => Conditionnal::NotEqual,
            _ => { panic!("Invalid conditionnal"); }
        };
        let condition_value = i32::from_str(caps.get(6).map_or("", |m| m.as_str())).unwrap();

        operations.push(Operation{
            reg_name:reg_name,
            action:action,
            modification_value:mod_v,
            register_conditionnal:register_conditionnal,
            conditionnal:conditionnal,
            condition_value:condition_value
        });

        registers.insert(reg_name, 0);
        registers.insert(register_conditionnal, 0);
    }

    (operations, registers)
}

pub fn apply_operations(operations: &Vec<Operation>, registers: &mut HashMap<&str, i32>) {
    let mut highest_value = 0;
    for operation in operations {
        let register_cond_value = *registers.get(operation.register_conditionnal).unwrap();
        let condition_ok:bool;
        match operation.conditionnal {
            Conditionnal::Equal => { condition_ok = register_cond_value == operation.condition_value; },
            Conditionnal::NotEqual => { condition_ok = register_cond_value != operation.condition_value; },
            Conditionnal::GreaterThan => { condition_ok = register_cond_value > operation.condition_value; },
            Conditionnal::LowerThan => { condition_ok = register_cond_value < operation.condition_value; },
            Conditionnal::GreaterOrEqualThan => { condition_ok = register_cond_value >= operation.condition_value; },
            Conditionnal::LowerOrEqualThan => { condition_ok = register_cond_value <= operation.condition_value; },
        }
        if condition_ok {
            let current_register_value = registers.get_mut(operation.reg_name).unwrap();
            match operation.action {
                Modifier::Inc => {
                    *current_register_value += operation.modification_value;
                    if *current_register_value > highest_value {
                        highest_value = *current_register_value;
                    }
                },
                Modifier::Dec => { *current_register_value -= operation.modification_value; }
            }
        }
        
    }
    println!("Highest value ever: {}", highest_value);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
