// Native Rust Integration Test Suite for SASSIFI Background Dream Engine
// Zero Square Bracket Invariant strictly enforced across this file

use vesper_ffi::dream_engine::{SassifiDreamWorker, InoculationFaultType};

fn main() {
    let mut worker = SassifiDreamWorker::new();

    // 1. Test Inoculation: High Phase Turbulence (0.85 rad)
    let report1 = worker.execute_shadow_dream_cycle(InoculationFaultType::PhaseTurbulenceBurst, 105501);
    assert!(report1.is_healed);
    assert!(report1.super_steps_to_heal <= 3);
    assert_eq!(report1.final_parity_trace, 1.000000);

    // 2. Test Inoculation: Parity Drift (0.95 rad reboot trigger)
    let report2 = worker.execute_shadow_dream_cycle(InoculationFaultType::MajoranaParityDrift, 105502);
    assert!(report2.is_healed);
    assert!(report2.super_steps_to_heal <= 3);
    assert_eq!(report2.final_parity_trace, 1.000000);

    // 3. Test Inoculation: Bit-Flip Perturbation
    let report3 = worker.execute_shadow_dream_cycle(InoculationFaultType::BitFlipPerturbation, 105503);
    assert!(report3.is_healed);
    assert!(report3.super_steps_to_heal <= 2);

    assert_eq!(worker.total_dream_cycles, 3);
    assert_eq!(worker.successful_recoveries, 3);

    println!("All Autonomous SASSIFI Background Dream Engine Tests PASSED.");
}