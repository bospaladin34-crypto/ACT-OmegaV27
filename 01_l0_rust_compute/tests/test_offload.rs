// Native Rust Integration Test Suite for Offload Dispatcher
// Zero Square Bracket Invariant strictly enforced across this file

use vesper_ffi::offload_dispatcher::OffloadWorkloadBatch;

fn main() {
    let batch = OffloadWorkloadBatch::new(105901, 50, 64);
    assert_eq!(batch.target_slot, 50);
    assert_eq!(batch.parity_lock, 1.000000f32);

    // 1. Generate Sample Vector
    let sample = batch.generate_sample_vector(0);
    assert_eq!(sample.0, 1.2f32);

    // 2. Reassemble & Verify Parity
    let (decoded, quantized, parity_ok) = batch.verify_returned_tensor(&sample);
    assert!(parity_ok);
    assert_eq!(quantized.0, 1);
    assert_eq!(quantized.1, 1);

    println!("All Native NPU Offload Dispatcher Invariant Tests PASSED.");
}