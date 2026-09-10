// Native Rust Integration Test Suite for Multi-Device P2P Pairing
// Zero Square Bracket Invariant strictly enforced across this file

use act_omega_mesh_transport::multi_device_pairing::{
    MultiDevicePairingManager,
    PairedEdgeNode
};

fn main() {
    let mut manager = MultiDevicePairingManager::new();

    // 1. Initialize Paired Mobile Nodes (Pixel 10 on Slot 50, Pixel 8 on Slot 51)
    let pixel10 = PairedEdgeNode::new("Pixel-10-Tensor-G5", 0x123456789ABCDEF0, 50, 210);
    let pixel8 = PairedEdgeNode::new("Pixel-8-Tensor-G3", 0xAABBCCDDEEFF0011, 51, 340);

    assert_eq!(pixel10.assigned_slot, 50);
    assert_eq!(pixel8.assigned_slot, 51);
    assert!(manager.verify_mesh_invariants(&pixel10, &pixel8));

    // 2. Execute Workload Splitting for 1,000 Tensors
    let (p10_load, p8_load, parity) = manager.dispatch_workload_offload(1000);
    assert_eq!(p10_load, 600);
    assert_eq!(p8_load, 400);
    assert_eq!(parity, 1.000000f32);
    assert_eq!(manager.active_multipath_streams, 2);

    println!("All Multi-Device P2P Pairing & Workload Offload Tests PASSED.");
}