
use bit_vec::BitVec;

use board::Board;
use board::board_utils;
use polyomino::Polyomino;

#[derive(PartialEq)]
pub enum PrintSolutions {
    PrintTotalOnly,
    PrintAll,
    PrintEveryNth(u32)
}

pub struct Solver<'a, 'b> {
    board: &'a mut Board<'a>,
    candidates: &'a Vec<Vec<Polyomino>>,
    region_check: Option<&'b Fn(&Board, usize)->bool>,
    print_solutions: PrintSolutions,
    solutions_found: u32
}


impl<'a, 'b> Solver<'a, 'b> {
    pub fn new(b: &'a mut Board<'a>, c: &'a Vec<Vec<Polyomino>>) -> Solver<'a, 'b>{
        Solver { board : b,
                 candidates: c,
                 region_check: None,
                 print_solutions: PrintSolutions::PrintTotalOnly,
                 solutions_found: 0}
    }

    #[allow(dead_code)]
    pub fn set_region_checker(&mut self, rc: &'b Fn(&Board, usize)->bool) {
        self.region_check = Some(rc);
    }

    #[allow(dead_code)]
    pub fn set_print_solutions(&mut self, print_solutions: PrintSolutions) {
        self.print_solutions = print_solutions;
    }


    pub fn solve(&mut self) -> u32 {
        let mut usable_candidates = BitVec::from_elem(self.candidates.len(), true);
        
        self.solve_ex(&mut usable_candidates);

        self.solutions_found
    }

    fn solve_ex(&mut self, usable_candidates: &mut BitVec) {
        if usable_candidates.none() {
            self.solutions_found = self.solutions_found + 1;

            match self.print_solutions {
                PrintSolutions::PrintAll => println!("Solution {}\n{}", self.solutions_found, self.board),
                PrintSolutions::PrintTotalOnly => (),
                PrintSolutions::PrintEveryNth(n) => if self.solutions_found % n == 0 { println!("Solution {}\n{}", self.solutions_found, self.board) }
            }

            return;
        }
        
        if let Some(fit_point) = board_utils::get_first_unoccupied(&mut self.board) {
            if let Some(region_check_fn) = self.region_check {
                if !region_check_fn(self.board, board_utils::get_all_adjacent(fit_point, &self.board).len()) {
                    return;
                }
            }

            for i in 0..usable_candidates.len() {
                if usable_candidates.get(i) == Some(true) {
                    for poly in &self.candidates[i] {
                        if let Some(point) = board_utils::fit_at(&mut self.board, &poly, fit_point) {
                            usable_candidates.set(i, false);
                            self.solve_ex(usable_candidates);
                            usable_candidates.set(i, true);
                            self.board.remove_polyomino(point);
                        }
                    }
                }
            }
        } else {
            panic!("Pieces left over, but no unoccupied points");
        }
    }
}
