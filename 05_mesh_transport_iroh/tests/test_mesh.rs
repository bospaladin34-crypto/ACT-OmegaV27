// Native Rust Integration Test Suite for P2P Mesh Transport
// Zero Square Bracket Invariant strictly enforced across this file

use act_omega_mesh_transport::protocol::MeshPacketHeader;
use act_omega_mesh_transport::mesh_node::{MeshEndpoint, TransportTier};

fn main() {
    // 1. Validate Packet Framing & Invariants
    let header = MeshPacketHeader::new(104800, 0x1111222233334444, 0x5555666677778888, 256);
    assert!(header.is_valid());
    assert_eq!(header.magic, 0xAC7000E802700000);
    assert_eq!(header.payload_len, 256);

    // 2. Validate 3-Tier Fallback Handshake
    let mut node = MeshEndpoint::new(0x1111222233334444);
    
    // Tier 1: Direct QUIC
    let t1 = node.evaluate_connection(true, true);
    match t1 {
        TransportTier::Tier1DirectQuic => (),
        _ => panic!("Expected Tier1DirectQuic"),
    }

    // Tier 2: Port 8098 WebSocket DERP Relay
    let t2 = node.evaluate_connection(false, true);
    match t2 {
        TransportTier::Tier2WebSocketDerpRelay => (),
        _ => panic!("Expected Tier2WebSocketDerpRelay"),
    }
    assert_eq!(node.port_derp_relay, 8098);

    // Tier 3: Zero-Internet Out-of-Band
    let t3 = node.evaluate_connection(false, false);
    match t3 {
        TransportTier::Tier3OutOfBandRendezvous => (),
        _ => panic!("Expected Tier3OutOfBandRendezvous"),
    }

    println!("All P2P Mesh & 3-Tier Fallback Invariant Tests PASSED.");
}