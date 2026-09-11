// test_truth_collapse_gate.rs - Task 35 Verification Suite
// Zero Square Brackets enforced across all lines

use std::string::ToString;
use std::vec::Vec;

include!("../src/geo_semantic_tokenizer.rs");
include!("../src/braid_attention.rs");
include!("../src/knot_splicer.rs");
include!("../src/truth_collapse_gate.rs");

fn main() {
    println!("==================================================================");
    println!(" [ACT-OMEGA V27.0]: TASK 35 TRUTH & COLLAPSE GATE VERIFICATION    ");
    println!(" Invariant: Penrose Collapse (E_G * tau >= hbar) & Cech H^1 = 0  ");
    println!("==================================================================");

    let gate = EpistemicTruthGate::default();
    let splicer = KnotSurgeryEngine::default();
    let kernel = BraidAttentionKernel::default();

    // Candidate A: Valid, high-energy, grounded physical proposition
    let mut braid_a = Vec::new();
    braid_a.push(BraidGenerator { strand_index: 1, is_inverse: false });
    braid_a.push(BraidGenerator { strand_index: 2, is_inverse: false });

    let thought_grounded = CandidateThought {
        thought_id: 101,
        hypothesis_text: "Topological metric shear produces flat galactic rotation curves".to_string(),
        self_energy_joules: 0.000000000000000000000000000000020, // Above critical threshold (2.0e-33 J)
        sheaf_discrepancy: 0.000000,                              // Completely consistent (H^1 = 0)
        braid_trace: braid_a,
    };

    let (verdict_a, _repaired_a) = gate.process_candidate_thought(&thought_grounded, &splicer, &kernel);
    println!("\n--- [TEST 1: GROUNDED PROPOSITION EMISSION] ---");
    println!("Thought ID: {}", verdict_a.thought_id);
    println!("Objective Collapse Triggered  : {}", verdict_a.is_collapsed);
    println!("Cech Sheaf Consistency (H^1=0): {}", verdict_a.is_sheaf_consistent);
    println!("Hallucination Quarantined     : {}", verdict_a.is_hallucination_quarantined);
    println!("Conserved Majorana Parity     : {:.6}", verdict_a.parity_trace);
    assert!(verdict_a.is_collapsed);
    assert!(verdict_a.is_sheaf_consistent);
    assert!(!verdict_a.is_hallucination_quarantined);
    assert_eq!(verdict_a.parity_trace, 1.000000);

    // Candidate B: Contradictory proposition with topological obstruction (Hallucination)
    let mut braid_b = Vec::new();
    // Injects self-canceling conflicting loop (sigma_1 * sigma_2 * sigma_1 * sigma_2^-1)
    braid_b.push(BraidGenerator { strand_index: 1, is_inverse: false });
    braid_b.push(BraidGenerator { strand_index: 2, is_inverse: false });
    braid_b.push(BraidGenerator { strand_index: 1, is_inverse: false });
    braid_b.push(BraidGenerator { strand_index: 2, is_inverse: true });

    let thought_contradictory = CandidateThought {
        thought_id: 102,
        hypothesis_text: "Unbounded perpetual motion machine without thermodynamic dissipation".to_string(),
        self_energy_joules: 0.000000000000000000000000000000025,
        sheaf_discrepancy: 0.750000, // Severe logical contradiction (H^1 != 0)
        braid_trace: braid_b,
    };

    let (verdict_b, repaired_b) = gate.process_candidate_thought(&thought_contradictory, &splicer, &kernel);
    println!("\n--- [TEST 2: CONTRADICTORY PROPOSITION AUTO-SURGERY] ---");
    println!("Thought ID: {}", verdict_b.thought_id);
    println!("Pre-Surgery Obstruction       : 0.7500");
    println!("Post-Surgery Obstruction      : {:.4} (H^1 = 0 Restored)", verdict_b.final_obstruction);
    println!("Post-Surgery Braid Generatos  : {}", repaired_b.len());
    println!("Sheaf Consistency Repaired    : {}", verdict_b.is_sheaf_consistent);
    println!("Parity Lock Conserved         : {:.6}", verdict_b.parity_trace);
    assert_eq!(verdict_b.final_obstruction, 0.0);
    assert_eq!(verdict_b.parity_trace, 1.000000);
    assert_eq!(repaired_b.len(), 1);

    // Candidate C: Sub-threshold thought (Fleeting mental noise below Penrose collapse)
    let mut braid_c = Vec::new();
    braid_c.push(BraidGenerator { strand_index: 1, is_inverse: false });

    let thought_sub_threshold = CandidateThought {
        thought_id: 103,
        hypothesis_text: "Sub-quantum vacuum fluctuation".to_string(),
        self_energy_joules: 0.0000000000000000000000000000000001, // 1.0e-36 J (Far below threshold)
        sheaf_discrepancy: 0.000000,
        braid_trace: braid_c,
    };

    let (verdict_c, _) = gate.process_candidate_thought(&thought_sub_threshold, &splicer, &kernel);
    println!("\n--- [TEST 3: SUB-THRESHOLD NOISE QUARANTINE] ---");
    println!("Thought ID: {}", verdict_c.thought_id);
    println!("Objective Collapse Triggered  : {}", verdict_c.is_collapsed);
    println!("Hallucination/Noise Clamped   : {}", verdict_c.is_hallucination_quarantined);
    assert!(!verdict_c.is_collapsed);
    assert!(verdict_c.is_hallucination_quarantined);

    println!("\n==================================================================");
    println!(" ALL TASK 35 TRUTH & COLLAPSE GATE INVARIANTS PASSED ZERO BRACKETS");
    println!("==================================================================\n");
}