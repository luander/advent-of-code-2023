use regex::Regex;
use std::collections::BTreeMap;

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("invalid instruction"),
        }
    }
}

#[derive(Debug)]
struct Network {
    nodes: BTreeMap<String, Node>,
    instructions: Vec<Instruction>,
}

impl Network {
    fn steps(&self) -> u32 {
        let mut visited = "AAA";
        let mut i = 0;
        loop {
            let cur_node = self.nodes.get(visited).expect("node should exist in map");
            if cur_node.id == "ZZZ" {
                break;
            }
            let direction = &self.instructions[i % self.instructions.len()];

            match direction {
                Instruction::Left => visited = cur_node.left.as_str(),
                Instruction::Right => visited = cur_node.right.as_str(),
            }
            println!("cur: {:?}, direction {:?}, i {}", cur_node, direction, i);
            i += 1;
        }
        i as u32
    }
}

impl From<&str> for Network {
    fn from(input: &str) -> Self {
        let split_input: Vec<&str> = input.split("\n\n").collect();

        let mut node_map = BTreeMap::new();
        split_input[1].lines().for_each(|line| {
            let n = Node::from(line);
            node_map.entry(n.id.clone()).or_insert(n);
        });

        dbg!(Self {
            instructions: split_input[0]
                .chars()
                .map(|c| Instruction::from(c))
                .collect(),
            nodes: node_map,
        })
    }
}

#[derive(Debug)]
struct Node {
    id: String,
    left: String,
    right: String,
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        //GXT = (MQM, CHN)
        let re = Regex::new(r"(?<id>[A-Z]+) = \((?<left>[A-Z]+), (?<right>[A-Z]+)\)").unwrap();
        let caps = re.captures(value).unwrap();
        Self {
            id: caps["id"].to_owned(),
            left: caps["left"].to_owned(),
            right: caps["right"].to_owned(),
        }
    }
}

pub fn process(input: &str) -> anyhow::Result<u32> {
    let network = Network::from(input);
    let steps = network.steps();

    Ok(steps)
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        2
    )]
    #[case(
        "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        6
    )]
    fn part_works(#[case] test_input: &str, #[case] expected: u32) -> anyhow::Result<()> {
        let result = process(test_input)?;
        assert_eq!(expected, result);
        Ok(())
    }
}
