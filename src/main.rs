use std::time::Instant;

use polyomino::board::Board;
use polyomino::point::SimplePoint;
use polyomino::polyomino::Polyomino;
use polyomino::polyomino::SimplePolyomino;
use polyomino::utils;
use polyomino::utils::Restrictions;
use polyomino::utils::PredefinedPolyominoes;
use polyomino::solver::Solver;

fn main() {
    let polyomino_name = "data/pentomino.poly";

    if let Ok(polyominoes) = utils::get_polyominoes::<SimplePolyomino<SimplePoint>>(PredefinedPolyominoes::Pentominoes) {

        // Comparing performance of merely counting the number of solutions vs computing
        // and returning each solution
        get_number_of_solutions(&polyominoes);
        
        get_solutions(&polyominoes);
    } else {
        panic!("Can't find polyomino file {}", polyomino_name);
    }
}

fn get_solutions<'a> (polyominoes: &[SimplePolyomino<SimplePoint>]) {
    let all_polyominoes = utils::build_variations(polyominoes, Restrictions::RectangularSymmetry);
    let mut b = Board::new(10, 6);
    let mut solver = Solver::new(&mut b, &all_polyominoes);
    // solver.set_callback_function(&call_back::<(), SimplePoint>);
    // solver.set_region_checker(&check_region_pentomino::<SimplePolyomino<SimplePoint>>);
    let start_time = Instant::now();
    let num_solutions = solver.solve().len();
    let elapsed = start_time.elapsed();
    let elapsed_millis = elapsed.as_secs() * 1000_u64 + elapsed.subsec_millis() as u64;
    
    println!(
        "{} solutions found in {}ms ({} solutions/second)",
        num_solutions,
        elapsed_millis,
        ((num_solutions as f64 / elapsed_millis as f64) * 1000.0).round()
            );
}

fn get_number_of_solutions<'a> (polyominoes: &[SimplePolyomino<SimplePoint>]) {
    let all_polyominoes = utils::build_variations(polyominoes, Restrictions::RectangularSymmetry);
    let mut b = Board::new(10, 6);
    let mut solver = Solver::new(&mut b, &all_polyominoes);
    // solver.set_callback_function(&call_back::<(), SimplePoint>);
    // solver.set_region_checker(&check_region_pentomino::<SimplePolyomino<SimplePoint>>);
    let start_time = Instant::now();
    let num_solutions = solver.count_solutions();
    let elapsed = start_time.elapsed();
    let elapsed_millis = elapsed.as_secs() * 1000_u64 + elapsed.subsec_millis() as u64;
    
    println!(
        "{} solutions found in {}ms ({} solutions/second)",
        num_solutions,
        elapsed_millis,
        ((num_solutions as f64 / elapsed_millis as f64) * 1000.0).round()
            );
}

#[allow(dead_code)]
fn call_back<P:Polyomino>(b: &Board<P>) {
    println!("{}", b)
}

#[allow(dead_code)]
fn check_region_pentomino<P:Polyomino>(_b: &Board<P>, region_size: usize) -> bool {
    region_size % 5 == 0
}
