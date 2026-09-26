//! Heap allocation report for generating sitemap XML.
//!
//! Runs each scenario once under the `dhat` allocator and prints the number of allocations, the
//! total bytes allocated, and the peak heap usage of the measured operation. The output is
//! deterministic, so reports from two branches can be compared with `diff`.
//!
//! This lives in its own binary because `dhat` slows down every allocation, which would distort
//! the wall-clock benchmarks.

mod common;

use common::Scenario;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    println!(
        "{:<20} {:>12} {:>14} {:>14}",
        "scenario", "allocations", "total_bytes", "peak_bytes"
    );
    for (name, scenario) in Scenario::all() {
        // the output buffer is allocated inside the profiled region on purpose: users pay for it too
        let profiler = dhat::Profiler::builder().testing().build();
        let mut buf: Vec<u8> = Vec::new();
        scenario.run(&mut buf);
        let stats = dhat::HeapStats::get();
        drop(buf);
        drop(profiler);

        println!(
            "{:<20} {:>12} {:>14} {:>14}",
            name, stats.total_blocks, stats.total_bytes, stats.max_bytes
        );
    }
}
