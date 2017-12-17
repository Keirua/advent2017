pub fn generate_spinlock(nb_iterations: usize, max_value: i32) -> Vec<i32> {
    let mut v:Vec<i32> = Vec::new();

    let mut pos:usize = 0;
    v.push(0);
    for current_value in 1..max_value+1 {
        pos = (pos + nb_iterations) % v.len();
        v.insert(pos, current_value);
        pos += 1;
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(vec![0], generate_spinlock(3,0));
        assert_eq!(vec![0,1], generate_spinlock(3,1));
        assert_eq!(vec![0,2,1], generate_spinlock(3,2));
        assert_eq!(vec![0,2,3,1], generate_spinlock(3,3));
        assert_eq!(vec![0,2,4,3,1], generate_spinlock(3,4));
        assert_eq!(vec![0,5,2,4,3,1], generate_spinlock(3,5));
        assert_eq!(vec![0,5,2,4,3,6,1], generate_spinlock(3,6));
        assert_eq!(vec![0,5,7,2,4,3,6,1], generate_spinlock(3,7));
        assert_eq!(vec![0,5,7,2,4,3,8,6,1], generate_spinlock(3,8));
        assert_eq!(vec![0,9,5,7,2,4,3,8,6,1], generate_spinlock(3,9));
    }
}
