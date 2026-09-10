// Automated P2P Device Enrollment Gateway & Fast-Path Router
// Zero Square Bracket Invariant strictly enforced across this file

pub struct PeerDeviceRecord {
    pub device_hash: u64,
    pub assigned_slot: u32,
    pub is_authenticated: bool,
    pub total_frames_processed: u64,
    pub parity_trace: f32,
}

impl PeerDeviceRecord {
    pub const fn new(hash: u64, slot: u32) -> Self {
        PeerDeviceRecord {
            device_hash: hash,
            assigned_slot: slot,
            is_authenticated: true,
            total_frames_processed: 0,
            parity_trace: 1.000000f32,
        }
    }
}

pub struct DeviceEnrollmentGateway {
    pub next_expansion_slot: u32,
    pub enrolled_peers_count: u32,
    pub active_connections: u32,
}

impl DeviceEnrollmentGateway {
    pub const fn new() -> Self {
        DeviceEnrollmentGateway {
            next_expansion_slot: 50,
            enrolled_peers_count: 0,
            active_connections: 0,
        }
    }

    pub fn compute_permanent_device_hash(&self, pub_key_high: u64, pub_key_low: u64, entropy: u64) -> u64 {
        let h1 = pub_key_high.rotate_left(13) ^ entropy;
        let h2 = pub_key_low.rotate_right(7) ^ 0xAC7000E802700000;
        h1 ^ h2
    }

    pub fn handle_device_handshake(&mut self, device_hash: u64, is_known_peer: bool, existing_slot: u32) -> PeerDeviceRecord {
        if is_known_peer {
            self.active_connections += 1;
            PeerDeviceRecord::new(device_hash, existing_slot)
        } else {
            let slot = self.next_expansion_slot;
            self.next_expansion_slot += 1;
            self.enrolled_peers_count += 1;
            self.active_connections += 1;
            PeerDeviceRecord::new(device_hash, slot)
        }
    }
}