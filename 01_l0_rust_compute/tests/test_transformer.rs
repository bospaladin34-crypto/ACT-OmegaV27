// Native Rust Integration Test Suite for Topological Transformer & Attention
// Zero Square Bracket Invariant strictly enforced across this file

use vesper_ffi::topological_transformer::{TopologicalAttentionHead, TopologicalTransformerBlock};

fn main() {
    // 1. Validate Non-Commutative Braid Attention
    let head = TopologicalAttentionHead::new(0, 8);
    let token = (1.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32);

    let (projected_adj, writhe_adj) = head.compute_braid_attention(0, 1, &token);
    assert_eq!(writhe_adj, 1);
    let sum_adj = projected_adj.0 + projected_adj.1 + projected_adj.2 + projected_adj.3 +
                  projected_adj.4 + projected_adj.5 + projected_adj.6 + projected_adj.7;
    assert_eq!((sum_adj as i32) % 2, 0);

    // 2. Validate Identity Self-Attention Reidemeister Cancellation
    let (_projected_self, writhe_self) = head.compute_braid_attention(1, 1, &token);
    assert_eq!(writhe_self, 0);

    // 3. Validate Multi-Head Transformer Block Forward Pass
    let block = TopologicalTransformerBlock::new();
    let token_b = (0.0f32, 1.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32);
    let (final_vector, net_w, parity) = block.forward(&token, &token_b);

    assert_eq!(parity, 1.000000f32);
    assert_eq!(net_w, 2);
    let sum_final = final_vector.0 + final_vector.1 + final_vector.2 + final_vector.3 +
                    final_vector.4 + final_vector.5 + final_vector.6 + final_vector.7;
    assert_eq!((sum_final as i32) % 2, 0);

    println!("All Custom Topological Attention & Transformer Tests PASSED.");
}