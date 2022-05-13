# Dispach Cost Benchmarking

This repository is intended to measure the overhead of dynamic trait dispatching.

You can see the benchmark result for different implementations with:
```bash
$ rustup install nightly-2022-03-01
$ cargo +nightly-2022-03-01 bench --bench main # You need nightly to benchmarking with "test" crate
```

## Benchmark
Results for benchmark of recursive trait function calls.  
Each benchmark name represents its number of recursions.
```log
running 12 tests
test dyn_dispatch1_100        ... bench:     283,071 ns/iter (+/- 11,475)
test dyn_dispatch1_10000      ... bench:     280,281 ns/iter (+/- 7,214)
test dyn_dispatch1_1000000    ... bench:     283,611 ns/iter (+/- 13,497)
test dyn_dispatch2_100        ... bench:     283,894 ns/iter (+/- 9,384)
test dyn_dispatch2_10000      ... bench:     282,723 ns/iter (+/- 1,508)
test dyn_dispatch2_1000000    ... bench:     282,295 ns/iter (+/- 4,623)
test static_dispatch1_100     ... bench:     272,341 ns/iter (+/- 4,641)
test static_dispatch1_10000   ... bench:     273,054 ns/iter (+/- 2,080)
test static_dispatch1_1000000 ... bench:     272,374 ns/iter (+/- 5,661)
test static_dispatch2_100     ... bench:     267,675 ns/iter (+/- 5,297)
test static_dispatch2_10000   ... bench:     268,102 ns/iter (+/- 6,114)
test static_dispatch2_1000000 ... bench:     268,240 ns/iter (+/- 5,940)
```
Dynamic dispatch seems to be slightly slower than static one, but it would not be critical performance problem because it's rate case where dynamic trait function calls occupy the majority of overall function calls of the program.

Executed on:
```
OS:
- macOS Big Sur 11.6
Machine:
- Macbook Pro Apple M1 (16GB)
```