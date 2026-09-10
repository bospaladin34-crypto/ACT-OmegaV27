// Carrier-NAT-Proof Mesh Transport Root Module
// Zero Square Bracket Invariant strictly enforced across this file

pub mod protocol;
pub mod mesh_node;
pub mod device_gateway;
pub mod multi_device_pairing;

pub const DEFAULT_DERP_RELAY_PORT: u16 = 8098;
pub const MAX_FALLBACK_LATENCY_MS: u64 = 300;