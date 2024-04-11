use rayon::prelude::*;
use tracing_subscriber::filter::LevelFilter;

use ductape;

pub fn main() {
    ductape::setup(LevelFilter::INFO);
    std::env::args()
        .skip(1)
        .collect::<Vec<_>>()
        .par_iter()
        .enumerate()
        .for_each(|(i, arg)| ductape::run(&i.to_string(), &["find", arg]));

    std::thread::sleep(std::time::Duration::from_secs(1));
}
