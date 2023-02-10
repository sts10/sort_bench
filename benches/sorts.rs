use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use icu::locid::Locale;
use sort_bench::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

pub fn make_vec_from_file(filename: &PathBuf) -> Vec<String> {
    let mut word_list: Vec<String> = [].to_vec();
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        word_list.push(l);
    }
    word_list
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("SortWords");
    group.sample_size(25);
    let locale: Locale = "en-US".parse().unwrap();

    // https://docs.rs/criterion/latest/criterion/struct.Bencher.html#method.iter_batched_ref
    let mut unsorted_word_list = make_vec_from_file(&PathBuf::from("./benches/wordlist.txt"));
    group.bench_function("Using regular stable sort", |b| {
        b.iter_batched_ref(
            || unsorted_word_list.clone(),
            |mut uwl| stable_sort(&mut uwl, &locale),
            BatchSize::SmallInput,
        )
    });
    let mut unsorted_word_list = make_vec_from_file(&PathBuf::from("./benches/wordlist.txt"));
    group.bench_function("Using unstable sort", |b| {
        b.iter_batched_ref(
            || unsorted_word_list.clone(),
            |mut uwl| unstable_sort(&mut uwl, &locale),
            BatchSize::SmallInput,
        )
    });

    let mut unsorted_word_list = make_vec_from_file(&PathBuf::from("./benches/wordlist.txt"));
    group.bench_function("Using glidesort", |b| {
        b.iter_batched_ref(
            || unsorted_word_list.clone(),
            |mut uwl| glidesort(&mut uwl, &locale),
            BatchSize::SmallInput,
        )
    });
    // group.bench_function("Using regular stable sort", |b| {
    //     b.iter(|| stable_sort(&mut unsorted_word_list, &locale))
    // });

    // let mut unsorted_word_list = make_vec_from_file(&PathBuf::from("./benches/wordlist.txt"));
    // group.bench_function("Using unstable sort", |b| {
    //     b.iter(|| unstable_sort(&mut unsorted_word_list, &locale))
    // });

    // let mut unsorted_word_list = make_vec_from_file(&PathBuf::from("./benches/wordlist.txt"));
    // group.bench_function("Using glidesort", |b| {
    //     b.iter(|| glidesort(&mut unsorted_word_list, &locale))
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
