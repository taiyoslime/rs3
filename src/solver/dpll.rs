use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Lit {
    Pos,
    Neg,
    Empty,
}

#[derive(Debug, Clone)]
pub struct Clause {
    eliminated: bool,
    num_non_empty_lits: isize,
    lits: Vec<Lit>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Field {
    num_vars: usize,
    num_clause: usize,

    clauses: Vec<Clause>,
}

pub struct DPLLSolver {
    fields: Vec<Field>,
    result: Vec<Vec<Option<bool>>>,
}

impl Solver for DPLLSolver {
    fn new() -> Self {
        DPLLSolver {
            fields: vec![],
            result: vec![],
        }
    }

    fn from_cnf(&mut self, clauses: &[dimacs::Clause], num_vars: u64) -> &mut Self {
        // memo usize
        let mut field = Field {
            num_vars: num_vars as usize,
            num_clause: clauses.len(),
            clauses: vec![
                Clause {
                    eliminated: false,
                    num_non_empty_lits: 0,
                    lits: vec![Lit::Empty; num_vars as usize]
                };
                clauses.len()
            ],
        };

        for (i, clause) in clauses.iter().enumerate() {
            for lit in clause.lits().iter() {
                let sign = lit.sign();
                let var = (lit.var().to_u64() - 1) as usize;

                field.clauses[i].lits[var] = match sign {
                    dimacs::Sign::Pos => Lit::Pos,
                    dimacs::Sign::Neg => Lit::Neg,
                };
                field.clauses[i].num_non_empty_lits += 1;
            }
        }
        self.result.push(vec![None; field.num_vars]);
        self.fields.push(field);
        self
    }

    fn solve(&mut self) -> Result<SATResult, Box<dyn std::error::Error>> {
        while !self.fields.is_empty() {
            #[cfg(debug_assertions)]
            println!("{:?}", self.fields);

            let front = self.fields.len() - 1;
            let target_field = &mut self.fields[front];
            let target_result = &mut self.result[front];

            // simplify
            // TOOD: 順序, loopどこまでやるかbenchmark

            // pure literal rule

            // 縦に見てダメだったらbreak
            #[allow(clippy::needless_range_loop)]
            for i in 0..target_field.num_vars {
                let mut is_pure_lit = true;
                let mut lit_m = Lit::Empty;
                for clause in target_field.clauses.iter() {
                    if !clause.eliminated {
                        let lit = &clause.lits[i];
                        if *lit != Lit::Empty && *lit != lit_m {
                            if lit_m == Lit::Empty {
                                lit_m = *lit;
                            } else {
                                is_pure_lit = false;
                                break;
                            }
                        }
                    }
                }
                if is_pure_lit && lit_m != Lit::Empty {
                    for clause in target_field.clauses.iter_mut() {
                        if clause.lits[i] != Lit::Empty {
                            clause.eliminated = true;
                        }
                    }
                    target_result[i] = match lit_m {
                        Lit::Pos => Some(true),
                        Lit::Neg => Some(false),
                        _ => unreachable!(),
                    };
                }
            }

            // unit rule

            loop {
                let mut found_unit_lit = false;
                let mut unit_lit: Lit = Lit::Empty;
                let mut unit_lit_idx: usize = 0;

                for clause in target_field
                    .clauses
                    .iter_mut()
                    .filter(|clause| !clause.eliminated)
                {
                    if clause.num_non_empty_lits == 1 {
                        found_unit_lit = true;
                        for (i, lit) in clause.lits.iter().enumerate() {
                            match lit {
                                Lit::Pos | Lit::Neg => {
                                    unit_lit_idx = i;
                                    unit_lit = *lit;
                                    break;
                                }
                                _ => (),
                            }
                        }
                        break;
                    }
                }

                if found_unit_lit {
                    for clause in target_field
                        .clauses
                        .iter_mut()
                        .filter(|clause| !clause.eliminated)
                    {
                        match (&unit_lit, &clause.lits[unit_lit_idx]) {
                            (Lit::Pos, Lit::Pos) | (Lit::Neg, Lit::Neg) => {
                                clause.eliminated = true;
                            }
                            (Lit::Pos, Lit::Neg) | (Lit::Neg, Lit::Pos) => {
                                clause.lits[unit_lit_idx] = Lit::Empty;
                                clause.num_non_empty_lits -= 1;
                            }
                            (_, _) => (),
                        }
                    }

                    target_result[unit_lit_idx] = match unit_lit {
                        Lit::Pos => Some(true),
                        Lit::Neg => Some(false),
                        _ => unreachable!(),
                    };
                } else {
                    break;
                }
            }

            // 節集合が空ならSAT, 節集合が空節を含むならUNSAT
            let mut clauses_empty = true;
            let mut unsat = false;
            for clause in target_field.clauses.iter() {
                if !clause.eliminated {
                    clauses_empty = false;
                    let non_empty_num =
                        clause.lits.iter().filter(|&lit| *lit != Lit::Empty).count();
                    if non_empty_num == 0 {
                        unsat = true;
                        break;
                    }
                }
            }

            if clauses_empty {
                let result = target_result
                    .iter()
                    .map(|r| r.unwrap_or(true))
                    .collect::<Vec<bool>>();
                return Ok(SATResult::Sat(result));
            }

            if unsat {
                #[cfg(debug_assertions)]
                println!("{:?}", self.fields);
                self.fields.pop();
                self.result.pop();
                continue;
            }

            // splitting rule

            target_field.clauses = target_field
                .clauses
                .iter()
                .filter(|clause| !clause.eliminated)
                .cloned()
                .collect::<Vec<Clause>>();

            let mut split_lit_idx: usize = 0;
            for (i, lit) in target_field.clauses[0].lits.iter().enumerate() {
                match lit {
                    Lit::Pos | Lit::Neg => {
                        split_lit_idx = i;
                        break;
                    }
                    _ => (),
                }
            }

            let mut target_field_dup = target_field.clone();
            let mut target_result_dup = target_result.clone();

            // true
            for clause in target_field.clauses.iter_mut() {
                match clause.lits[split_lit_idx] {
                    Lit::Pos => {
                        clause.eliminated = true;
                    }
                    Lit::Neg => {
                        clause.lits[split_lit_idx] = Lit::Empty;
                        clause.num_non_empty_lits -= 1;
                    }
                    _ => (),
                }
            }

            target_result[split_lit_idx] = Some(true);

            // false
            for clause in target_field_dup.clauses.iter_mut() {
                match clause.lits[split_lit_idx] {
                    Lit::Neg => {
                        clause.eliminated = true;
                    }
                    Lit::Pos => {
                        clause.lits[split_lit_idx] = Lit::Empty;
                        clause.num_non_empty_lits -= 1;
                    }
                    _ => (),
                }
            }

            target_result_dup[split_lit_idx] = Some(false);

            self.fields.push(target_field_dup);
            self.result.push(target_result_dup);

            #[cfg(debug_assertions)]
            println!("{:?}", self.fields);
        }

        Ok(SATResult::UnSat)
    }
}
