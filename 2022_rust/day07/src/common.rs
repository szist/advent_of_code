use std::{collections::HashMap, slice::Iter};

use pest::Parser;

use crate::parser::{CLIParser, Rule};

#[derive(PartialEq, Debug)]
pub struct Dir {
    name: String,
    children: HashMap<String, Dir>,
    files: HashMap<String, usize>,
}

impl Dir {
    pub fn empty(name: String) -> Self {
        Self {
            name,
            children: HashMap::new(),
            files: HashMap::new(),
        }
    }
}

pub fn dfs(current: &Dir, path: &str, result: &mut HashMap<String, usize>) -> usize {
    let current_path = String::from(path) + &current.name + "/";
    let subdirs_size = current
        .children
        .iter()
        // THIS IS UGLY AF, basically the results are filled in here
        .map(|(_name, subdir)| dfs(subdir, current_path.as_str(), result))
        .reduce(|accum, size| accum + size)
        .unwrap_or(0);

    let file_sizes: usize = current.files.values().sum();

    let total_size = file_sizes + subdirs_size;

    let res = result.insert(current_path, total_size);

    if let Some(s) = res {
        println!(
            "Found duplicate {} with size {} current: {}",
            current.name, s, path
        );
    }

    total_size
}

fn get_dir<'a>(parent: &'a mut Dir, path: &mut Iter<String>) -> &'a mut Dir {
    let p = path.next();
    if let Some(str) = p {
        if str == "/" {
            get_dir(parent, path)
        } else {
            get_dir(parent.children.get_mut(str).unwrap(), path)
        }
    } else {
        parent
    }
}

pub fn parse_log(input: &str) -> Dir {
    let mut root_dir = Dir::empty(String::from("/"));
    let parsed = CLIParser::parse(Rule::cli, input).unwrap();
    let cli = parsed.peek().unwrap();
    let mut cwp: Vec<String> = Vec::new();
    if let Rule::cli = cli.as_rule() {
        for expr in cli.into_inner() {
            if expr.as_str().is_empty() {
                continue;
            }
            let action = expr.into_inner().peek().unwrap();
            match action.as_rule() {
                Rule::change_dir => {
                    let chdir = action.into_inner().peek().unwrap();
                    match chdir.as_rule() {
                        Rule::root_dir => cwp.push(String::from("/")),
                        Rule::parent_dir => {
                            cwp.pop();
                        }
                        Rule::name => {
                            let new_dir = String::from(chdir.as_str());
                            let parent = get_dir(&mut root_dir, &mut cwp.iter());
                            parent
                                .children
                                .insert(new_dir.clone(), Dir::empty(new_dir.clone()));
                            cwp.push(new_dir);
                        }
                        _ => panic!("Unexpected change dir rule"),
                    }
                }
                Rule::list => {
                    let cwd = get_dir(&mut root_dir, &mut cwp.iter());
                    parse_list(action, cwd);
                }
                _ => panic!("Bad syntax"),
            }
        }
    }
    root_dir
}

fn parse_list(action: pest::iterators::Pair<Rule>, cwd: &mut Dir) {
    action.into_inner().for_each(|pair| {
        if Rule::output_line == pair.as_rule() {
            let entry = pair.into_inner().peek().unwrap();
            match entry.as_rule() {
                Rule::file => {
                    let mut size: String = String::new();
                    let mut name: String = String::new();
                    entry.into_inner().for_each(|pair| match pair.as_rule() {
                        Rule::file_size => {
                            size = pair.as_str().to_string();
                        }
                        Rule::name => {
                            name = pair.as_str().to_string();
                        }
                        _ => panic!("Unexpected file entry"),
                    });
                    if !size.is_empty() && !name.is_empty() {
                        cwd.files.insert(name, size.parse().unwrap());
                    }
                }
                Rule::directory => {
                    let name = entry.into_inner().peek().unwrap();
                    // the only rule should be the name
                    if name.as_rule() != Rule::name {
                        panic!("Unexpected dir entry");
                    }
                    let name = name.as_str();
                    if !cwd.children.contains_key(name) {
                        cwd.children
                            .insert(String::from(name), Dir::empty(String::from(name)));
                    }
                }
                _ => panic!("Unexpected list entry"),
            }
        }
    });
}
