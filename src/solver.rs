use bit_vec::BitVec;

use crate::board::board_utils;
use crate::board::Board;
use crate::polyomino::Polyomino;

type RegionCheckFn<T> = dyn Fn(&Board<T>, usize) -> bool;
type SolutionCallbackFn<T> = dyn Fn(&Board<T>);

pub struct Solver<'a, P:Polyomino> {
    board: &'a mut Board<'a, P>,
    candidates: &'a Vec<Vec<P>>,
    region_check: Option<&'a RegionCheckFn<P>>,
    callback_each_solution: Option<&'a SolutionCallbackFn<P>>,
    solutions: Vec<Board<'a, P>>,
    enumerate_solutions: bool,
    num_solutions: u32,
}

impl<'a, P:Polyomino> Solver<'a, P> {
    pub fn new(b: &'a mut Board<'a, P>, c: &'a Vec<Vec<P>>) -> Solver<'a, P> {
        Solver {
            board: b,
            candidates: c,
            region_check: None,
            callback_each_solution: None,
            solutions: Vec::new(),
            enumerate_solutions: false,
            num_solutions: 0,
        }
    }

    pub fn set_region_checker(&mut self, rc: &'a RegionCheckFn<P>) {
        self.region_check = Some(rc);
    }

    pub fn set_callback_function(&mut self, cb: &'a SolutionCallbackFn<P>) {
        self.callback_each_solution = Some(cb);
    }
                                 
    pub fn count_solutions(&'a mut self) -> u32 {
        let mut usable_candidates = BitVec::from_elem(self.candidates.len(), true);
        
        self.solve_ex(&mut usable_candidates);
        
        self.num_solutions
    }
    
    pub fn solve(&'a mut self) -> &'a Vec<Board<'a, P>> {
        let mut usable_candidates = BitVec::from_elem(self.candidates.len(), true);

        self.enumerate_solutions = true;
        
        self.solve_ex(&mut usable_candidates);

        &self.solutions
    }

    fn solve_ex(&mut self, usable_candidates: &mut BitVec) {
        if usable_candidates.none() {
            self.num_solutions += 1;

            if self.enumerate_solutions {
                self.solutions.push(self.board.clone());
            }
                
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
    









