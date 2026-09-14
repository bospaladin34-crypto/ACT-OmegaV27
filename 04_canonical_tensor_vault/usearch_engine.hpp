#pragma once
#include <cstdint>
#include <array>
#include <atomic>

namespace act_omega::vault {

struct alignas(64) VectorRecord {
    uint64_t vector_id;
    uint32_t root_id;
    float compatibility;
    std::array<uint8_t, 8> rabitq_bits;
    std::array<uint8_t, 40> reserved;
};

static_assert(sizeof(VectorRecord) == 64, "VectorRecord must fit single 64-byte cache line");

class MMapUsearchEngine {
private:
    uint32_t capacity_;
    std::atomic<uint32_t> count_{0};

public:
    explicit MMapUsearchEngine(uint32_t capacity) noexcept : capacity_(capacity) {}

    [[nodiscard]] inline float query_hamming(const uint8_t* q_bits, const uint8_t* target_bits) const noexcept {
        uint32_t dist = 0;
        for (int i = 0; i < 8; ++i) {
            uint8_t diff = q_bits[i] ^ target_bits[i];
            while (diff) {
                dist += diff & 1;
                diff >>= 1;
            }
        }
        return 1.0f - (static_cast<float>(dist) / 64.0f);
    }

    [[nodiscard]] inline uint32_t get_capacity() const noexcept { return capacity_; }
    [[nodiscard]] inline uint32_t get_count() const noexcept { return count_.load(std::memory_order_relaxed); }
};

} // namespace act_omega::vault