// test_natural_language.rs - Conversational Natural Language Audit (Revision Omega.2)
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

fn evaluate_natural_prompt(tok: &GeoSemanticTokenizer, prompt_type: &str, prompt: &str) {
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
                "    Word: {:<12} | Root: {:<3} | Role: {:<8} | Color: C{} | Compat: {:+.4}",
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
    println!(" ACT-OMEGA v27.0 CONVERSATIONAL NATURAL LANGUAGE AUDIT            ");
    println!(" Evaluating Everyday User Prompts over 240 E_8 Lattice Roots     ");
    println!("==================================================================");

    let tok = make_tokenizer();

    // Prompt 1: Weather / Daily Life
    evaluate_natural_prompt(
        &tok,
        "WEATHER & DAILY LIFE",
        "what is the weather like outside today"
    );

    // Prompt 2: How-To / Everyday Routine
    evaluate_natural_prompt(
        &tok,
        "PROCEDURAL HOW-TO",
        "how do I make a good cup of coffee"
    );

    // Prompt 3: Philosophical / Time
    evaluate_natural_prompt(
        &tok,
        "PHILOSOPHICAL INQUIRY",
        "why does time only move forward and not backward"
    );

    // Prompt 4: Coding / Software Task
    evaluate_natural_prompt(
        &tok,
        "PROGRAMMING / CODING PROMPT",
        "write a fast function to sort a list of numbers"
    );

    // Prompt 5: Creative / Narrative
    evaluate_natural_prompt(
        &tok,
        "CREATIVE WRITING / STORYTELLING",
        "tell me a short story about a wandering traveler in the mountains"
    );

    println!("\n==================================================================");
    println!(" ALL NATURAL LANGUAGE AUDITS COMPLETED (ZERO SQUARE BRACKETS)     ");
    println!("==================================================================\n");
}