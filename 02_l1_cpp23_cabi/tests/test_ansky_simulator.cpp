#include "../include/ansky_simulator.hpp"
#include <iostream>
#include <cassert>

using namespace act_omega::cabi;

int main() {
    std::cout << "==================================================================" << std::endl;
    std::cout << " [ACT-OMEGA V27.0]: TASK 48 ANSKY MACRO-STICTION AUDIT           " << std::endl;
    std::cout << " 108-Hour QPE Limit Cycle / Santos 108° / Reidemeister II Burst   " << std::endl;
    std::cout << "==================================================================" << std::endl;

    CppAnskySimulator sim;
    assert(sim.get_recurrence_hours() == 108.0);
    assert(sim.get_decadic_scale() == 0.100000);

    // Test 1: Mid-Orbit Quiescence (Hour 54.0 - Phase 0.50)
    AnskyOrbitDescriptor mid = sim.step_epoch(54.0);
    assert(mid.is_burst_active == 0);
    assert(mid.b2_betti_number == 1);
    assert(mid.parity_trace == 1.000000);
    std::cout << "STEP 1 PASS: Mid-Orbit Accretion (Hour 54.0 | Shear: " << mid.metric_shear_joules << " J | b2: 1 | Parity: 1.000000)" << std::endl;

    // Test 2: Pre-Burst Stiction Accumulation (Hour 105.0 - Phase ~0.97)
    AnskyOrbitDescriptor pre = sim.step_epoch(105.0);
    assert(pre.is_burst_active == 0);
    assert(pre.metric_shear_joules > 13.5); // Nearing 14.411 J
    std::cout << "STEP 2 PASS: Stiction Stored (Hour 105.0 | Shear: " << pre.metric_shear_joules << " J / 14.411 J)" << std::endl;

    // Test 3: Macro Reidemeister Type II Cord-Snap (Hour 107.5 - Phase > 0.98)
    AnskyOrbitDescriptor burst = sim.step_epoch(107.5);
    assert(burst.is_burst_active == 1);
    assert(burst.b2_betti_number == 0); // Loop collapse
    assert(burst.metric_shear_joules == 0.0); // Reset to vacuum ground
    assert(burst.parity_trace == 1.000000);
    std::cout << "STEP 3 PASS: Reidemeister II Collapse Triggered (Burst Active: 1 | b2: 0 | Parity Locked)" << std::endl;

    std::cout << "\n==================================================================" << std::endl;
    std::cout << " ALL TASK 48 ANSKY SIMULATOR INVARIANTS VERIFIED SUCCESSFULLY     " << std::endl;
    std::cout << "==================================================================\n" << std::endl;
    return 0;
}