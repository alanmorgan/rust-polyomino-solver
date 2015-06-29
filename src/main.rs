#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Polyomino {
    points: Vec<Point>
}

fn main() {
    let p = Polyomino{points: vec![Point{x:0,y:1}, Point{x:1,y:0}, Point{x:1,y:1}, Point{x:1,y:2}, Point{x:2,y:2}]};

    println!("p = {:?}", p);
}
      
