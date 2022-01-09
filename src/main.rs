use rs3::solver::Solver;
use rs3::*;
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: $ ./bin example.cnf");
        std::process::exit(1);
    }
    let filename = &args[1];
    let input = fs::read_to_string(filename)?;
    let solution = solve(&input, solver::dpll::DPLLSolver::new())?;
    println!("{}", solution);
    Ok(())
}
