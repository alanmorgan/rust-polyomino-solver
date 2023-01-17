use bit_vec::BitVec;

use board::board_utils;
use board::Board;
use point::PointT;
use polyomino::Polyomino;

pub struct Solver<'a, T:PointT> {
    board: &'a mut Board<'a, T>,
    candidates: &'a Vec<Vec<Polyomino<T>>>,
    region_check: Option<&'a dyn Fn(&Board<T>, usize) -> bool>,
    callback_each_solution: Option<&'a dyn Fn(&Board<T>)>,
    solutions_found: u32,
}

impl<'a, T:PointT> Solver<'a, T> {
    pub fn new(b: &'a mut Board<'a, T>, c: &'a Vec<Vec<Polyomino<T>>>) -> Solver<'a, T> {
        Solver {
            board: b,
            candidates: c,
            region_check: None,
            callback_each_solution: None,
            solutions_found: 0,
        }
    }

    pub fn set_region_checker(&mut self, rc: &'a dyn Fn(&Board<T>, usize) -> bool) {
        self.region_check = Some(rc);
    }

    pub fn set_callback_function(&mut self, cb: &'a dyn Fn(&Board<T>)) {
        self.callback_each_solution = Some(cb);
    }
                                 
    pub fn solve(&mut self) -> u32 {
        let mut usable_candidates = BitVec::from_elem(self.candidates.len(), true);

        self.solve_ex(&mut usable_candidates);

        self.solutions_found
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
    
