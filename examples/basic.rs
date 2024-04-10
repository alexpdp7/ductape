use rayon::prelude::*;

use ductape;

pub fn main() {
    ductape::setup();
    std::env::args()
        .skip(1)
        .collect::<Vec<_>>()
        .par_iter()
        .enumerate()
        .for_each(|(i, arg)| ductape::run(&i.to_string(), &["find", arg]));
}
