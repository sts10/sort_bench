# Benchmarking different word-sorting options

Why I'm doing this: [https://github.com/sts10/tidy/pull/33](https://github.com/sts10/tidy/pull/33)

## Running the benchmarks
Run `cargo bench` to run benchmarks.

## My initial results

Here's what I get running `cargo bench` on my Intel i7-7700HQ (8) @ 3.800GHz (System76 Oryx Pro running Pop_OS 20.04 LTS):

```text
Benchmarking SortWords/Using regular stable sort: Collecting 25 samples SortWords/Using regular stable sort
        time:   [184.27 ms 192.76 ms 202.13 ms]
        change: [-1.0745% +3.9177% +9.7754%] (p = 0.17 > 0.05)
        No change in performance detected.
Found 1 outliers among 25 measurements (4.00%)
1 (4.00%) high mild
Benchmarking SortWords/Using unstable sort: Warming up for 3.0000 s
Warning: Unable to complete 25 samples in 5.0s. You may wish to increase target time to 5.3s, or reduce sample count to 20.
Benchmarking SortWords/Using unstable sort: Collecting 25 samples in estSortWords/Using unstable sort
        time:   [210.85 ms 214.36 ms 217.97 ms]
        change: [-5.8918% -2.5614% +0.8032%] (p = 0.17 > 0.05)
        No change in performance detected.
Benchmarking SortWords/Using glidesort: Warming up for 3.0000 s
Warning: Unable to complete 25 samples in 5.0s. You may wish to increase target time to 5.8s, or reduce sample count to 20.
Benchmarking SortWords/Using glidesort: Collecting 25 samples in estimatSortWords/Using glidesort
        time:   [215.95 ms 220.46 ms 225.15 ms]
        change: [-0.7737% +1.5986% +4.1985%] (p = 0.21 > 0.05)
        No change in performance detected.
```
