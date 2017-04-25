
use bit_vec::BitVec;

use poly::board::Board;
use poly::board::board_utils;
use poly::polyomino::Polyomino;

#[derive(PartialEq)]
pub enum PrintSolutions {
    PrintTotalOnly,
    PrintAll,
    PrintEveryNth(u32)
}

pub struct Solver<'a, 'b> {
    board: &'a mut Board<'a>,
    candidates: &'a Vec<Vec<Polyomino>>,
    region_check: Option<&'b Fn(&Board)->bool>,
    print_solutions: PrintSolutions
}


impl<'a, 'b> Solver<'a, 'b> {
    pub fn new(b: &'a mut Board<'a>, c: &'a Vec<Vec<Polyomino>>) -> Solver<'a, 'b>{
        Solver { board : b,
                 candidates: c,
                 region_check: None,
                 print_solutions: PrintSolutions::PrintTotalOnly}
    }

    #[allow(dead_code)]
    pub fn set_region_checker(&mut self, rc: &'b Fn(&Board)->bool) {
        self.region_check = Some(rc);
    }

    #[allow(dead_code)]
    pub fn set_print_solutions(&mut self, print_solutions: PrintSolutions) {
        self.print_solutions = print_solutions;
    }


    pub fn solve(&mut self) -> u32 {
        let mut real_solver = RealSolver::new(self);
        
        real_solver.solve()
    }
}

// There's no way to make just some fields of a struct private, but no-one else needs to see the
// internal bits of the solver (not that there is much). So the actual solving will be done with
// the (not pub) RealSolver, which is built from a (pub) Solver
struct RealSolver<'a, 'b:'a, 'c:'a> {
    solutions_found: u32,
    solver: &'a mut Solver<'b, 'c>
}

impl <'a, 'b, 'c> RealSolver<'a, 'b, 'c> {
    pub fn new(s: &'a mut Solver<'b, 'c>) -> RealSolver<'a, 'b, 'c>{
        RealSolver { solutions_found: 0,
                     solver: s }
    }

    pub fn solve(&mut self) -> u32 {
        let mut usable_candidates = BitVec::from_elem(self.solver.candidates.len(), true);
        
        self.solve_ex(&mut usable_candidates);

        self.solutions_found
    }

    fn solve_ex(&mut self, usable_candidates: &mut BitVec) {
        if usable_candidates.none() {
            self.solutions_found = self.solutions_found + 1;

            match self.solver.print_solutions {
                PrintSolutions::PrintAll => println!("Solution {}\n{}", self.solutions_found, self.solver.board),
                PrintSolutions::PrintTotalOnly => (),
                PrintSolutions::PrintEveryNth(n) => if self.solutions_found % n == 0 { println!("Solution {}\n{}", self.solutions_found, self.solver.board) }
            }

            return;
        }
        
        if let Some(region_check_fn) = self.solver.region_check {
            if !region_check_fn(self.solver.board) {
                return;
            }
        }

        if let Some(fit_point) = board_utils::get_first_unoccupied(&mut self.solver.board) {
            for i in 0..usable_candidates.len() {
                if usable_candidates.get(i) == Some(true) {
                    for poly in &self.solver.candidates[i] {
                        if let Some(point) = board_utils::fit_at(&mut self.solver.board, &poly, fit_point) {
                            usable_candidates.set(i, false);
                            self.solve_ex(usable_candidates);
                            usable_candidates.set(i, true);
                            self.solver.board.remove_polyomino(point);
                        }
                    }
                }
            }
        } else {
            panic!("Pieces left over, but no unoccupied points");
        }
    }
}
