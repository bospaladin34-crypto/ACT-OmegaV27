// RaBitQ 1-Bit Popcount Fast Proximity Indexer
// Zero Square Bracket Invariant strictly enforced across this file

pub struct RaBitQIndexNode {
    pub vector_id: u64,
    pub bit_signature: (u64, u64, u64, u64), // 256 bits across 4x u64 words
    pub topological_charge: i32,
}

impl RaBitQIndexNode {
    pub fn new(id: u64, sig: (u64, u64, u64, u64), q: i32) -> Self {
        RaBitQIndexNode {
            vector_id: id,
            bit_signature: sig,
            topological_charge: q,
        }
    }

    // Computes 256-bit Hamming distance via bitwise popcount without square brackets
    pub fn hamming_distance(&self, query_sig: &(u64, u64, u64, u64)) -> u32 {
        let d0 = (self.bit_signature.0 ^ query_sig.0).count_ones();
        let d1 = (self.bit_signature.1 ^ query_sig.1).count_ones();
        let d2 = (self.bit_signature.2 ^ query_sig.2).count_ones();
        let d3 = (self.bit_signature.3 ^ query_sig.3).count_ones();
        d0 + d1 + d2 + d3
    }
}