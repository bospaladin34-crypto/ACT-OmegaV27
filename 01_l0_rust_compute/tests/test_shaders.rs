// Native Rust Integration Test Suite for CubeCL GPU Compute Pipeline
// Zero Square Bracket Invariant strictly enforced across this file

use vesper_ffi::shaders::cubecl_pipeline::{
    GpuComputeDispatchHeader,
    gpu_kernel_conway_sloane_e8_project,
    gpu_kernel_artin_braid_transform,
    gpu_kernel_penrose_5grid_dualize
};

fn main() {
    // 1. Validate GPU Dispatch Header
    let header = GpuComputeDispatchHeader::new(256);
    assert_eq!(header.magic, 0xAC7000E802700000);
    assert_eq!(header.workgroup_size_x, 64);
    assert_eq!(header.parity_lock, 1.000000);

    // 2. Validate GPU E8 Lattice Projection Kernel
    let input = (1.2, 0.8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let (e8_root, quantized) = gpu_kernel_conway_sloane_e8_project(&input, 1.0);
    let sum = e8_root.0 + e8_root.1 + e8_root.2 + e8_root.3 + e8_root.4 + e8_root.5 + e8_root.6 + e8_root.7;
    assert_eq!((sum as i32) % 2, 0);
    assert_eq!(quantized.0, 1);
    assert_eq!(quantized.1, 1);

    // 3. Validate GPU Artin Braid Generator Matrix Transform Kernel
    let (tx, ty) = gpu_kernel_artin_braid_transform(1.0, 0.0, 1, false);
    let expected_cos = 0.17259029_f32.cos();
    let expected_sin = 0.17259029_f32.sin();
    assert!((tx - expected_cos).abs() < 1e-5);
    assert!((ty - expected_sin).abs() < 1e-5);

    // 4. Validate GPU Penrose 5-Grid Aperiodic Coordinate Dualization Kernel
    let (k0, _k1, _k2, _k3, _k4) = gpu_kernel_penrose_5grid_dualize(2.5, 3.5);
    assert_eq!(k0, 2.0);

    println!("All Native CubeCL GPU Compute Shader Pipeline Tests PASSED.");
}