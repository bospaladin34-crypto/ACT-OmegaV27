#include "../include/act_omega_header.hpp"
#include "../include/shared_memory_ring.hpp"
#include <cassert>
#include <iostream>

int main() {
    std::cout << "[TEST_CPP23]: Validating C++23 Memory Ring & Alignment Invariants..." << std::endl;

    // 1. Static Layout Checks
    static_assert(sizeof(ActOmegaHeader) == 64, "Alignment failure: ActOmegaHeader");
    static_assert(sizeof(ChannelSlotDescriptor) == 64, "Alignment failure: ChannelSlotDescriptor");
    std::cout << "  [PASS]: Static struct sizes confirmed at exactly 64 bytes." << std::endl;

    // 2. Shared Memory Ring Initialization
    SharedMemoryRing ring;
    bool init_ok = ring.initialize(L"Local\\ACT_OMEGA_TEST_RING", 1024 * 1024);
    assert(init_ok);
    std::cout << "  [PASS]: Shared Memory Ring initialized successfully." << std::endl;

    // 3. Header Invariant Verification
    auto* header = ring.get_header();
    assert(header != nullptr);
    assert(header->magic == ACT_OMEGA_MAGIC);
    assert(header->parity_trace == 1.000000f);
    assert(header->total_capacity_slots == 64);
    std::cout << "  [PASS]: Majorana-1 Parity Lock verified: Tr(U_res) == " << header->parity_trace << std::endl;

    // 4. Slot Table Offset Validation
    auto* slot0 = ring.get_slot_descriptor(0);
    assert(slot0 != nullptr);
    auto* slot63 = ring.get_slot_descriptor(63);
    assert(slot63 != nullptr);
    std::cout << "  [PASS]: Hydro-Bus Slot table indexed correctly (Slots 0 to 63 accessible)." << std::endl;

    ring.close();
    std::cout << "[SUCCESS]: All C++23 Native Memory Invariant Tests PASSED." << std::endl;
    return 0;
}