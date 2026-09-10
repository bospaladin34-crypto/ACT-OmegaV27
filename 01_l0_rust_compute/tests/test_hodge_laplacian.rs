// Native Rust Integration Test Suite for Sheaf Hodge Laplacian Engine
// Zero Square Bracket Invariant strictly enforced across this file

use vesper_ffi::hodge_laplacian::SheafHodgeLaplacian;

fn main() {
    let laplacian = SheafHodgeLaplacian::new();
    assert_eq!(laplacian.stalk_dimension, 8);
    assert_eq!(laplacian.parity_lock, 1.000000f32);

    let stalk_a = (1.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32);
    let stalk_b = (0.0f32, 1.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32);

    let (diffused, is_ok) = laplacian.compute_sheaf_diffusion(&stalk_a, &stalk_b, 1.0f32);
    assert!(is_ok);

    let sum = diffused.0 + diffused.1 + diffused.2 + diffused.3 + diffused.4 + diffused.5 + diffused.6 + diffused.7;
    assert_eq!((sum as i32) % 2, 0);

    println!("All Sheaf Hodge Laplacian Diffusion Invariant Tests PASSED.");
}