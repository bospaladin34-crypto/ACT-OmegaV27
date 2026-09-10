// Canonical Algebraic Triplet Invariant Schema
// Zero Square Bracket Invariant strictly enforced across this file

pub type E8Root8D = (f32, f32, f32, f32, f32, f32, f32, f32);

pub struct CanonicalTriplet {
    pub tensor_id: &'static str,
    pub tier_level: u32,
    pub braid_word: &'static str,
    pub e8_root: E8Root8D,
    pub topological_charge_q: i32,
    pub writhe: i32,
    pub landauer_joules: f32,
    pub parity_lock: f32,
}

impl CanonicalTriplet {
    pub const fn new(
        id: &'static str,
        tier: u32,
        braid: &'static str,
        root: E8Root8D,
        q: i32,
        w: i32,
        heat: f32,
    ) -> Self {
        CanonicalTriplet {
            tensor_id: id,
            tier_level: tier,
            braid_word: braid,
            e8_root: root,
            topological_charge_q: q,
            writhe: w,
            landauer_joules: heat,
            parity_lock: 1.000000,
        }
    }
}