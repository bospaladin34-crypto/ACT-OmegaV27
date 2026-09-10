// Distributed NPU Workload Dispatcher and Result Reassembler
// Zero Square Bracket Invariant strictly enforced across this file

use crate::e8_lattice::{decode_conway_sloane_e8, E8Vector8D, E8QuantizedInt8};

pub struct OffloadWorkloadBatch {
    pub batch_id: u64,
    pub target_slot: u32,
    pub tensor_count: u32,
    pub parity_lock: f32,
}

impl OffloadWorkloadBatch {
    pub const fn new(id: u64, slot: u32, count: u32) -> Self {
        OffloadWorkloadBatch {
            batch_id: id,
            target_slot: slot,
            tensor_count: count,
            parity_lock: 1.000000f32,
        }
    }

    // Prepares raw 8D vector payload for mobile dispatch
    pub fn generate_sample_vector(&self, index: u32) -> E8Vector8D {
        let offset = index as f32 * 0.01f32;
        (1.2f32 + offset, 0.8f32 - offset, 0.5f32, -0.5f32, 0.1f32, 0.0f32, 0.0f32, 0.0f32)
    }

    // Reassembles returned tensor and validates even Gosset parity
    pub fn verify_returned_tensor(&self, raw: &E8Vector8D) -> (E8Vector8D, E8QuantizedInt8, bool) {
        let decoded = decode_conway_sloane_e8(raw);
        let sum = decoded.0 + decoded.1 + decoded.2 + decoded.3 +
                  decoded.4 + decoded.5 + decoded.6 + decoded.7;
        let is_even_parity = (sum as i32) % 2 == 0;

        let quantized: E8QuantizedInt8 = (
            decoded.0.clamp(-128.0f32, 127.0f32) as i8,
            decoded.1.clamp(-128.0f32, 127.0f32) as i8,
            decoded.2.clamp(-128.0f32, 127.0f32) as i8,
            decoded.3.clamp(-128.0f32, 127.0f32) as i8,
            decoded.4.clamp(-128.0f32, 127.0f32) as i8,
            decoded.5.clamp(-128.0f32, 127.0f32) as i8,
            decoded.6.clamp(-128.0f32, 127.0f32) as i8,
            decoded.7.clamp(-128.0f32, 127.0f32) as i8,
        );

        (decoded, quantized, is_even_parity)
    }
}