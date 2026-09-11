// test_knot_splicer.rs - Task 32 Verification Suite
// Zero Square Brackets enforced across all lines

include!("../src/geo_semantic_tokenizer.rs");
include!("../src/braid_attention.rs");
include!("../src/knot_splicer.rs");

fn main() {
    println!("==================================================================");
    println!(" [ACT-OMEGA V27.0]: TASK 32 KNOT SPLICER & SHEAF REPAIR SUITE     ");
    println!(" Invariant: Yang-Baxter Surgery & Obstruction Collapse (H^1 = 0)  ");
    println!("==================================================================");

    let splicer = KnotSurgeryEngine::default();
    let kernel = BraidAttentionKernel::default();

    // Test 1: Obstruction Detection
    let has_obs = splicer.detect_obstruction(0.42);
    let clean_obs = splicer.detect_obstruction(0.000001);
    assert!(has_obs);
    assert!(!clean_obs);
    println!("\n--- [TEST 1: SHEAF OBSTRUCTION DETECTION] ---");
    println!("Discrepancy 0.420 -> Obstruction Flagged: {}", has_obs);
    println!("Discrepancy 0.000 -> Obstruction Clean  : {}", !clean_obs);

    // Test 2: Yang-Baxter Surgery: sigma_1 * sigma_2 * sigma_1 -> sigma_2 * sigma_1 * sigma_2
    let mut yb_input = Vec::new();
    yb_input.push(BraidGenerator { strand_index: 1, is_inverse: false });
    yb_input.push(BraidGenerator { strand_index: 2, is_inverse: false });
    yb_input.push(BraidGenerator { strand_index: 1, is_inverse: false });

    println!("\n--- [TEST 2: YANG-BAXTER SURGICAL TRANSFORMATION] ---");
    println!("Pre-surgery braid generators: {}", yb_input.len());
    let (yb_output, moves) = splicer.apply_yang_baxter_surgery(&yb_input);
    println!("Yang-Baxter surgery moves applied: {}", moves);
    println!("Post-surgery braid generators   : {}", yb_output.len());
    assert_eq!(moves, 1);
    assert_eq!(yb_output.len(), 3);
    assert_eq!(yb_output.get(0).unwrap().strand_index, 2);
    assert_eq!(yb_output.get(1).unwrap().strand_index, 1);
    assert_eq!(yb_output.get(2).unwrap().strand_index, 2);

    // Test 3: Complex Knot Splicing & Reidemeister II Collapse (H^1 -> 0)
    println!("\n--- [TEST 3: FULL KNOT SPLICING & REIDEMEISTER COLLAPSE] ---");
    let mut entangled = Vec::new();
    entangled.push(BraidGenerator { strand_index: 1, is_inverse: false });
    entangled.push(BraidGenerator { strand_index: 2, is_inverse: false });
    entangled.push(BraidGenerator { strand_index: 1, is_inverse: false });
    entangled.push(BraidGenerator { strand_index: 2, is_inverse: true });

    let initial_discrepancy = 0.85f32;
    let (_repaired_braid, report) = splicer.repair_sheaf_obstruction(&entangled, &kernel, initial_discrepancy);

    println!("Initial Obstruction Discrepancy : {:.4}", report.initial_obstruction);
    println!("Yang-Baxter Moves Executed      : {}", report.yang_baxter_moves);
    println!("Reidemeister II Loop Collapses  : {}", report.loop_collapses);
    println!("Final Cohomology Obstruction    : {:.4} (Target: 0.0000)", report.final_obstruction);
    println!("Majorana Parity Lock            : {:.6} [CONSERVED]", report.parity_trace);
    println!("Sheaf Repair Status             : {}", report.is_repaired);

    assert!(report.is_repaired);
    assert_eq!(report.final_obstruction, 0.0);
    assert_eq!(report.parity_trace, 1.000000);
    assert!(report.loop_collapses >= 1);

    println!("\n==================================================================");
    println!(" ALL TASK 32 KNOT SPLICER & SHEAF REPAIR TESTS PASSED ZERO BRACKETS");
    println!("==================================================================\n");
}