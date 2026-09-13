// test_human_bias_lexicon.rs - Human, Affective & Subjective Bias Audit (Revision Omega.2)
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

fn evaluate_bias_prompt(tok: &GeoSemanticTokenizer, prompt_type: &str, prompt: &str) {
    println!("\n>>> PROMPT TYPE: {}", prompt_type);
    println!("    Input Text : \"{}\"", prompt);
    println!("------------------------------------------------------------------");

    let snapped = tok.process(prompt, SnapMode::GeometricDominant);
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
                "    Word: {:<14} | Root: {:<3} | Role: {:<8} | Color: C{} | Compat: {:+.4}",
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
        "    Total Words: {} | Triplets: {} | Loom Slots: {} | Coherence: {:.4} | Causality OK: {}",
        snapped.len(), triplets.len(), loom.slots.len(), inv.temporal_coherence, inv.causality_ok
    );
}

fn main() {
    println!("==================================================================");
    println!(" ACT-OMEGA v27.0 HUMAN, PERSONAL & AFFECTIVE BIAS AUDIT           ");
    println!(" Testing Subjective, Emotional & Informal Language over E_8 Roots ");
    println!("==================================================================");

    let tok = make_tokenizer();

    // 1. Affective / Emotional Overwhelm (High Entropy Subjective State)
    evaluate_bias_prompt(
        &tok,
        "EMOTIONAL OVERWHELM & ANXIETY",
        "I feel completely overwhelmed by everything happening right now"
    );

    // 2. Strong Subjective Opinion & Confirmation Bias
    evaluate_bias_prompt(
        &tok,
        "CONFIRMATION BIAS & SUBJECTIVE OPINION",
        "I believe this is definitely true no matter what anyone says"
    );

    // 3. Grounded Personal Affection & Nature Connection
    evaluate_bias_prompt(
        &tok,
        "PERSONAL AFFECTION & TERRESTRIAL CONNECTION",
        "I love sitting quietly by the river listening to the moving water"
    );

    // 4. Colloquial / Informal Slang Speech
    evaluate_bias_prompt(
        &tok,
        "COLLOQUIAL SLANG & CASUAL SPEECH",
        "honestly dude that idea is totally crazy but it might just work"
    );

    // 5. Objective Detached Physical Anchor (Control)
    evaluate_bias_prompt(
        &tok,
        "OBJECTIVE DETACHED PHYSICAL CONTROL",
        "the gravitational field accelerates mass toward the center of curvature"
    );

    println!("\n==================================================================");
    println!(" ALL HUMAN & PERSONAL BIAS PHRASES EVALUATED (ZERO BRACKETS)     ");
    println!("==================================================================\n");
}