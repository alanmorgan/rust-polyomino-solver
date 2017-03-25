use std::collections::HashSet;
use Polyomino;

pub fn make_variations(p: Polyomino) -> HashSet<Polyomino> {
    let mut res = HashSet::new();

    res.insert(p.clone());
    res.insert(p.clone().rotate());
    res.insert(p.clone().rotate().rotate());
    res.insert(p.clone().rotate().rotate().rotate());

    res.insert(p.clone().flip());
    res.insert(p.clone().flip().rotate());
    res.insert(p.clone().flip().rotate().rotate());
    res.insert(p.clone().flip().rotate().rotate().rotate());

    res
}
