// Native Rust Integration Test Suite for Landauer Benchmark Suite
// Zero Square Bracket Invariant strictly enforced across this file

use vesper_ffi::benchmark_suite::LandauerBenchmarkSuite;

fn main() {
    let suite = LandauerBenchmarkSuite::new(10000);
    let results = suite.run_benchmark_cycles();

    assert_eq!(results.total_iterations, 10000);
    assert_eq!(results.parity_trace, 1.000000f32);
    assert!(results.invariants_conserved);
    assert!(results.total_landauer_joules <= 1.4411f32);
    assert!(results.gigabytes_per_second > 1.0f32);

    println!("All Landauer Throughput & Invariant Benchmark Tests PASSED.");
}