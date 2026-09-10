#pragma once
#include <cstdint>
#include <atomic>

typedef void (*ModuleTransformFn)(uint8_t* in_ptr, uint8_t* out_ptr, size_t len);
typedef float (*ModuleVerifyParityFn)();
typedef void (*ModuleDisposeFn)();

// 64-byte aligned VTable descriptor
struct alignas(64) ModuleVTable {
    uint32_t version;
    ModuleTransformFn transform;
    ModuleVerifyParityFn verify_parity;
    ModuleDisposeFn dispose;
    uint8_t reserved[32];
};

class HotSwapVTableController {
private:
    std::atomic<const ModuleVTable*> active_vtable;

public:
    HotSwapVTableController() : active_vtable(nullptr) {}

    void set_initial_vtable(const ModuleVTable* vtable) {
        active_vtable.store(vtable, std::memory_order_release);
    }

    const ModuleVTable* get_active_vtable() const {
        return active_vtable.load(std::memory_order_acquire);
    }

    // Atomic latch at quiescent super-step boundary (< 100 ns)
    const ModuleVTable* commit_atomic_swap(const ModuleVTable* new_vtable) {
        return active_vtable.exchange(new_vtable, std::memory_order_acq_rel);
    }
};