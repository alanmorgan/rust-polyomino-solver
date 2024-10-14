use std::time::Instant;

use polyomino::board::Board;
use polyomino::point::Point;
use polyomino::point::Pt;
use polyomino::polyomino::TagTrait;
use polyomino::utils;
use polyomino::utils::Restrictions;
use polyomino::utils::PredefinedPolyominoes;
use polyomino::solver::Solver;
use polyomino::solver::SolverResult;

fn main() {
    let polyomino_name = "data/pentomino.poly";

    if let Ok(polyominoes) = utils::get_polyominoes::<(), Point>(PredefinedPolyominoes::Pentominoes, &Point::new) {
        let all_polyominoes = utils::build_variations(&polyominoes, Restrictions::RectangularSymmetry);
        let mut b = Board::new(10, 6);
        let start_time = Instant::now();

        let mut solver = Solver::new(&mut b, &all_polyominoes);
        // solver.set_region_checker(&check_region_pentomino::<Point>);
        // solver.set_callback_function(&call_back::<(), Point>);

        if let SolverResult::Count(num_solutions) = solver.solve() {
            let elapsed = start_time.elapsed();
            let elapsed_millis = elapsed.as_secs() * 1000_u64 + elapsed.subsec_millis() as u64;

            println!(
                "{} solutions found in {}ms ({} solutions/second)",
                num_solutions,
                elapsed_millis,
                ((num_solutions as f64 / elapsed_millis as f64) * 1000.0).round()
                    );
        }
    } else {
        panic!("Can't find polyomino file {}", polyomino_name);
    }
}

#[allow(dead_code)]
fn call_back<S: TagTrait, T:Pt>(b: &Board<S, T>) {
    println!("{}", b)
}

#[allow(dead_code)]
fn check_region_pentomino<S: TagTrait, T:Pt>(_b: &Board<S, T>, region_size: usize) -> bool {
    region_size % 5 == 0
}
