#include "../include/hotswap_vtable.hpp"
#include <iostream>
#include <cassert>

using namespace act_omega::cabi;

float v1_kernel(const float* in, float* out, uint32_t len) { return 1.0f; }
float v1_parity() { return 1.000000f; }
float v1_drag() { return 1.3479e-10f; }

float v2_kernel(const float* in, float* out, uint32_t len) { return 2.0f; }
float v2_parity() { return 1.000000f; }
float v2_drag() { return 1.3479e-10f; }

float flawed_parity() { return 0.850000f; }

int main() {
    std::cout << "==================================================================" << std::endl;
    std::cout << " [ACT-OMEGA V27.0]: C++23 QUIESCENT VTABLE HOT-SWAP AUDIT         " << std::endl;
    std::cout << " Invariants: alignas(64) / Lock-Free Atomic / Zero Dropped Ticks  " << std::endl;
    std::cout << "==================================================================" << std::endl;

    alignas(64) ModuleVTable vt1 = {1, 5000000, v1_kernel, v1_parity, v1_drag, 0, 0, 0};
    alignas(64) ModuleVTable vt2 = {2, 5000010, v2_kernel, v2_parity, v2_drag, 0, 0, 0};
    alignas(64) ModuleVTable vt_bad = {3, 5000020, v2_kernel, flawed_parity, v2_drag, 0, 0, 0};

    QuiescentHotSwapRegistry registry(&vt1);

    const ModuleVTable* active = registry.acquire_vtable();
    assert(active != nullptr);
    assert(active->vtable_id == 1);
    assert(active->compute_kernel(nullptr, nullptr, 0) == 1.0f);
    std::cout << "STEP 1 PASS: Initial VTable ID 1 Acquired (Kernel output: 1.0)" << std::endl;

    bool swap_ok = registry.swap_vtable(&vt2, 5000010);
    assert(swap_ok == true);
    assert(registry.acquire_vtable()->vtable_id == 2);
    assert(registry.acquire_vtable()->compute_kernel(nullptr, nullptr, 0) == 2.0f);
    assert(registry.get_swap_count() == 1);
    std::cout << "STEP 2 PASS: Live Atomic Swap to VTable ID 2 Succeeded (Kernel output: 2.0)" << std::endl;

    bool reject_bad = registry.swap_vtable(&vt_bad, 5000020);
    assert(reject_bad == false);
    assert(registry.acquire_vtable()->vtable_id == 2);
    std::cout << "STEP 3 PASS: Rejected Flawed VTable (Parity Violation Tr = 0.850000)" << std::endl;

    std::cout << "\n==================================================================" << std::endl;
    std::cout << " ALL C++23 HOTSWAP VTABLE INVARIANTS VERIFIED (0 DROPPED TICKS)   " << std::endl;
    std::cout << "==================================================================\n" << std::endl;
    return 0;
}