// test_hybrid_encoding_suite.rs - Tri-Modal Linguistic Audit (Revision Omega.2)
// Enforces Zero Square Bracket Invariant across all lines

include!("../src/geo_semantic_tokenizer.rs");
include!("../src/e8_root_table.rs");

fn identity_projection() -> ProjectionMatrix {
    let mut data = Vec::new();
    let mut r = 0usize;
    while r < 8usize {
        let mut c = 0usize;
        while c < 8usize {
            if r == c { data.push(1.0f32); } else { data.push(0.0f32); }
            c = c + 1usize;
        }
        r = r + 1usize;
    }
    ProjectionMatrix { rows: 8usize, cols: 8usize, data }
}

fn make_tokenizer() -> GeoSemanticTokenizer {
    GeoSemanticTokenizer {
        proj: identity_projection(),
        roots: build_e8_root_table(),
    }
}

fn evaluate_tri_modal_phrase(tok: &GeoSemanticTokenizer, category: &str, phrase: &str) {
    println!("\n>>> MODALITY: {}", category);
    println!("    Phrase  : \"{}\"", phrase);
    println!("------------------------------------------------------------------");

    let snapped = tok.process(phrase, SnapMode::GeometricDominant);
    let mut i = 0usize;
    while i < snapped.len() {
        if let Some(t) = snapped.get(i) {
            let role_name = match t.root.role_tag {
                1 => "Entity",
                2 => "Operator",
                3 => "State",
                _ => "Unknown",
            };
            println!(
                "    Word: {:<16} | Root: {:<3} | Role: {:<8} | Color: C{} | Compat: {:+.4}",
                t.raw.text, t.root.root_id, role_name, t.root.color_charge, t.compatibility_score
            );
        }
        i = i + 1usize;
    }

    let triplets = weave_triplets(&snapped);
    let loom = build_loom(&triplets);
    let inv = compute_physics_invariants(&loom);
    println!("------------------------------------------------------------------");
    println!(
        "    Words: {} | Triplets: {} | Loom Slots: {} | Coherence: {:.4} | Causality OK: {}",
        snapped.len(), triplets.len(), loom.slots.len(), inv.temporal_coherence, inv.causality_ok
    );
}

fn main() {
    println!("==================================================================");
    println!(" ACT-OMEGA v27.0 TRI-MODAL TOKENIZER & LOOM VALIDATION           ");
    println!(" Testing Clean Syntax, Geometric-Semantic, and Mixed Hybrids     ");
    println!("==================================================================");

    let tok = make_tokenizer();

    // Group 1: Proper, Clean, Structured Natural Language (9 tokens = 3 complete triads)
    evaluate_tri_modal_phrase(
        &tok,
        "CLEAN NATURAL: RIVER ECOSYSTEM",
        "the quiet river flows through the ancient mountain valley"
    );
    evaluate_tri_modal_phrase(
        &tok,
        "CLEAN NATURAL: ASTRONOMICAL DAWN",
        "light illuminates the distant horizon with warm gentle color"
    );

    // Group 2: Pure Geometric-Semantic Encoded Phrasing (9 tokens = 3 complete triads)
    evaluate_tri_modal_phrase(
        &tok,
        "GEOMETRIC-SEMANTIC: TOPOLOGICAL SHEAR",
        "topological metric shear creates discrete e8 lattice root reflections"
    );
    evaluate_tri_modal_phrase(
        &tok,
        "GEOMETRIC-SEMANTIC: BRAID PARITY",
        "artin braid operators preserve majorana parity across continuous manifold charts"
    );

    // Group 3: Hybrid Mixed (Slang / Casual + Structured + Geometric-Semantic Encoding, 12 tokens = 4 triads)
    evaluate_tri_modal_phrase(
        &tok,
        "HYBRID MIXED: COLLOQUIAL + GEOMETRIC",
        "honestly dude the topological metric shear is totally bending the spacetime lattice"
    );
    evaluate_tri_modal_phrase(
        &tok,
        "HYBRID MIXED: SUBJECTIVE BELIEF + TOPOLOGY",
        "I believe this chaotic turbulence settles when e8 braid parity locks down"
    );

    println!("\n==================================================================");
    println!(" TRI-MODAL LINGUISTIC AUDIT COMPLETE (ZERO SQUARE BRACKETS)       ");
    println!("==================================================================\n");
}