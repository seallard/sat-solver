# SAT solver


## Davis Putnam
1. Simplification
2. Resolution
3. Termination

### Simplification
Eliminate pure literals: if a literal appears with the same polarity in all clauses, those clauses can be removed.
Unit propagation: all clauses containg a unit clause literal can be removed. The complement can be discarded from
all remaining clauses.

### Resolution
This rule finds pairs of clauses containing complementary literals for a given variable.
The pair is combined into a new clause without the variable and the original clauses are removed.
Example: (A or B) and (not B or C) -> (A or C)


### Termination
The algorithm terminates if
- An empty clause is encountered, indicating a contradiction -> not SAT.
- All clauses are satisfied or removed -> SAT.


## DPLL
An improved version of the DP algorithm using backtracking. It explores the search space more efficiently.

Backtracking works by guessing a truth value for a variable. If a contradiction is encountered, the assignment is backtracked and alternative
paths are explored. All clauses that become true under the assignment can be removed from the formula and all literals that become false are
removed from the remaing clauses.

1. Unit propagation: iteratively apply the unit clause rule.
2. Pure literal elimination: assign pure literals and remove clauses with them.
3. Stopping condition:
    - Formula is empty -> satisfiable
    - Formula contains empty clause -> unsat
4. DPLL procedure: choose an unassigned literal:
    - Assign it true and see if it leads to SAT
    - Assign it false and see if it leads to SAT
