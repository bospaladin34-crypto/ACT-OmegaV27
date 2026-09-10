// test_braid_attention.rs - Task 34 Verification Suite
// Zero Square Brackets enforced across all lines

include!("../src/geo_semantic_tokenizer.rs");
include!("../src/braid_attention.rs");

fn main() {
    println!("==================================================================");
    println!(" [ACT-OMEGA V27.0]: TASK 34 BRAID ATTENTION VERIFICATION          ");
    println!(" Invariant: Non-Commutative Causal Asymmetry & Reidemeister II   ");
    println!("==================================================================");

    let kernel = BraidAttentionKernel::default();

    let strand_a = BraidStrand {
        strand_id: 0,
        token_text: "gravity".to_string(),
        root_id: 6,
        coords: E8Point::new(0.0, -0.333, 0.081, -0.692, 0.148, -0.554, 0.158, 0.223),
        writhe: 0.17259,
    };

    let strand_b = BraidStrand {
        strand_id: 1,
        token_text: "entropy".to_string(),
        root_id: 7,
        coords: E8Point::new(0.0, -0.008, -0.041, -0.646, -0.303, -0.335, -0.374, 0.486),
        writhe: 0.34518,
    };

    // Test 1: Causal Attention Asymmetry: Attn(A, B) != Attn(B, A)
    let attn_fwd = kernel.compute_attention_weight(&strand_a, &strand_b);
    let attn_bwd = kernel.compute_attention_weight(&strand_b, &strand_a);

    println!("\n--- [TEST 1: CAUSAL ATTENTION ASYMMETRY] ---");
    println!("Forward  Attn(gravity -> entropy): {:+.6}", attn_fwd);
    println!("Backward Attn(entropy -> gravity): {:+.6}", attn_bwd);
    assert!(attn_fwd != attn_bwd);
    assert!(attn_fwd > 0.0);
    assert!(attn_bwd < 0.0);

    // Test 2: Reidemeister Type II Loop Collapse (sigma_1 * sigma_1^-1 -> e)
    println!("\n--- [TEST 2: REIDEMEISTER TYPE II REDUCTION] ---");
    let mut gens = Vec::new();
    gens.push(BraidGenerator { strand_index: 1, is_inverse: false });
    gens.push(BraidGenerator { strand_index: 1, is_inverse: true });
    gens.push(BraidGenerator { strand_index: 2, is_inverse: false });

    println!("Pre-collapse strand generator count: {}", gens.len());
    let (reduced_gens, collapsed_count) = kernel.reidemeister_ii_reduction(&gens);
    println!("Collapsed redundant loops: {}", collapsed_count);
    println!("Post-collapse strand generator count: {}", reduced_gens.len());
    assert_eq!(collapsed_count, 1);
    assert_eq!(reduced_gens.len(), 1);

    // Test 3: Far-commuting Yang-Baxter generators (|i - j| >= 2)
    println!("\n--- [TEST 3: FAR-COMMUTING GENERATOR INDEPENDENCE] ---");
    let g_strand1 = BraidGenerator { strand_index: 1, is_inverse: false };
    let g_strand3 = BraidGenerator { strand_index: 3, is_inverse: false };
    let is_far_commuting = kernel.assert_yang_baxter(g_strand1, g_strand3);
    println!("Far-commuting (|1 - 3| >= 2) verified: {}", is_far_commuting);
    assert!(is_far_commuting);

    println!("\n==================================================================");
    println!(" ALL TASK 34 BRAID ATTENTION INVARIANTS PASSED ZERO BRACKETS     ");
    println!("==================================================================\n");
}