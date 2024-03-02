use sat_solver::parser::parser::parse_dimacs;
use sat_solver::models::formula::Formula;


#[test]
fn test_parse_dimacs() {
    // GIVEN a valid DIMACS file
    let file = "tests/test.dimacs";

    // WHEN the file is parsed
    let formula: Formula = parse_dimacs(file).unwrap();

    // THEN the formula is returned
    assert_eq!(formula.len(), 2);
}
