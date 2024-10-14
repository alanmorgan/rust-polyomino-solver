use bit_vec::BitVec;

use crate::board::board_utils;
use crate::board::Board;
use crate::point::Pt;
use crate::polyomino::Polyomino;
use crate::polyomino::TagTrait;

type RegionCheckFn<S, T> = dyn Fn(&Board<S, T>, usize) -> bool;
type SolutionCallbackFn<S, T> = dyn Fn(&Board<S, T>);

pub struct Solver<'a, S:TagTrait, T:Pt> {
    board: &'a mut Board<'a, S, T>,
    candidates: &'a Vec<Vec<Polyomino<S, T>>>,
    region_check: Option<&'a RegionCheckFn<S, T>>,
    callback_each_solution: Option<&'a SolutionCallbackFn<S, T>>,
    solutions: Vec<Board<'a, S, T>>,
    return_all_solutions: bool,
    solutions_found: u32,
}

pub enum SolverResult<'a, S:TagTrait, T:Pt> {
    Count(u32),
    Solutions(&'a Vec<Board<'a, S, T>>),
}

impl<'a, S:TagTrait, T:Pt> Solver<'a, S, T> {
    pub fn new(b: &'a mut Board<'a, S, T>, c: &'a Vec<Vec<Polyomino<S, T>>>) -> Solver<'a, S, T> {
        Solver {
            board: b,
            candidates: c,
            region_check: None,
            callback_each_solution: None,
            solutions: Vec::new(),
            return_all_solutions: false,
            solutions_found: 0,
        }
    }

    pub fn set_return_all_solutions(&mut self, all_solutions: bool) {
        self.return_all_solutions = all_solutions;
    }

    pub fn set_region_checker(&mut self, rc: &'a RegionCheckFn<S, T>) {
        self.region_check = Some(rc);
    }

    pub fn set_callback_function(&mut self, cb: &'a SolutionCallbackFn<S, T>) {
        self.callback_each_solution = Some(cb);
    }
                                 
    pub fn solve(&'a mut self) -> SolverResult<'a, S, T> {
        let mut usable_candidates = BitVec::from_elem(self.candidates.len(), true);

        self.solve_ex(&mut usable_candidates);

        if self.return_all_solutions {
            SolverResult::Solutions(&self.solutions)
        }
        else {
            SolverResult::Count(self.solutions_found)
        }
    }

    fn solve_ex(&mut self, usable_candidates: &mut BitVec) {
        if usable_candidates.none() {
            self.solutions_found += 1;

            if let Some(cb) = self.callback_each_solution {
                cb(self.board)
            }
            
            return;
        }

        if let Some(fit_point) = board_utils::get_first_unoccupied(self.board) {
            if let Some(region_check_fn) = self.region_check {
                if !region_check_fn(
                    self.board,
                    board_utils::get_all_adjacent(fit_point, self.board).len(),
                ) {
                    return;
                }
            }

            for i in 0..usable_candidates.len() {
                if usable_candidates.get(i) == Some(true) {
                    for poly in &self.candidates[i] {
                        if board_utils::fit_at(self.board, poly, &fit_point) {
                            usable_candidates.set(i, false);
                            self.solve_ex(usable_candidates);
                            usable_candidates.set(i, true);
                            self.board.remove_polyomino(&fit_point);
                        }
                    }
                }
            }
        } else {
            panic!("Pieces left over, but no unoccupied points");
        }
    }
}
    
