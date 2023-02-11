# User-submitted benchmark results

Feel free to submit a PR where you add your benchmark results to this page. Please include your CPU information, the version of sort_bench you used, and the output of `cargo bench`.


**Intel i7-7700HQ (8) @ 3.800GHz (System76 Oryx Pro running Pop_OS 20.04 LTS)**, running sort_bench v 0.1.1.

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

**Intel Core2 Duo T5850 (2) @ 2.16 GHz (HP Inspiron 1720 running Pop!OS 22.04)**, running sort_bench v 0.1.1.

```
Running benches/sorts.rs (/home/tim/rust/target/release/deps/sorts-6c5bf30a3a258ff9)
Benchmarking SortWords/Using regular stable sort: Warming up for 3.0000 s
Warning: Unable to complete 25 samples in 5.0s. You may wish to increase target time to 8.9s, or reduce sample count to 10.
Benchmarking SortWords/Using regular stable sort: Collecting 2SortWords/Using regular stable sort
                    time:   [346.71 ms 347.11 ms 347.58 ms]
                    change: [+552.17% +553.65% +554.94%] (p = 0.00 < 0.05)
                    Performance has regressed.
Found 1 outliers among 25 measurements (4.00%)
1 (4.00%) high mild
Benchmarking SortWords/Using unstable sort: Warming up for 3.0000 s
Warning: Unable to complete 25 samples in 5.0s. You may wish to increase target time to 10.4s, or reduce sample count to 10.
Benchmarking SortWords/Using unstable sort: Collecting 25 sampSortWords/Using unstable sort
                    time:   [405.33 ms 405.61 ms 405.96 ms]
                    change: [+650.35% +652.43% +654.01%] (p = 0.00 < 0.05)
                    Performance has regressed.
Found 2 outliers among 25 measurements (8.00%)
2 (8.00%) high severe
Benchmarking SortWords/Using glidesort: Warming up for 3.0000 s
Warning: Unable to complete 25 samples in 5.0s. You may wish to increase target time to 10.6s, or reduce sample count to 10.
Benchmarking SortWords/Using glidesort: Collecting 25 samples SortWords/Using glidesort
                    time:   [410.01 ms 410.73 ms 411.72 ms]
                    change: [+662.63% +664.67% +666.99%] (p = 0.00 < 0.05)
                    Performance has regressed.
Found 3 outliers among 25 measurements (12.00%)
1 (4.00%) high mild
2 (8.00%) high severe
```


**AMD Ryzen 7 2700 (8) @ 3.2GHz (HP Desktop model number 690-0034 running Debian bookworm)**, running sort_bench v 0.1.1.

```
     Running benches/sorts.rs (/home/tim/rust/target/release/deps/sorts-6c5bf30a3a258ff9)
Benchmarking SortWords/Using regular stable sort: Collecting 25 samples in estimSortWords/Using regular stable sort
                        time:   [117.66 ms 117.84 ms 118.02 ms]
                        change: [+0.0482% +0.2929% +0.5159%] (p = 0.02 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 25 measurements (4.00%)
  1 (4.00%) high mild
Benchmarking SortWords/Using unstable sort: Collecting 25 samples in estimated 7SortWords/Using unstable sort
                        time:   [136.50 ms 136.86 ms 137.24 ms]
                        change: [-1.0287% -0.6113% -0.2154%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 25 measurements (4.00%)
  1 (4.00%) high mild
Benchmarking SortWords/Using glidesort: Collecting 25 samples in estimated 7.118SortWords/Using glidesort
                        time:   [138.50 ms 138.91 ms 139.36 ms]
                        change: [-1.1884% -0.8165% -0.4408%] (p = 0.00 < 0.05)
                        Change within noise threshold.
```
