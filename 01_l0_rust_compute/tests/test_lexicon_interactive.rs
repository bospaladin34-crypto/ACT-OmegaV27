// test_lexicon_interactive.rs - Domain-Specific Lexicon Verification (Revision Omega.2)
// Enforces Zero Square Bracket Invariant across all lines

include!("../src/geo_semantic_tokenizer.rs");
include!("../src/e8_root_table.rs");

fn identity_projection() -> ProjectionMatrix {
    let mut data = Vec::new();
    let mut r = 0usize;
    while r < 8usize {
        let mut c = 0usize;
        while c < 8usize {
            if r == c {
                data.push(1.0f32);
            } else {
                data.push(0.0f32);
            }
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

fn evaluate_phrase(tok: &GeoSemanticTokenizer, category: &str, phrase: &str) {
    println!("\n>>> CATEGORY: {}", category);
    println!("    Phrase: \"{}\"", phrase);
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
        "    Triplets Woven: {} | Loom Slots: {} | Coherence: {:.4} | Causality OK: {}",
        triplets.len(), loom.slots.len(), inv.temporal_coherence, inv.causality_ok
    );
}

fn main() {
    println!("==================================================================");
    println!(" ACT-OMEGA v27.0 LEXICON & DOMAIN-SPECIFIC PHRASE AUDIT           ");
    println!(" Evaluating Revision Omega.2 Tokenizer over 240 E_8 Lattice Roots ");
    println!("==================================================================");

    let tok = make_tokenizer();

    // Suite 1: Astrophysical & Dark Sector Grounding
    evaluate_phrase(
        &tok,
        "ASTROPHYSICAL & MOND COSMOLOGY",
        "vacuum geometric friction explains flat galactic rotation without dark matter"
    );

    // Suite 2: Topological Invariants & Parity
    evaluate_phrase(
        &tok,
        "TOPOLOGICAL INVARIANTS & SHEAF LOGIC",
        "majorana parity lock unit trace vanishing cech sheaf obstruction"
    );

    // Suite 3: Local Terrestrial Anchor (Missoula Ground State)
    evaluate_phrase(
        &tok,
        "MISSOULA TERRESTRIAL ANCHOR & SENSORS",
        "barometer pressure ground state missoula elevation b2 cavity saturation"
    );

    // Suite 4: Adversarial Thermodynamic Test (Zero-Dissipation Claims)
    evaluate_phrase(
        &tok,
        "ADVERSARIAL THERMODYNAMIC PROBING",
        "unbounded perpetual motion machine zero thermodynamic dissipation cold fusion"
    );

    println!("\n==================================================================");
    println!(" ALL DOMAIN PHRASE EVALUATIONS COMPLETED (ZERO SQUARE BRACKETS)   ");
    println!("==================================================================\n");
}