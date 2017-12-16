pub fn rev(v: &mut Vec<i32>, start:usize, len:usize) {
    let l = v.len();
    /*for i in 0..(((len as f32+1.0)/2.0).floor() as usize)  {
        let tmp = v[(start + middle - i + l) % l];
        v[(start + middle - i + l) % l] = v[(start + middle + i + l) % l];
        v[(start + middle + i + l) % l] = tmp;
    }*/
    for i in 0..(len/2 as usize)  {
        let a = (start + i) % l;
        let b = (start + len-1 - i) % l;

        let tmp = v[a];
        v[a] = v[b];
        v[b] = tmp;
    }
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
/*
    #[test]
    fn rev_v3() {
        let mut v = vec![0,1,2,3,4];
        rev(&mut v, 0, 3);
        assert_eq!(vec![2,1,0,3,4], v);
    }

    #[test]
    fn rev_v4() {
        let mut v = vec![0,1,2,3,4];
        rev(&mut v, 0, 3);
        assert_eq!(vec![2,1,0,3,4], v);
    }

    #[test]
    fn rev_v5() {
        let mut v = vec![0,1,2,3,4];
        rev(&mut v, 0, 3);
        assert_eq!(vec![2,1,0,3,4], v);
    }*/
}
