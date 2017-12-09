
pub fn compute1(s: &str) -> u32 {
    let b = s.as_bytes();
    let len = b.len();

    let mut sum: u32= 0;
    for i in 0..b.len() {
        if b[i] == b[(i+1) % len] {
            sum += (b[i] - b'0') as u32;
        }
    }
    sum
}

pub fn compute2(s: &str) -> u32 {
    let b = s.as_bytes();
    let len = b.len();

    let mut sum: u32= 0;
    for i in 0..b.len() {
        if b[i] == b[(i+len/2) % len] {
            sum += (b[i] - b'0') as u32;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(3, compute1("1122"));
        assert_eq!(0, compute1("1234"));
        assert_eq!(4, compute1("1111"));
        assert_eq!(9, compute1("91212129"));
    }

    #[test]
    fn it_works_part2() {
        assert_eq!(6, compute2("1212"));
        assert_eq!(0, compute2("1221"));
        assert_eq!(4, compute2("123425"));
        assert_eq!(12, compute2("123123"));
        assert_eq!(4, compute2("12131415"));
    }
}
