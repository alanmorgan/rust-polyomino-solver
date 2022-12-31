extern crate bit_vec;
extern crate polyomino;

use std::time::Instant;

use polyomino::board::Board;
use polyomino::point::Point;
use polyomino::point::PointT;
use polyomino::utils;
use polyomino::utils::Restrictions;
use polyomino::solver::PrintSolutions;
use polyomino::solver::Solver;

fn main() {
    let polyomino_name = "data/pentomino.poly";

    if let Ok(polyominoes) = utils::read_polyomino_file(polyomino_name, &Point::new) {
        let all_polyominoes = utils::build_variations(&polyominoes, Restrictions::RectangularSymmetry);
        let _crp = &check_region_pentomino::<Point>;
        let mut b = Board::new(10, 6);
        let start_time = Instant::now();

        let mut solver = Solver::new(&mut b, &all_polyominoes);
        // solver.set_region_checker(crp);
        solver.set_print_solutions(PrintSolutions::PrintTotalOnly);

        let num_solutions = solver.solve();

        let elapsed = start_time.elapsed();
        let elapsed_millis = elapsed.as_secs() * 1000_u64 + elapsed.subsec_millis() as u64;

        println!(
            "{} solutions found in {}ms ({} solutions/second)",
            num_solutions,
            elapsed_millis,
            (num_solutions as f64 / elapsed_millis as f64) * 1000.0
        );
    } else {
        panic!("Can't find polyomino file {}", polyomino_name);
    }
}

fn check_region_pentomino<'a, T:PointT>(_b: &Board<'a, T>, region_size: usize) -> bool {
    region_size % 5 == 0
}
