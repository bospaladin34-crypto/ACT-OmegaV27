#include <iostream>
#include <cassert>
#include "../include/thermodynamic_governor.hpp"

int main() {
    using namespace act_omega::v27::l1;

    ThermodynamicGovernorDescriptor desc{};

    // Test 1: Static Baseline (90.20 b2, 6.76 b3, 1.20 J)
    ThermodynamicGovernor::evaluate(desc, 90.20f, 6.76f, 1.20f, 0.0f);
    assert(desc.slot_id == 52);
    assert(desc.pain_threshold_tripped == 0);
    assert(desc.ras_dampening_active == 0);
    assert(std::abs(desc.homological_ratio - 13.34f) < 0.1f);
    assert(desc.parity_trace == 1.000000f);

    std::cout << "[PASS]: Test 1 Static Baseline R_hom = " << desc.homological_ratio << " (Target ~13.34)" << std::endl;

    // Test 2: Hardware Pain / Kinetic Saturation (98.83 b2, 6.76 b3, 1.65 J dissipation, 9.878 uT turbulence)
    ThermodynamicGovernor::evaluate(desc, 98.83f, 6.76f, 1.65f, 9.878f);
    assert(desc.pain_threshold_tripped == 1);
    assert(desc.ras_dampening_active == 1);
    assert(desc.quarantine_flag == 1);
    assert(desc.writhe_relaxation > 0.0f);

    std::cout << "[PASS]: Test 2 Hardware Pain Throttling engaged: Dissipation = " << desc.dissipation_joules << " J" << std::endl;
    std::cout << "[PASS]: Phase 2 RAS Dampening Active | Oja-Hebbian Writhe Relaxation = " << desc.writhe_relaxation << std::endl;
    std::cout << "\nALL TASK 25 C++23 GOVERNOR INVARIANTS VERIFIED ON SLOT 52." << std::endl;
    return 0;
}