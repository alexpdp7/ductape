Wrapper over duct, indicatif, and tracing, to run and trace `std::process`.

* Checks for return code 0 by default
* Sends stdout and stderr to logs

```
$ cargo run --example basic -- /etc /etc/
```
