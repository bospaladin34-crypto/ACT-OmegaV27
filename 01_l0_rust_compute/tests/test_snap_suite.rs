// test_snap_suite.rs - Adversarial snap validation, Revision Omega.2
// Zero Square Bracket Invariant preserved across this entire file.

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

fn project_word(tok: &GeoSemanticTokenizer, w: &str) -> E8Point {
    let raw = RawToken {
        text: w.to_string(),
        span_start: 0usize,
        span_end: 1usize,
    };
    let emb = tok.embed(&raw);
    tok.project(&emb)
}

fn cosine(a: &E8Point, b: &E8Point) -> f32 {
    let denom = a.norm_sq().sqrt() * b.norm_sq().sqrt();
    if denom == 0.0f32 {
        0.0f32
    } else {
        a.dot(b) / denom
    }
}

fn snap_one(tok: &GeoSemanticTokenizer, w: &str) -> u16 {
    let out = tok.process(w, SnapMode::GeometricDominant);
    out.get(0).unwrap().root.root_id
}

fn main() {
    println!("==================================================================");
    println!(" [ACT-OMEGA V27.0]: REVISION OMEGA.2 ADVERSARIAL SNAP SUITE      ");
    println!(" Invariant: N-Gram Cymatics / 240 E_8 Roots / Zero Brackets      ");
    println!("==================================================================");

    // Section 1: The 240-root table is valid
    let roots = build_e8_root_table();
    assert_eq!(roots.len(), 240usize);
    assert!(verify_e8_root_table(&roots));
    println!("SECTION 1 PASS: 240-root table valid (112 vector + 128 spinor roots)");

    let tok = make_tokenizer();

    // Section 2: Determinism across 200 runs
    let text = "the quick brown fox jumps over the lazy dog";
    let first = tok.process(text, SnapMode::GeometricDominant);
    assert_eq!(first.len(), 9usize);
    let mut run = 0u32;
    while run < 200u32 {
        let again = tok.process(text, SnapMode::GeometricDominant);
        assert_eq!(again.len(), first.len());
        let mut k = 0usize;
        while k < first.len() {
            let a = first.get(k).unwrap();
            let b = again.get(k).unwrap();
            assert_eq!(a.root.root_id, b.root.root_id);
            assert!(a.compatibility_score == b.compatibility_score);
            k = k + 1usize;
        }
        run = run + 1u32;
    }
    println!("SECTION 2 PASS: Snap determinism verified over 200 iterations");

    // Section 3: Anagram separation (order drives the phase sequence)
    assert!(snap_one(&tok, "listen") != snap_one(&tok, "silent"));
    assert!(snap_one(&tok, "dusty") != snap_one(&tok, "study"));
    assert!(snap_one(&tok, "night") != snap_one(&tok, "thing"));
    println!("SECTION 3 PASS: Anagram separation verified (n-gram order breaks symmetry)");

    // Section 4: Morphological neighbors nearer than unrelated words
    let p_play = project_word(&tok, "play");
    let p_playing = project_word(&tok, "playing");
    let p_test = project_word(&tok, "test");
    let p_testing = project_word(&tok, "testing");
    let p_quantum = project_word(&tok, "quantum");
    assert!(cosine(&p_play, &p_playing) > cosine(&p_play, &p_quantum));
    assert!(cosine(&p_test, &p_testing) > cosine(&p_test, &p_quantum));
    println!("SECTION 4 PASS: Morphological nearness confirmed (subwords cluster)");

    // Section 5: Collision rate over broad 200+ word vocabulary
    let corpus = "time person year way day thing man world life hand part child eye woman place work week case point company system program question night government number city community name team minute idea kid body information back parent face other level office door health art war history party result change morning reason research girl guy moment air teacher force education foot boy age process music market sense service nation plan college interest death experience effect value video care group mother field fish garden heart hospital hotel image phone photo river road rock salt sand scale scene sea shape share sheep sheet ship shirt shock shoe shop shore short side sign signal singer sister site size skill skin skirt sleep slice slide small smart smell smile smoke snake snow society solar song soul sound south space speak speed spell spend spice spirit split spoke sport staff stage stair stand star start state station stay steam steel steep stick still stock stone stood store storm story stove street strike string strong stuck student study stuff style subject subway sugar suit summer sun super supply sweet swing sword table taken talk tall tank taste teach teeth thank theater thick thief think third though thought thread threat three throat through throw thumb tiger tight tired title today token tooth top total touch tough tower town track trade train travel treat tree trend trial tribe trick truck truly trust truth twice under union unite until upper upset urban usual valid visit voice vote watch water wealth wear weather web weight whale wheat wheel where which while white whole whose wide wife wild will wind window wine wing winter wire wise wish wood word worker worry worth would wound write wrong yard yellow young youth zone";
    let snapped = tok.process(corpus, SnapMode::GeometricDominant);
    let total = snapped.len();
    assert!(total > 150usize);

    let mut ids = Vec::new();
    let mut k = 0usize;
    while k < total {
        let rid = snapped.get(k).unwrap().root.root_id;
        assert!((rid as usize) < 240usize);
        ids.push(rid);
        k = k + 1usize;
    }

    let mut distinct = 0usize;
    let mut a = 0usize;
    while a < total {
        let mut seen = false;
        let mut b = 0usize;
        while b < a {
            if ids.get(b).unwrap() == ids.get(a).unwrap() {
                seen = true;
            }
            b = b + 1usize;
        }
        if !seen {
            distinct = distinct + 1usize;
        }
        a = a + 1usize;
    }
    assert!(distinct * 10usize >= total * 7usize);
    println!("SECTION 5 PASS: Collision rate within bound (>70% distinct E_8 roots)");

    // Section 6: Compatibility score carries signal, not constant
    let colors = tok.process(
        "red blue green yellow orange purple black white gray pink brown",
        SnapMode::GeometricDominant,
    );
    let mut lo = 1000000.0f32;
    let mut hi = -1000000.0f32;
    let mut m = 0usize;
    while m < colors.len() {
        let s = colors.get(m).unwrap().compatibility_score;
        if s < lo { lo = s; }
        if s > hi { hi = s; }
        m = m + 1usize;
    }
    assert!(hi - lo > 0.01f32);
    println!("SECTION 6 PASS: Compatibility score varies with geometric alignment");

    // Section 7: Empty, tiny, and multibyte inputs safe
    let e = tok.process("", SnapMode::GeometricDominant);
    assert_eq!(e.len(), 0usize);
    let one = tok.process("a", SnapMode::GeometricDominant);
    assert_eq!(one.len(), 1usize);
    assert!((one.get(0).unwrap().root.root_id as usize) < 240usize);
    println!("SECTION 7 PASS: Edge inputs safe (empty, tiny strings handled)");

    // Section 8: Both snap modes valid
    let text2 = "braid lattice parity shear";
    let g = tok.process(text2, SnapMode::GeometricDominant);
    let s = tok.process(text2, SnapMode::StructuralDominant);
    assert_eq!(g.len(), s.len());
    let mut n = 0usize;
    while n < g.len() {
        assert!((g.get(n).unwrap().root.root_id as usize) < 240usize);
        assert!((s.get(n).unwrap().root.root_id as usize) < 240usize);
        n = n + 1usize;
    }
    println!("SECTION 8 PASS: Both snap modes valid across all tokens");

    // Section 9: Loom weave over real 240-root table
    let six = tok.process("alpha beta gamma delta epsilon zeta", SnapMode::GeometricDominant);
    assert_eq!(six.len(), 6usize);
    let triplets = weave_triplets(&six);
    assert_eq!(triplets.len(), 2usize);
    let loom = build_loom(&triplets);
    assert_eq!(loom.slots.len(), 2usize);
    let inv = compute_physics_invariants(&loom);
    assert!(inv.causality_ok);
    assert_eq!(inv.temporal_coherence, 1.0f32);
    println!("SECTION 9 PASS: Loom weave over real E_8 table (Temporal Coherence = 1.0)");

    println!("\n==================================================================");
    println!(" ALL SNAP SUITE INVARIANTS VERIFIED (ZERO SQUARE BRACKETS)        ");
    println!("==================================================================\n");
}