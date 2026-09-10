// test_tokenizer_loom.rs - Validation for Revision Omega.1 Tokenizer & Braid Loom
// Enforces Zero Square Bracket Invariant across all lines

include!("../src/geo_semantic_tokenizer.rs");

fn main() {
    let mut m_data = Vec::new();
    let mut i = 0;
    while i < 32 {
        m_data.push(1.0);
        i = i + 1;
    }
    let proj = ProjectionMatrix {
        rows: 8,
        cols: 4,
        data: m_data,
    };

    let mut roots = Vec::new();
    roots.push(LatticeRoot {
        root_id: 0,
        coords: E8Point::new(1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        color_charge: 1,
        decay_state: 0,
        role_tag: 1,
    });

    let tokenizer = GeoSemanticTokenizer { proj, roots };
    let input_text = "Alice likes coffee";
    let snapped = tokenizer.process(input_text, SnapMode::GeometricDominant);
    assert_eq!(snapped.len(), 3);

    let triplets = weave_triplets(&snapped);
    assert_eq!(triplets.len(), 1);

    let loom = build_loom(&triplets);
    assert_eq!(loom.slots.len(), 1);

    let inv = compute_physics_invariants(&loom);
    assert!(inv.causality_ok);
    assert_eq!(inv.temporal_coherence, 1.0);

    let mut templates = Vec::new();
    templates.push(PhraseTemplate {
        name: "svo".to_string(),
        pattern: "{A} {B} {C}".to_string(),
        role_a: 1,
        role_b: 1,
        role_c: 1,
    });

    let matched = match_template(triplets.first().unwrap(), &templates);
    assert!(matched.is_some());

    let rendered = render_phrase(triplets.first().unwrap(), templates.first().unwrap());
    assert_eq!(rendered, "Alice likes coffee");

    println!("ALL GEOMETRIC-SEMANTIC TOKENIZER & BRAID LOOM INVARIANTS VERIFIED (ZERO BRACKETS)");
}