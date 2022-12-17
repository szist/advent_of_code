use pathfinding::prelude::bfs;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn unwrap_match_to_usize(re_match: Option<regex::Match<'_>>) -> usize {
    re_match
        .unwrap()
        .as_str()
        .parse()
        .expect("Expected a number here")
}

#[derive(Debug, Clone)]
pub struct RawValve {
    pub name: String,
    pub flow: usize,
    pub tunnels: Vec<String>,
}

fn parse_all_valves(input: &str) -> Vec<RawValve> {
    let move_reg =
        Regex::new(r"^Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (.*)$")
            .expect("Bad regexp");
    input
        .trim_end()
        .lines()
        .map(|line| {
            let captures = move_reg.captures(line).expect("Line does not match a move");
            let name = captures.get(1).unwrap().as_str().to_string();
            let flow = unwrap_match_to_usize(captures.get(2));
            let tunnels = captures
                .get(3)
                .unwrap()
                .as_str()
                .split(", ")
                .map(|x| x.to_string())
                .collect();
            RawValve {
                name,
                flow,
                tunnels,
            }
        })
        .collect()
}

// only the non-zero valves
#[derive(Debug, PartialEq, Clone)]
pub struct Valve {
    pub name: String,
    pub flow: usize,
    pub paths: HashMap<String, usize>, // shortest path to the next non-null valve + opening the valve
}

pub fn parse_valves(input: &str) -> HashMap<String, Valve> {
    let raw_valves = parse_all_valves(input.trim());
    let mut raw_valves_by_name: HashMap<String, RawValve> = HashMap::new();
    for valve in &raw_valves {
        raw_valves_by_name.insert(valve.name.clone(), valve.clone());
    }

    let mut valves_by_name: HashMap<String, Valve> = HashMap::new();
    let non_null_valves: Vec<String> = raw_valves
        .iter()
        .filter(|x| x.flow > 0)
        .map(|x| x.name.clone())
        .collect();

    let mut sources: HashSet<String> =
        HashSet::from_iter(non_null_valves.iter().map(String::clone));
    sources.insert("AA".to_string());
    // there is probably a better way.
    // Basically finding the path from all non-null valves to all other
    // non-null valves
    for source in &sources {
        let mut paths: HashMap<String, usize> = HashMap::new();
        let source_valve = raw_valves_by_name.get(source).unwrap();
        for dest in &non_null_valves {
            if source == dest {
                continue;
            }

            let p = bfs(
                source,
                |x| {
                    raw_valves_by_name
                        .get(x)
                        .unwrap()
                        .tunnels
                        .iter()
                        .map(String::clone)
                        .collect::<Vec<String>>()
                },
                |x| x == dest,
            )
            .unwrap();
            paths.insert(dest.clone(), p.len());
        }

        valves_by_name.insert(
            source.clone(),
            Valve {
                name: source_valve.name.clone(),
                flow: source_valve.flow,
                paths,
            },
        );
    }

    valves_by_name
}

struct Node {
    pub name: String,
    pub visited: HashSet<String>,
    pub released: usize,
    pub steps: usize,
}

pub fn max_flow(valves: &HashMap<String, Valve>, max_steps: usize) -> Option<usize> {
    let mut stack: Vec<Node> = vec![Node {
        name: "AA".to_string(),
        visited: HashSet::new(),
        released: 0,
        steps: 0,
    }];
    let all_valves: Vec<String> = valves
        .values()
        .filter(|x| x.flow > 0)
        .map(|v| v.name.clone())
        .collect();

    let mut max_flow = 0;

    while !stack.is_empty() {
        let node = stack.pop()?;
        if node.released > max_flow {
            max_flow = node.released;
        }

        let valve = valves.get(&node.name)?;
        for next_valve in &all_valves {
            if next_valve == &valve.name {
                continue;
            }
            let steps = valve.paths.get(next_valve)?;
            if node.visited.contains(next_valve) || node.steps + steps > max_steps {
                continue;
            }
            let next_valve = valves.get(next_valve)?;
            let steps = node.steps + steps;
            let mut visited = node.visited.clone();
            visited.insert(next_valve.name.clone());
            stack.push(Node {
                name: next_valve.name.clone(),
                released: next_valve.flow * (max_steps - steps) + node.released,
                steps,
                visited,
            })
        }
    }

    Some(max_flow)
}
