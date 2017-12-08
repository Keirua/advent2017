extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct Node<'a> {
	name:&'a str,
	childs:Vec<&'a str>
}

fn get_nodes<'a>(s:&'a str) -> (Vec<Node<'a>>, HashMap<&str, i32>) {
	let lines = s.split("\n");
    let re_arrow = Regex::new(r"^([A-Za-z]+)\((\d+)\)->(.*)$").unwrap();
	let re_no_arrow = Regex::new(r"^([A-Za-z]+)\((\d+)\)$").unwrap();
    let mut nodes:Vec<Node> = Vec::new();
    let mut weights:HashMap<&str, i32> = HashMap::new();

    for line in lines {
    	if let Some(_) = line.find("->") {
    		let caps = re_arrow.captures(line).unwrap();
            let node_name = caps.get(1).map_or("", |m| m.as_str());
    		let node_weight:i32 = i32::from_str(caps.get(2).map_or("", |m| m.as_str())).unwrap();
			let childs_str = caps.get(3).map_or("", |m| m.as_str());
			let childs = childs_str.split(",").collect::<Vec<&str>>();
            weights.insert(node_name, node_weight);
			nodes.push(Node{
				name:node_name,childs:childs
			});
    	}
        else {
            let caps = re_no_arrow.captures(line).unwrap();
            let node_name = caps.get(1).map_or("", |m| m.as_str());
            let node_weight:i32 = i32::from_str(caps.get(2).map_or("", |m| m.as_str())).unwrap();
            weights.insert(node_name, node_weight);
        }
    }
    //println!("{:?}", weights);

    (nodes, weights)
}

pub fn parse_info(info: &str) {
    let s = str::replace(info, " ", "");

    let (nodes, weights) = get_nodes(&s);

    for i in 0..nodes.len() {
        let mut nb = 0;
    	for j in 0..nodes.len() {
            if i != j && nodes[j].childs.contains(&nodes[i].name){
                nb += 1;
                break;
            }
    	}
        if nb == 0 {
            println!("root node : {:?}", nodes[i].name);
            break;
        }
    }

    for i in 0..nodes.len() {
        let mut total_weight:i32 = *weights.get(nodes[i].name).unwrap();
        //println!("{} {}", nodes[i].name, *weights.get(nodes[i].name).unwrap());
        for c in nodes[i].childs.clone() {
            total_weight += *weights.get(c).unwrap();
            //println!("\t{}", *weights.get(c).unwrap());
        }
        println!("{:?}", total_weight);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
