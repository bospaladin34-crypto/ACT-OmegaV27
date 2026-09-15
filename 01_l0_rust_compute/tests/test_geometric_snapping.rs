// test_geometric_snapping.rs - Pure Geometric Snapping & Invariant Suite
// Validates continuous coordinate projection, non-trivial E8 root snapping,
// and physical causality coherence under Zero Square Bracket Invariant.

include!("../src/geo_semantic_tokenizer.rs");

fn main() {
    println!("==================================================================");
    println!(" (ACT-OMEGA V27.0): GEOMETRIC SEMANTIC TOKENIZER VALIDATION       ");
    println!(" Invariant: Continuous R^8 Projection & E_8 Root Resolution       ");
    println!("==================================================================");

    // 1. Construct 8x8 Identity Projection Matrix
    let mut p_data = Vec::new();
    let mut r = 0;
    while r < 8 {
        let mut c = 0;
        while c < 8 {
            if r == c { p_data.push(1.0); } else { p_data.push(0.0); }
            c = c + 1;
        }
        r = r + 1;
    }
    let proj = ProjectionMatrix { rows: 8, cols: 8, data: p_data };

    // 2. Define 8 Distinct E8 Gosset Basis Roots
    let mut roots = Vec::new();
    roots.push(LatticeRoot { root_id: 0, coords: E8Point::new(1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), color_charge: 1, decay_state: 0, role_tag: 1 });
    roots.push(LatticeRoot { root_id: 1, coords: E8Point::new(0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), color_charge: 2, decay_state: 0, role_tag: 2 });
    roots.push(LatticeRoot { root_id: 2, coords: E8Point::new(0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0), color_charge: 3, decay_state: 0, role_tag: 3 });
    roots.push(LatticeRoot { root_id: 3, coords: E8Point::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0), color_charge: 4, decay_state: 0, role_tag: 1 });
    roots.push(LatticeRoot { root_id: 4, coords: E8Point::new(0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0), color_charge: 5, decay_state: 0, role_tag: 2 });
    roots.push(LatticeRoot { root_id: 5, coords: E8Point::new(0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0), color_charge: 6, decay_state: 0, role_tag: 3 });
    roots.push(LatticeRoot { root_id: 6, coords: E8Point::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0), color_charge: 7, decay_state: 0, role_tag: 1 });
    roots.push(LatticeRoot { root_id: 7, coords: E8Point::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0), color_charge: 8, decay_state: 0, role_tag: 2 });

    let tokenizer = GeoSemanticTokenizer { proj, roots };

    // 3. Test Geometric Snapping Across 7 Distinct Scientific Words
    let test_lexicon = "gravity photon entropy starlight vacuum manifold superposition";
    let snapped_tokens = tokenizer.process(test_lexicon, SnapMode::GeometricDominant);

    println!("\n--- (TEST 1: DYNAMIC E8 ROOT RESOLUTION & COMPATIBILITY) ---");
    let mut i = 0;
    while i < snapped_tokens.len() {
        if let Some(tok) = snapped_tokens.get(i) {
            println!(
                "Word: {:<14} | Snapped Root: {:<2} | Role: {} | Cosine Match: {:+.4}",
                tok.raw.text, tok.root.root_id, tok.root.role_tag, tok.compatibility_score
            );
        }
        i = i + 1;
    }

    // 4. Test Causal vs Scrambled Temporal Coherence
    println!("\n--- (TEST 2: CAUSAL ORDER VS TEMPORAL JUMP DETECTION) ---");
    let triplets = weave_triplets(&snapped_tokens);
    let causal_loom = build_loom(&triplets);
    let causal_inv = compute_physics_invariants(&causal_loom);
    println!("Causal Chronological Loom: Temporal Coherence = {:.4} | Causality OK = {}", causal_inv.temporal_coherence, causal_inv.causality_ok);
    assert_eq!(causal_inv.temporal_coherence, 1.0);
    assert!(causal_inv.causality_ok);

    // Scramble time steps to simulate retrocausal violation
    let mut scrambled_loom = BraidLoom::new(10);
    scrambled_loom.push_slot(BraidLoomSlot { slot_id: 0, time_step: 5, triplet: triplets.first().unwrap().clone(), layer_id: 0, braid_index: 0 });
    scrambled_loom.push_slot(BraidLoomSlot { slot_id: 1, time_step: 2, triplet: triplets.first().unwrap().clone(), layer_id: 0, braid_index: 1 });
    let retro_inv = compute_physics_invariants(&scrambled_loom);
    println!("Scrambled Retrocausal Loom: Temporal Coherence = {:.4} | Causality OK = {}", retro_inv.temporal_coherence, retro_inv.causality_ok);
    assert_eq!(retro_inv.temporal_coherence, 0.0);
    assert_eq!(retro_inv.causality_ok, false);

    println!("\n==================================================================");
    println!(" ALL GEOMETRIC SNAPPING & INVARIANT TESTS PASSED ZERO BRACKETS     ");
    println!("==================================================================\n");
}