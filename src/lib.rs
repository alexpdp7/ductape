use std::io;
use std::io::BufRead;

use tracing::info;
use tracing::instrument;
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};
use tracing_subscriber::prelude::*;

pub fn setup(default_level_filter: LevelFilter) {
    let indicatif_layer = IndicatifLayer::new();

    let subscriber = tracing_subscriber::registry();

    let filter = EnvFilter::builder()
        .with_default_directive(default_level_filter.into())
        .from_env_lossy();

    let subscriber = subscriber
        .with(filter)
        .with(tracing_subscriber::fmt::layer().with_writer(indicatif_layer.get_stderr_writer()))
        .with(indicatif_layer);

    #[cfg(feature = "opentelemetry")]
    {
        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(opentelemetry_otlp::new_exporter().http())
            .install_simple()
            .unwrap();

        let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);
        let subscriber = subscriber.with(telemetry_layer);
        subscriber.init();
    }

    #[cfg(not(feature = "opentelemetry"))]
    {
        subscriber.init();
    }
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
