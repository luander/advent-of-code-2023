use num::integer::lcm;
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
    nodes: BTreeMap<String, (String, String)>,
    instructions: Vec<Instruction>,
}

impl Network {
    fn steps(&self) -> usize {
        let starting_nodes = self.nodes.iter().filter(|(key, _)| key.ends_with('A'));
        starting_nodes
            .map(|(key, (left, right))| {
                let mut steps = 0;
                let mut cur_node = key.to_owned();
                let mut cur_direction = (left.to_owned(), right.to_owned());
                while !cur_node.ends_with('Z') {
                    println!("node: {}", cur_node);
                    let direction = &self.instructions[steps % self.instructions.len()];

                    let next = match direction {
                        Instruction::Left => cur_direction.0,
                        Instruction::Right => cur_direction.1,
                    };
                    cur_direction = self.nodes.get(&next).unwrap().to_owned();
                    cur_node = next;
                    steps += 1;
                }
                steps
            })
            .reduce(lcm)
            .unwrap()
    }
}

impl From<&str> for Network {
    fn from(input: &str) -> Self {
        let split_input: Vec<&str> = input.split("\n\n").collect();

        let mut node_map = BTreeMap::new();
        split_input[1].lines().for_each(|line| {
            //GXT = (MQM, CHN)
            let re = Regex::new(r"(?<id>[A-Z0-9]+) = \((?<left>[A-Z0-9]+), (?<right>[A-Z0-9]+)\)")
                .unwrap();
            let caps = re.captures(line).unwrap();

            node_map
                .entry(caps["id"].to_owned())
                .or_insert((caps["left"].to_owned(), caps["right"].to_owned()));
        });

        Self {
            instructions: split_input[0].chars().map(Instruction::from).collect(),
            nodes: node_map,
        }
    }
}

pub fn process(input: &str) -> anyhow::Result<u32> {
    let network = Network::from(input);
    let steps = network.steps();

    Ok(steps as u32)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_works() -> anyhow::Result<()> {
        let test_input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let result = process(test_input)?;
        assert_eq!(6, result);
        Ok(())
    }
}
