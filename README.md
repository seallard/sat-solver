# SAT solver


## Davis Putnam
1. Simplification
2. Resolution
3. Recursion
4. Termination

### Simplification
- Eliminate pure literals: if a literal appears with the same polarity in all clauses, it can be removed.
- Unit propagation: if a clause contains a single literal (a unit clause), it can be removed.

### Resolution
This rule finds pairs of clauses containing complementary literals for a given variable.
The pair is combined into a new clause without the variable and the original clauses are removed.
Example: (A or B) and (not B or C) -> (A or C)


### Termination
The algorithm terminates if
- An empty clause is encountered, indicating a contradiction -> not SAT.
- All clauses are satisfied or removed -> SAT.


The algorithm proceeds from simplification to resolution when all possible simplifications have been made.
The resolutions are applied for a variable until it is eliminated and then switches back to simplification.
And so on, until either a contradiction is encountered or SAT.

## DPLL


## CDCL
