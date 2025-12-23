# Multi-Agent Engine Benchmarks

This is a crate with a collection of benchmarks for Multi-Agent Engine.

## Running benchmarks

Benchmarks can be run through Cargo:

```shell
# Run all benchmarks
cargo bench --package benches

# Just compile the benchmarks, do not run them
cargo bench --package benches --no-run

# Run the benchmarks for a specific crate
cargo bench --package benches --bench multi_agent_engine

# List all available benchmarks
cargo bench --package benches -- --list

# Save a baseline to be compared against later
cargo bench --package benches -- --save-baseline before

# Compare the current benchmarks against a baseline to find performance gains and regressions
cargo bench --package benches -- --baseline before
```

## Criterion

Multi-Agent Engine's benchmarks use [Criterion](https://crates.io/crates/criterion).
If you want to learn more about using Criterion for comparing performance against a baseline or generating detailed reports, you can read the [Criterion.rs documentation](https://bheisler.github.io/criterion.rs/book/criterion_rs.html).

## License

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
