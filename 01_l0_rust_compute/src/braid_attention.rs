// braid_attention.rs - Task 34 Non-Commutative Braid Attention Engine
// Enforces Zero Square Bracket Invariant across entire module


pub struct BraidGenerator {
    pub strand_index: usize,
    pub is_inverse: bool,
}

impl Clone for BraidGenerator {
    fn clone(&self) -> Self {
        Self {
            strand_index: self.strand_index,
            is_inverse: self.is_inverse,
        }
    }
}
impl Copy for BraidGenerator {}

pub struct BraidStrand {
    pub strand_id: usize,
    pub token_text: String,
    pub root_id: u16,
    pub coords: E8Point,
    pub writhe: f32,
}

impl Clone for BraidStrand {
    fn clone(&self) -> Self {
        Self {
            strand_id: self.strand_id,
            token_text: self.token_text.clone(),
            root_id: self.root_id,
            coords: self.coords,
            writhe: self.writhe,
        }
    }
}

pub struct BraidAttentionKernel {
    pub gamma_fric: f32,
    pub strand_count: usize,
}

impl Default for BraidAttentionKernel {
    fn default() -> Self {
        Self {
            gamma_fric: 0.00000000013479,
            strand_count: 8,
        }
    }
}

impl BraidAttentionKernel {
    pub fn compute_attention_weight(
        &self,
        source: &BraidStrand,
        target: &BraidStrand,
    ) -> f32 {
        let geom_dot = source.coords.dot(&target.coords);
        let s_norm = source.coords.norm_sq().sqrt();
        let t_norm = target.coords.norm_sq().sqrt();
        let cos_sim = if s_norm > 0.00001 && t_norm > 0.00001 {
            geom_dot / (s_norm * t_norm)
        } else {
            0.0
        };

        let writhe_delta = (source.writhe - target.writhe).abs();
        let friction_damping = (-self.gamma_fric * writhe_delta).exp();

        // Non-commutative causal asymmetry:
        // Forward crossings (i < j) have positive chiral orientation (+1.0)
        // Backward crossings (i > j) have phase-inverted non-commutative reflection (-phi^-1)
        let causal_phase = if source.strand_id < target.strand_id {
            1.0f32
        } else if source.strand_id > target.strand_id {
            -0.61803398875f32
        } else {
            1.0f32
        };

        cos_sim * friction_damping * causal_phase
    }

    pub fn reidemeister_ii_reduction(
        &self,
        generators: &Vec<BraidGenerator>,
    ) -> (Vec<BraidGenerator>, usize) {
        let mut reduced = Vec::new();
        let mut collapsed_count = 0usize;
        let mut i = 0;
        while i < generators.len() {
            let current = generators.get(i).copied();
            let next = generators.get(i + 1).copied();

            match (current, next) {
                (Some(c), Some(n)) => {
                    // sigma_i * sigma_i^-1 -> e (identity collapse)
                    if c.strand_index == n.strand_index && c.is_inverse != n.is_inverse {
                        collapsed_count = collapsed_count + 1;
                        i = i + 2; // Collapse and skip redundant pair
                    } else {
                        reduced.push(c);
                        i = i + 1;
                    }
                }
                (Some(c), None) => {
                    reduced.push(c);
                    i = i + 1;
                }
                _ => {
                    i = i + 1;
                }
            }
        }
        (reduced, collapsed_count)
    }

    pub fn assert_yang_baxter(
        &self,
        g1: BraidGenerator,
        g2: BraidGenerator,
    ) -> bool {
        // Far-commuting generator independence: |i - j| >= 2 => sigma_i * sigma_j == sigma_j * sigma_i
        let idx_diff = if g1.strand_index >= g2.strand_index {
            g1.strand_index - g2.strand_index
        } else {
            g2.strand_index - g1.strand_index
        };
        idx_diff >= 2
    }
}