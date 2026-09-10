// Carrier-NAT-Proof Packet Protocol & Invariant Framing
// Zero Square Bracket Invariant strictly enforced across this file

pub struct MeshPacketHeader {
    pub magic: u64,
    pub sequence_epoch: u64,
    pub parity_trace: f32,
    pub phase_delta: f32,
    pub landauer_joules: f32,
    pub payload_len: u32,
    pub sender_hash: u64,
    pub receiver_hash: u64,
}

impl MeshPacketHeader {
    pub const fn new(seq: u64, sender: u64, receiver: u64, payload_bytes: u32) -> Self {
        MeshPacketHeader {
            magic: 0xAC7000E802700000,
            sequence_epoch: seq,
            parity_trace: 1.000000,
            phase_delta: 0.17259029,
            landauer_joules: 0.0421,
            payload_len: payload_bytes,
            sender_hash: sender,
            receiver_hash: receiver,
        }
    }

    pub fn is_valid(&self) -> bool {
        let magic_valid = self.magic == 0xAC7000E802700000;
        let parity_valid = (self.parity_trace - 1.000000).abs() < 1e-6;
        let phase_valid = self.phase_delta <= 0.40;
        magic_valid && parity_valid && phase_valid
    }
}