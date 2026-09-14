#pragma once
#include <cstdint>
#include <atomic>

namespace act_omega::cabi {

struct alignas(64) ModuleVTable {
    uint64_t vtable_id;
    uint64_t epoch_bound;
    float (*compute_kernel)(const float* in, float* out, uint32_t len);
    float (*verify_parity)();
    float (*get_metric_drag)();
    uint64_t reserved_0;
    uint64_t reserved_1;
    uint64_t reserved_2;
};

static_assert(sizeof(ModuleVTable) == 64, "ModuleVTable must be exactly 64 bytes");

class QuiescentHotSwapRegistry {
private:
    alignas(64) std::atomic<const ModuleVTable*> active_vtable_{nullptr};
    alignas(64) std::atomic<uint64_t> current_epoch_{0};
    alignas(64) std::atomic<uint64_t> swap_count_{0};

public:
    explicit QuiescentHotSwapRegistry(const ModuleVTable* initial_vtable) noexcept {
        active_vtable_.store(initial_vtable, std::memory_order_release);
    }

    [[nodiscard]] inline const ModuleVTable* acquire_vtable() const noexcept {
        return active_vtable_.load(std::memory_order_acquire);
    }

    bool swap_vtable(const ModuleVTable* new_vtable, uint64_t target_epoch) noexcept {
        if (!new_vtable) return false;

        if (new_vtable->verify_parity) {
            float p = new_vtable->verify_parity();
            float diff = p - 1.000000f;
            if (diff < 0.0f) diff = -diff;
            if (diff > 1e-6f) {
                return false;
            }
        }

        active_vtable_.store(new_vtable, std::memory_order_release);
        swap_count_.fetch_add(1, std::memory_order_relaxed);
        current_epoch_.store(target_epoch, std::memory_order_release);
        return true;
    }

    [[nodiscard]] inline uint64_t get_swap_count() const noexcept {
        return swap_count_.load(std::memory_order_relaxed);
    }

    [[nodiscard]] inline uint64_t get_current_epoch() const noexcept {
        return current_epoch_.load(std::memory_order_acquire);
    }
};

} // namespace act_omega::cabi