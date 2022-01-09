pub mod solver;
use solver::*;

pub fn solve<T: Solver>(
    input: &str,
    mut solver: T,
) -> Result<SATResult, Box<dyn std::error::Error>> {
    let instance = dimacs::parse_dimacs(input).unwrap();
    match instance {
        dimacs::Instance::Cnf { clauses, num_vars } => solver.from_cnf(&clauses, num_vars).solve(),
        dimacs::Instance::Sat { .. } => {
            unimplemented!();
        }
    }
}
