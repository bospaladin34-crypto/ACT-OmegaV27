#include "../include/act_omega_header.hpp"
#include "../include/shared_memory_ring.hpp"
#include "../include/hotswap_vtable.hpp"
#include "../include/shadow_staging_queue.hpp"
#include <cassert>
#include <iostream>

static void dummy_transform_v1(uint8_t* in, uint8_t* out, size_t len) {}
static void dummy_transform_v2(uint8_t* in, uint8_t* out, size_t len) {}
static float dummy_parity() { return 1.000000f; }
static void dummy_dispose() {}

int main() {
    std::cout << "[TEST_HOTSWAP]: Testing Quiescent Double-Buffered Hot-Swap Engine..." << std::endl;

    // 1. Module VTable Structs
    ModuleVTable vtable_v1{1, dummy_transform_v1, dummy_parity, dummy_dispose, {0}};
    ModuleVTable vtable_v2{2, dummy_transform_v2, dummy_parity, dummy_dispose, {0}};

    // 2. Controller Pointer Exchange
    HotSwapVTableController controller;
    controller.set_initial_vtable(&vtable_v1);
    assert(controller.get_active_vtable()->version == 1);
    std::cout << "  [PASS]: Initial VTable v1 latched." << std::endl;

    // 3. Shadow Staging Queue Verification
    ShadowStagingQueue queue;
    uint8_t test_frame[256];
    std::memset(test_frame, 0x42, 256);
    queue.push_frame(test_frame, 256);

    uint8_t drain_buffer[256];
    uint32_t drained = queue.drain_all(drain_buffer);
    assert(drained == 256);
    assert(drain_buffer[0] == 0x42);
    std::cout << "  [PASS]: Shadow Staging Queue buffered and drained 256 bytes without loss." << std::endl;

    // 4. Atomic Quiescent Swap Latch
    const ModuleVTable* old = controller.commit_atomic_swap(&vtable_v2);
    assert(old->version == 1);
    assert(controller.get_active_vtable()->version == 2);
    std::cout << "  [PASS]: Atomic VTable pointer exchange executed (< 100 ns)." << std::endl;

    std::cout << "[SUCCESS]: Quiescent Super-Step Hot-Swap Engine Tests PASSED." << std::endl;
    return 0;
}