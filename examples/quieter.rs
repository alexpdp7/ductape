use rayon::prelude::*;
use tracing::instrument;
use tracing_subscriber::filter::LevelFilter;

use ductape;

#[instrument]
pub fn do_stuff(arg: &str) {
    ductape::run(&format!("thread {arg} start"), &["echo", "first"]);
    for i in 0..arg.parse().unwrap() {
        ductape::run(&format!("thread {arg} sleep {i}"), &["sleep", arg]);
        ductape::run(&format!("thread {arg} print {i}"), &["echo", arg]);
    }
    ductape::run(&format!("thread {arg} end"), &["echo", "last"]);
}

pub fn main() {
    ductape::setup(LevelFilter::INFO);
    std::env::args()
        .skip(1)
        .collect::<Vec<_>>()
        .par_iter()
        .for_each(|arg| do_stuff(arg));
}
