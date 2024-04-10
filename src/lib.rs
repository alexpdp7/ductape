use std::io;
use std::io::BufRead;

use duct;

use tracing::instrument;
use tracing::{debug, info};
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn setup() {
    let indicatif_layer = IndicatifLayer::new();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_writer(indicatif_layer.get_stderr_writer()))
        .with(indicatif_layer)
        .init();
}

#[instrument]
pub fn run(id: &str, cmd: &[&str]) {
    let (program, args) = cmd.split_at(1);
    io::BufReader::new(
        duct::cmd(program[0], args)
            .stderr_to_stdout()
            .reader()
            .unwrap(),
    )
    .lines()
    .for_each(|l| info!("{}", l.unwrap()));
}
