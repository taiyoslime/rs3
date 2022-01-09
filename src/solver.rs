use std::error;
use std::fmt;

pub mod dpll;

#[derive(Debug, Clone)]
pub enum SATResult {
    Sat(Vec<bool>),
    UnSat,
}

impl fmt::Display for SATResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SATResult::Sat(v) => {
                let mut buf = String::new();
                buf.push_str("s SATISFIABLE\nv ");
                buf.push_str(
                    &v.iter()
                        .enumerate()
                        .map(|(i, b)| match b {
                            true => (i + 1).to_string(),
                            false => format!("-{}", i + 1),
                        })
                        .collect::<Vec<_>>()
                        .join(" "),
                );
                write!(f, "{}", buf)
            }
            SATResult::UnSat => write!(f, "s UNSATISFIABLE"),
        }
    }
}

pub trait Solver {
    fn new() -> Self;
    fn from_cnf(&mut self, clauses: &[dimacs::Clause], num_vars: u64) -> &mut Self;
    fn solve(&mut self) -> Result<SATResult, Box<dyn error::Error>>;
}
