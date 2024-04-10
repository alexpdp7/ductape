Wrapper over duct, indicatif, and tracing, to run and trace `std::process`.

* Checks for return code 0 by default
* Sends stdout and stderr to logs
* Honors RUST_LOG
* Can send traces to OpenTelemetry

```
$ cargo run --example basic -- /etc /etc/
```

```
$ cargo run -F opentelemetry --example quieter 4 5 6
```

(You can use https://github.com/CtrlSpice/otel-desktop-viewer to view OpenTelemetry traces.)
