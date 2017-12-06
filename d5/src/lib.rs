use std::str::FromStr;

/// ```
/// use d5::str_to_maze;
///
/// assert_eq!(vec![0,-1,3], str_to_maze("0 -1 3", " "));
/// ```
pub fn str_to_maze(s:&str, split_char:&str) -> Vec<i32> {
	s.split(split_char)
		.map(|x| i32::from_str(x).unwrap())
		.collect::<Vec<i32>>()
}

pub fn find_nb_steps(s:&str) -> i32 {
	let mut nb_steps = 0;
	let mut maze = str_to_maze(s, "\n");

	let mut pos:i32 = 0;
	while pos >= 0 && pos < maze.len() as i32{
	    let new_pos = pos + maze[pos as usize];
	    maze[pos as usize] += 1;
	    pos = new_pos;

	    nb_steps += 1;
	}
	nb_steps
}

pub fn find_nb_steps2(s:&str) -> i32 {
	let mut nb_steps = 0;
	let mut maze = str_to_maze(s, "\n");

	let mut pos:i32 = 0;
	while pos >= 0 && pos < maze.len() as i32{
	    let new_pos = pos + maze[pos as usize];
	    if maze[pos as usize] >= 3 {
			maze[pos as usize] -= 1;
	    }
	    else {
	    	maze[pos as usize] += 1;
	    }
	    pos = new_pos;

	    nb_steps += 1;
	}
	nb_steps
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn it_can_find_nb_steps() {
        assert_eq!(5, find_nb_steps("0\n3\n0\n1\n-3"));
    }

    #[test]
    fn it_can_find_nb_steps2() {
        assert_eq!(10, find_nb_steps2("0\n3\n0\n1\n-3"));
    }
}
