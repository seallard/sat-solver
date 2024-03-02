use rstest::{fixture, rstest};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tempfile::tempdir;

use sat_solver::parser::parser::parse_dimacs;

#[fixture]
pub fn dimacs_file() -> PathBuf {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.dimacs");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "c This is a comment\np cnf 3 2\n1 -3 0\n2 3 -1 0").unwrap();
    file_path
}

#[rstest]
fn test_parse_dimacs(dimacs_file: PathBuf) {
    let formula = parse_dimacs(dimacs_file).unwrap();

    assert_eq!(formula.len(), 2);
    // Further assertions as needed
}