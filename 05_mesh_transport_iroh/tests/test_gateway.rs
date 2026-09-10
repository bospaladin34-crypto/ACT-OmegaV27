// Native Rust Integration Test Suite for P2P Device Gateway
// Zero Square Bracket Invariant strictly enforced across this file

use act_omega_mesh_transport::device_gateway::DeviceEnrollmentGateway;

fn main() {
    let mut gateway = DeviceEnrollmentGateway::new();

    // 1. Generate Permanent Device Hash for Pixel 10 Edge Node
    let pixel10_hash = gateway.compute_permanent_device_hash(
        0x123456789ABCDEF0,
        0xFEDCBA9876543210,
        0x9810360000000000
    );
    assert_ne!(pixel10_hash, 0);

    // 2. Test Initial Enrollment (Auto-assigns Slot 50)
    let peer1 = gateway.handle_device_handshake(pixel10_hash, false, 0);
    assert!(peer1.is_authenticated);
    assert_eq!(peer1.assigned_slot, 50);
    assert_eq!(peer1.parity_trace, 1.000000f32);
    assert_eq!(gateway.enrolled_peers_count, 1);

    // 3. Test Initial Enrollment for Second Device (Auto-assigns Slot 51)
    let pixel8_hash = gateway.compute_permanent_device_hash(
        0xAABBCCDDEEFF0011,
        0x2233445566778899,
        0x9810360000000001
    );
    let peer2 = gateway.handle_device_handshake(pixel8_hash, false, 0);
    assert_eq!(peer2.assigned_slot, 51);
    assert_eq!(gateway.enrolled_peers_count, 2);

    // 4. Test Fast-Path Reconnection (< 200 us) on Pixel 10
    let peer1_reconnect = gateway.handle_device_handshake(pixel10_hash, true, 50);
    assert_eq!(peer1_reconnect.assigned_slot, 50);
    assert_eq!(peer1_reconnect.device_hash, pixel10_hash);

    println!("All Automated P2P Device Gateway Invariant Tests PASSED.");
}