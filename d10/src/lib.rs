pub fn rev(v: &mut Vec<i32>, start:usize, len:usize) {
    let l = v.len();
    for i in 0..(len/2 as usize)  {
        let a = (start + i) % l;
        let b = (start + len-1 - i) % l;

        let tmp = v[a];
        v[a] = v[b];
        v[b] = tmp;
    }
}

use std::str::FromStr;

pub fn perform_hash1(v: &mut Vec<i32>, operations: Vec<i32>) {
    let mut current_position:i32 = 0;
    let mut current_skip:i32 = 0;
    for op in operations {
        rev(v, current_position as usize, op as usize);
        println!("{} {:?}", op, v);
        current_position += op + current_skip;
        current_skip += 1;
    }
}

pub fn generate_operation_list(s:&str) -> Vec<i32> {
    let operations = s.split(",")
        .map(|x| i32::from_str(x).unwrap())
        .collect::<Vec<i32>>();
    operations
}

pub fn generate_operation_list2(s:&str) -> Vec<i32> {
    let mut operations:Vec<i32> = Vec::new();
    for c in s.chars() {
        operations.push(c as i32)
    }

    operations.push(17);
    operations.push(31);
    operations.push(73);
    operations.push(47);
    operations.push(23);

    operations
}

fn perform_hash64(v: &mut Vec<i32>, operations: Vec<i32>) {
    let mut current_position:i32 = 0;
    let mut current_skip:i32 = 0;

    for _ in 0..64 {
        let op_list = operations.clone();
        for op in op_list {
            rev(v, current_position as usize, op as usize);
            current_position += op + current_skip;
            current_skip += 1;
        }
    }
}

fn get_hash(v:&Vec<i32>) -> String{
    let mut hash:Vec<i32> = Vec::new();
    let mut s:Vec<String> = Vec::new();
    for i in 0..16 {
        let mut xored = v[i*16];
        for j in 1..16 {
            xored ^= v[i*16+j];
        }
        hash.push(xored);
        if xored <= 16 {
            s.push(format!("0{:x}", xored));
        }
        else {
            s.push(format!("{:x}", xored));   
        }
    }

    s.join("")
}


pub fn generate_hash(s:&str) -> String {
    let mut v:Vec<i32> = Vec::with_capacity(256);
    for i in 0..256 {
        v.push(i);
    }
    let operations2 = generate_operation_list2(&s);
    perform_hash64(&mut v, operations2);
    get_hash(&v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rev_v1() {
        let mut v = vec![0,1,2,3,4];
        rev(&mut v, 0, 3);
        assert_eq!(vec![2,1,0,3,4], v);
    }

    #[test]
    fn rev_v1b() {
        let mut v = vec![0,1,2,3,4];
        rev(&mut v, 3, 3);
        assert_eq!(vec![3,1,2,0,4], v);
    }

    #[test]
    fn rev_v2b() {
        let mut v = vec![0,1,2,3,4];
        rev(&mut v, 3, 4);
        assert_eq!(vec![4,3,2,1,0], v);
    }

    #[test]
    fn rev_v2() {
        let mut v = vec![2,1,0,3,4];
        rev(&mut v, 3, 4);
        assert_eq!(vec![4,3,0,1,2], v);
    }

    #[test]
    fn hash_part2() {
        assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", generate_hash(""));
        assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", generate_hash("1,2,3"));
        assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", generate_hash("1,2,4"));
    }
}
