use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

use crate::models::clause::Clause;
use crate::models::formula::Formula;
use crate::models::literal::Literal;

pub fn parse_dimacs(path: impl AsRef<Path>) -> io::Result<Formula> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut formula = Formula::new();

    for line in reader.lines() {
        let line = line?;

        if is_comment_or_header(&line) {
            continue;
        }

        let clause: Clause = parse_clause(&line);

        if !clause.is_empty() {
            formula.push(clause);
        }
    }
    Ok(formula)
}

fn is_comment_or_header(line: &str) -> bool {
    line.starts_with('c') || line.starts_with('p')
}

fn parse_clause(line: &str) -> Clause {
    line.split_whitespace()
        .filter_map(|lit| lit.parse::<Literal>().ok())
        .filter(|&lit| lit != 0)
        .collect()
}
