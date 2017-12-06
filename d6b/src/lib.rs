use std::cmp::PartialOrd;

pub fn locate_max<T:PartialOrd+Copy>(v:&[T]) -> (usize, T) {
	let mut pos = 0;
	let mut max_value = v[0];

	for (offset, &value) in v.iter().enumerate() {
		if value > max_value {
			pos = offset;
			max_value = v[pos];
		}
	}
	(pos, max_value)
}

pub fn advance(v1:&Vec<i32>) -> Vec<i32> {
	let mut v = v1.clone();
	let (mut pos, mut max_value) = locate_max::<i32>(&v1[..]);
	v[pos] = 0;
	while max_value > 0	{
		pos = (pos + 1) % v.len();
		v[pos] += 1;
		max_value -= 1;
	}

	return v.clone();
}

pub fn is_in_list(list:&[Vec<i32>], value:&Vec<i32>) -> Option<i32> {
	for (i, v) in list.iter().enumerate() {
		if v == value {
			return Some(i as i32);
		}
	}
	return None;
}

pub fn find_nb_iterations(v1:Vec<i32>) -> (i32, i32) {
	let mut nb_iterations = 0;
	let mut already_found:Vec<Vec<i32>> = Vec::new();
	let mut curr_step = v1.clone();
	already_found.push(v1.clone());
	loop {
		let new_step = advance(&curr_step);
		nb_iterations += 1;
		/*println!("## Step {}", curr_step.clone(), new_step);
		println!("{:?} {:?}", curr_step.clone(), new_step);
		println!("{:#?}", already_found.clone());*/
		match is_in_list(&already_found, &new_step) {
			Some(position) => { return (nb_iterations - position, nb_iterations); },
			None => {
				already_found.push(new_step.clone());
			curr_step = new_step;
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
    fn it_can_locate_max() {
        assert_eq!((2,7), locate_max(&vec![0,2,7,0]));
        assert_eq!((1,4), locate_max(&vec![2,4,1,2]));
        assert_eq!((0,3), locate_max(&vec![3,1,2,3]));
        assert_eq!((0,3.), locate_max(&vec![3.,1.,2.,3.]));
    }

    #[test]
    fn it_can_advance() {
        assert_eq!(vec![2,4,1,2], advance(&vec![0,2,7,0]));
        assert_eq!(vec![3,1,2,3], advance(&vec![2,4,1,2]));
        assert_eq!(vec![0,2,3,4], advance(&vec![3,1,2,3]));
        assert_eq!(vec![1,3,4,1], advance(&vec![0,2,3,4]));
        assert_eq!(vec![2,4,1,2], advance(&vec![1,3,4,1]));
    }

    #[test]
    fn it_can_check_if_value_is_in_list() {
        assert_eq!(Some(1), is_in_list(&vec![vec![1,2],vec![3,4]], &vec![3,4]));
        assert_eq!(None, is_in_list(&vec![vec![1,2],vec![3,4]], &vec![5,6]));
    }

    #[test]
    fn it_can_count_iterations() {
        assert_eq!((4, 5), find_nb_iterations(vec![0,2,7,0]));
    }
}
