# Benchmarking different word-sorting options

Why I'm doing this: [https://github.com/sts10/tidy/pull/33](https://github.com/sts10/tidy/pull/33)

## Running the benchmarks
Run `cargo bench` to run benchmarks.

## My initial results

```text
SortWords/Using regular stable sort
                        time:   [18.010 ms 18.116 ms 18.239 ms]
                        change: [-4.0765% -2.9408% -1.8513%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
SortWords/Using unstable sort
                        time:   [17.559 ms 17.655 ms 17.781 ms]
                        change: [-9.8066% -8.0489% -6.3420%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe
SortWords/Using glidesort
                        time:   [17.540 ms 17.585 ms 17.635 ms]
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe
```
