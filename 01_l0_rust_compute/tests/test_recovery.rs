// Native Rust Integration Test Suite for Proprioceptive Cortex & 4-Phase Recovery
// Zero Square Bracket Invariant strictly enforced across this file

use vesper_ffi::proprioceptive_cortex::{
    ProprioceptiveCortex,
    RecoveryStatePhase,
    MISSOULA_MECHANICAL_GROUND_HZ,
    LOCAL_GRAVITATIONAL_ANCHOR
};

fn main() {
    let mut cortex = ProprioceptiveCortex::new();

    // 1. Verify Grounding Anchors
    assert_eq!(MISSOULA_MECHANICAL_GROUND_HZ, 36.0);
    assert!((LOCAL_GRAVITATIONAL_ANCHOR - 9.80665).abs() < 1e-5);

    // 2. Verify Bilateral Writhe Bound (|w_L - w_R| <= 0.02)
    assert!(cortex.verify_bilateral_writhe(0.010, 0.015));
    assert!(!cortex.verify_bilateral_writhe(0.000, 0.035));

    // 3. Verify Oja-Hebbian Writhe Decay
    let decayed = cortex.apply_oja_writhe_decay(1.0, 0.5, 0.5, 0.0001);
    assert!(decayed < 1.0001);

    // 4. Verify 4-Phase State Recovery Transitions
    let p1 = cortex.evaluate_recovery_cycle(0.25);
    match p1 {
        RecoveryStatePhase::Phase1StableFlow => (),
        _ => panic!("Expected Phase 1"),
    }

    let p2 = cortex.evaluate_recovery_cycle(0.55);
    match p2 {
        RecoveryStatePhase::Phase2RasDampening => (),
        _ => panic!("Expected Phase 2"),
    }

    let p3 = cortex.evaluate_recovery_cycle(0.75);
    match p3 {
        RecoveryStatePhase::Phase3StomachionQuarantine => (),
        _ => panic!("Expected Phase 3"),
    }

    let p4 = cortex.evaluate_recovery_cycle(0.95);
    match p4 {
        RecoveryStatePhase::Phase4TopologicalReboot => (),
        _ => panic!("Expected Phase 4"),
    }
    assert_eq!(cortex.parity_trace, 1.000000);

    println!("All Proprioceptive Cortex & 4-Phase Recovery Tests PASSED.");
}