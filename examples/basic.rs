use ductape;
use rayon::prelude::*;
use tracing_subscriber::filter::LevelFilter;

pub fn main() {
    ductape::setup(LevelFilter::INFO);
    std::env::args()
        .skip(1)
        .collect::<Vec<_>>()
        .par_iter()
        .enumerate()
        .for_each(|(i, arg)| ductape::run(&i.to_string(), &["find", arg]));
    //    opentelemetry::global::shutdown_tracer_provider();
}
