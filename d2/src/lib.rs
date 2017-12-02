use std::str::FromStr;

//let s = "6046	6349	208	276	4643	1085	1539	4986			7006	5374	252	4751	226	6757	7495	2923";

fn find_int_values(s:&str) -> Vec<i32>{
	let mut v:Vec<i32> = Vec::new();
	let sp = s.split("\t");
	for curr_sp in sp {
		if curr_sp != "" {
			let i_val = i32::from_str(curr_sp).unwrap();
			v.push(i_val);
		}
	}
	return v;
}

fn find_min_max(v:Vec<i32>) -> (i32,i32) {
	let mut mini = v[0];
	let mut maxi = v[0];

	for value in v {
		if value < mini {
			mini = value;
		}
		if value > maxi {
			maxi = value;
		}
	}

	(mini, maxi)
}

pub fn find_checksum(s:&str) ->  i32 {
	let mut checksum = 0;
	let lines = s.split("\n");
	for line in lines {
		let int_vals = find_int_values(line);
		let (mini, maxi) = find_min_max(int_vals);
		checksum += maxi - mini;
	}
	checksum
}

fn find_divisors(v:Vec<i32>) -> (i32,i32) {
	for i in 0..v.len() {
		for j in 0..v.len() {
			if i != j && v[i] != 0 && v[j] != 1 && v[j] != 0 && v[i] % v[j] == 0 {
				return (v[i], v[j]);
			}
		}
	}
	return (0,1);
}

pub fn find_checksum2(s:&str) ->  i32 {
	let mut checksum = 0;
	let lines = s.split("\n");
	for line in lines {
		let int_vals = find_int_values(line);
		let (n, d) = find_divisors(int_vals);
		checksum += n/d;
	}
	checksum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_can_find_int_values() {
        assert_eq!(vec![1,2,3], find_int_values("1	2	3"));
        assert_eq!(vec![1,2,3], find_int_values("1	2		3"));
    }
    #[test]
    fn it_can_find_min_max() {
        assert_eq!((1,3), find_min_max(vec![1,2,3]));
    }
    #[test]
    fn it_can_find_checksum() {
let spreadsheet = "5	1	9	5
7	5	3
2	4	6	8";
        assert_eq!(18, find_checksum(spreadsheet));
    }

    #[test]
    fn it_can_find_divisors() {
        assert_eq!((8,2), find_divisors(vec![5,9,2,8]));
        assert_eq!((9,3), find_divisors(vec![9,4,7,3]));
    }

    #[test]
    fn it_can_find_checksum2() {
let spreadsheet = "5	9	2	8
9	4	7	3
3	8	6	5";
        assert_eq!(9, find_checksum2(spreadsheet));
    }
}
