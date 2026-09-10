// Comprehensive 5-Tier Canonical Vault & Entropic Time Test Suite
// Zero Square Bracket Invariant strictly enforced across this file

use act_omega_tensor_vault::get_canonical_tensor_by_id;

fn main() {
    // 1. Tier 1 Test: 3 Generations of Fermions
    let top_quark = get_canonical_tensor_by_id("T1_Q_T_L_G3").unwrap();
    assert_eq!(top_quark.tier_level, 1);
    assert_eq!(top_quark.topological_charge_q, 2);
    assert_eq!(top_quark.writhe, 6);
    assert_eq!(top_quark.parity_lock, 1.000000);

    let tau_lepton = get_canonical_tensor_by_id("T1_L_TAU_L_G3").unwrap();
    assert_eq!(tau_lepton.writhe, -6);

    // 2. Tier 2 Test: Category C Morphisms
    let qcd_emission = get_canonical_tensor_by_id("T2_QCD_EMISSION").unwrap();
    assert_eq!(qcd_emission.tier_level, 2);
    assert_eq!(qcd_emission.braid_word, "q_i -> q_j + g");

    // 3. Tier 3 Test: Santos Rotation Tensor (108 deg)
    let santos_rot = get_canonical_tensor_by_id("T3_SANTOS_ROTATION_108").unwrap();
    assert_eq!(santos_rot.tier_level, 3);
    assert_eq!(santos_rot.topological_charge_q, 1);

    // 4. Tier 4 Test: Entropic Time Emergence (July 2026 Literature & Missoula Grounding)
    let static_comp = get_canonical_tensor_by_id("T4_ENTROPIC_STATIC_COMPENSATOR").unwrap();
    assert_eq!(static_comp.tier_level, 4);
    assert_eq!(static_comp.braid_word, "b2_static_88.99_records_per_sec");

    let kinetic_mart = get_canonical_tensor_by_id("T4_ENTROPIC_KINETIC_MARTINGALE").unwrap();
    assert_eq!(kinetic_mart.topological_charge_q, 1);

    let grav_zero = get_canonical_tensor_by_id("T4_GRAVITY_CALIBRATED_ZERO").unwrap();
    assert_eq!(grav_zero.landauer_joules, 0.0000);

    let heartbeat_q = get_canonical_tensor_by_id("T4_HEARTBEAT_RECORD_QUANTUM").unwrap();
    assert_eq!(heartbeat_q.braid_word, "6_b2_records_per_62.636ms");

    // 5. Tier 5 Test: 7777D Polytope Acceptance Window
    let poly7777 = get_canonical_tensor_by_id("T5_POLYTOPE_7777D_WINDOW").unwrap();
    assert_eq!(poly7777.tier_level, 5);
    assert_eq!(poly7777.writhe, 48);

    println!("All 5-Tier Canonical Vault & Entropic Time Invariant Tests PASSED.");
}