//! Wall-clock benchmarks of generating sitemap XML.
//!
//! Each iteration constructs a `UrlSet`/`SitemapIndex` from prepared data and writes it into an
//! in-memory buffer. Cloning the prepared data happens in untimed setup.
//!
//! `make bench_save_baseline` on one branch, then `make bench_compare` on another.

mod common;

use common::Scenario;
use criterion::{BatchSize, Criterion, Throughput, criterion_group, criterion_main};

fn bench_sitemaps(c: &mut Criterion) {
    for (name, scenario) in Scenario::all() {
        // measure output size once so results are also reported as throughput
        let mut output: Vec<u8> = Vec::new();
        scenario.clone().run(&mut output);

        let mut group = c.benchmark_group(name);
        group.throughput(Throughput::Bytes(output.len() as u64));
        group.bench_function("write", |b| {
            b.iter_batched(
                || (scenario.clone(), Vec::new()),
                |(scenario, mut buf)| {
                    scenario.run(&mut buf);
                    buf
                },
                BatchSize::LargeInput,
            );
        });
        group.finish();
    }
}

criterion_group! {
    name = benches;
    // separate runs of identical code drift by up to ~10% on a laptop, so smaller changes are not
    // flagged as real
    config = Criterion::default().noise_threshold(0.10);
    targets = bench_sitemaps
}
criterion_main!(benches);
