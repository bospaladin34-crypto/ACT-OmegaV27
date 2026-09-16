#include "../include/thread_affinity.hpp"
#include <iostream>
#include <cassert>

int main() {
    std::cout << "[ACT-OMEGA V27.0]: Testing Task 52 P-Core / E-Core Affinity..." << std::endl;

    act_omega::hardware::enable_precision_timer();

    bool pcore_ok = act_omega::hardware::set_pcore_affinity();
    std::cout << "  - P-Core Affinity (Mask 0x00FF): " << (pcore_ok ? "OK" : "FAILED") << std::endl;
    assert(pcore_ok);

    bool ecore_ok = act_omega::hardware::set_ecore_affinity();
    std::cout << "  - E-Core Affinity (Mask 0x0F00): " << (ecore_ok ? "OK" : "FAILED") << std::endl;
    assert(ecore_ok);

    // Restore to P-Cores for carrier execution
    act_omega::hardware::set_pcore_affinity();

    act_omega::hardware::disable_precision_timer();
    std::cout << "[PASS] Task 52 C++23 Thread Affinity Verified." << std::endl;
    return 0;
}