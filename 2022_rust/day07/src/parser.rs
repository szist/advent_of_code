#[allow(unused)]
use pest::Parser;

#[derive(Parser)]
#[grammar = "cli.pest"]
pub struct CLIParser;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sanity_check_parser() {
        let test = "$ cd /\n\
$ ls\n\
dir a\n\
14848514 b.txt\n\
8504156 c.dat\n\
dir d\n\
$ cd a\n\
$ ls\n\
dir e\n\
29116 f\n\
2557 g\n\
62596 h.lst";
        let successful_parse = CLIParser::parse(Rule::cli, test);
        assert!(
            match successful_parse {
                Ok(_) => true,
                Err(_) => false,
            },
            "{:?}",
            successful_parse
        );
    }
}
