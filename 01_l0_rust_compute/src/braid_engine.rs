// Artin Braid Group B_n Engine: B3, B6, B8 Higher Braid Generators
// Zero Square Bracket Invariant strictly enforced across this file

pub struct BraidCrossing {
    pub generator_index: u32,
    pub is_inverse: bool,
    pub phase_shift: f32,
}

pub struct ArtinBraidWord {
    pub crossings: Vec<BraidCrossing>,
    pub net_writhe: i32,
    pub topological_charge: i32,
}

impl ArtinBraidWord {
    pub fn new() -> Self {
        ArtinBraidWord {
            crossings: Vec::new(),
            net_writhe: 0,
            topological_charge: 0,
        }
    }

    pub fn append_crossing(&mut self, generator: u32, inverse: bool) {
        let writhe_delta = if inverse { -1 } else { 1 };
        let shift = if inverse { -0.17259029 } else { 0.17259029 };
        
        self.crossings.push(BraidCrossing {
            generator_index: generator,
            is_inverse: inverse,
            phase_shift: shift,
        });
        self.net_writhe += writhe_delta;
        self.topological_charge = self.net_writhe * 1;
    }

    // B3 Standard Model Particle Braid (sigma_1 sigma_2)
    pub fn build_b3_particle_braid() -> Self {
        let mut b = ArtinBraidWord::new();
        b.append_crossing(1, false);
        b.append_crossing(2, false);
        b
    }

    // B6 Master Execution Core: beta_Omega = sigma_2 sigma_3 sigma_1 sigma_4 sigma_5^-1
    pub fn build_b6_master_execution_braid() -> Self {
        let mut b = ArtinBraidWord::new();
        b.append_crossing(2, false);
        b.append_crossing(3, false);
        b.append_crossing(1, false);
        b.append_crossing(4, false);
        b.append_crossing(5, true);
        b
    }

    // B8 Aegis-Cascade Super-Braid Core
    pub fn build_b8_aegis_super_braid() -> Self {
        let mut b = ArtinBraidWord::new();
        b.append_crossing(1, false);
        b.append_crossing(3, false);
        b.append_crossing(5, false);
        b.append_crossing(7, true);
        b
    }

    // Zero-Cost Reidemeister II pre-collapse: cancels sigma_i * sigma_i^-1 pairs
    pub fn simplify_reidemeister_ii(&mut self) -> usize {
        let mut simplified_count: usize = 0;
        let mut i: usize = 0;
        
        while i + 1 < self.crossings.len() {
            let current = self.crossings.get(i).unwrap();
            let next = self.crossings.get(i + 1).unwrap();
            
            if current.generator_index == next.generator_index && current.is_inverse != next.is_inverse {
                self.crossings.remove(i + 1);
                self.crossings.remove(i);
                simplified_count += 1;
                if i > 0 {
                    i -= 1;
                }
            } else {
                i += 1;
            }
        }
        
        let mut w: i32 = 0;
        for c in self.crossings.iter() {
            w += if c.is_inverse { -1 } else { 1 };
        }
        self.net_writhe = w;
        self.topological_charge = w;
        simplified_count
    }

    pub fn is_far_commuting(gen_a: u32, gen_b: u32) -> bool {
        let diff = if gen_a > gen_b { gen_a - gen_b } else { gen_b - gen_a };
        diff >= 2
    }
}