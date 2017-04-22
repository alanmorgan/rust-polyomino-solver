
use bit_vec::BitVec;

use poly::board::Board;
use poly::board::board_utils;
use poly::polyomino::Polyomino;

pub struct Solver<'a, 'b> {
    board: &'a mut Board<'a>,
    candidates: &'a Vec<Vec<Polyomino>>,
    region_check: Option<&'b Fn(&Board)->bool>

    // Add print vs. count
}

impl<'a, 'b> Solver<'a, 'b> {
    pub fn new(b: &'a mut Board<'a>, c: &'a Vec<Vec<Polyomino>>) -> Solver<'a, 'b>{
        Solver { board : b,
                 candidates: c,
                 region_check: None }
    }

    #[allow(dead_code)]
    pub fn set_region_checker(&mut self, rc: &'b Fn(&Board)->bool) {
        self.region_check = Some(rc);
    }

    pub fn solve(&mut self) -> i32 {
        let mut usable_candidates = BitVec::from_elem(self.candidates.len(), true);
        
        self.solve_ex(&mut usable_candidates)
    }

    fn solve_ex(&mut self, usable_candidates: &mut BitVec) -> i32 {
        let mut total = 0;
        
        if usable_candidates.none() {
            return 1;
        }
        
        if let Some(region_check_fn) = self.region_check {
            if !region_check_fn(self.board) {
                return 0;
            }
        }

        if let Some(fit_point) = board_utils::get_first_unoccupied(&mut self.board) {
            for i in 0..usable_candidates.len() {
                if usable_candidates.get(i) == Some(true) {
                    for poly in &self.candidates[i] {
                        if let Some(point) = board_utils::fit_at(&mut self.board, &poly, fit_point) {
                            usable_candidates.set(i, false);
                            total += self.solve_ex(usable_candidates);
                            usable_candidates.set(i, true);
                            self.board.remove_polyomino(point);
                        }
                    }
                }
            }
        } else {
            panic!("Pieces left over, but no unoccupied points");
        }
        
        total
    }
}
