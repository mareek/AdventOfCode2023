use std::{collections::HashMap, str::Lines};

pub struct Day8;

impl crate::day_trait::DaySolver for Day8 {
    fn day_of_month(&self) -> i32 {
        return 8;
    }

    fn solve_first_problem(&self, file_content: &str) -> Option<String> {
        let mut lines = file_content.lines();
        let instructions = lines.next()?;
        let nodes = parse_nodes(lines)?;
        let mut current_node = nodes.get("AAA")?;
        let mut step = 0;
        while current_node.name != "ZZZ" {
            for direction in instructions.chars() {
                match direction {
                    'L' => current_node = nodes.get(&current_node.left)?,
                    'R' => current_node = nodes.get(&current_node.right)?,
                    d => panic!("Unknown direction {d}"),
                }
                step += 1;
            }
        }

        return Some(format!("{step}"));
    }

    fn solve_second_problem(&self, file_content: &str) -> Option<String> {
        let mut lines = file_content.lines();
        let directions = lines.next()?;
        let nodes = parse_nodes(lines)?;

        let starting_nodes: Vec<&Node> = nodes
            .values()
            .filter(|n| n.name.ends_with('A'))
            .collect::<Vec<_>>();
        let mut repeat_count: Vec<usize> = starting_nodes.iter().map(|_| 0).collect();
        for i in 0..starting_nodes.len() {
            let mut current_node = starting_nodes[i];
            while !current_node.name.ends_with('Z') {
                for direction in directions.chars() {
                    match direction {
                        'L' => current_node = nodes.get(&current_node.left)?,
                        'R' => current_node = nodes.get(&current_node.right)?,
                        d => panic!("Unknown direction {d}"),
                    }
                }
                repeat_count[i] += 1;
            }
        }

        let mut result = directions.len();
        for truc in repeat_count.iter() {
            result*=truc;
        }

        return Some(format!("{result}"));
    }
}

fn parse_nodes(lines: Lines) -> Option<HashMap<String, Node>> {
    let mut result: HashMap<String, Node> = HashMap::new();
    for line in lines {
        match Node::parse(line) {
            Some(node) => {
                result.insert(node.name.clone(), node);
            }
            None => {}
        }
    }
    return Some(result);
}

struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn parse(line: &str) -> Option<Node> {
        //MPV = (VTD, GCD)
        if line.len() != 16 {
            return None;
        }
        let result = Node {
            name: String::from(&line[0..3]),
            left: String::from(&line[7..10]),
            right: String::from(&line[12..15]),
        };
        return Some(result);
    }
}
