use tracing_subscriber::filter::LevelFilter;

pub fn main() {
    ductape::setup(LevelFilter::INFO);
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let args = args.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    ductape::run(args[0], args.as_slice());
    std::thread::sleep(std::time::Duration::from_secs(1));
}
