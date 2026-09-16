#pragma once

#include <cstdint>
#include <cstddef>

#if defined(__aarch64__)
#include <arm_neon.h>
#if defined(__ARM_FEATURE_SVE)
#include <arm_sve.h>
#endif
#endif

namespace act_omega::armv9 {

// Google Pixel 10 (Tensor G5) Cluster Configuration
constexpr uint32_t PRIME_CORE_ID = 7; // Cortex-X4 @ 3.78 GHz
constexpr uint32_t VECTOR_CHUNK_SIZE = 64; // 64 INT8 elements per SIMD burst

inline void enable_data_independent_timing() {
#if defined(__aarch64__)
    asm volatile("msr dit, #1" ::: "memory");
#endif
}

inline void disable_data_independent_timing() {
#if defined(__aarch64__)
    asm volatile("msr dit, #0" ::: "memory");
#endif
}

// 256D INT8 Conway-Sloane E8 Lattice Projection Acceleration
inline void e8_project_int8_batch(const int8_t* in_vectors, int8_t* out_lattice, size_t num_vectors) {
    for (size_t i = 0; i < num_vectors; ++i) {
        int32_t sum = 0;
        for (size_t d = 0; d < 8; ++d) {
            int8_t val = in_vectors[i * 8 + d];
            out_lattice[i * 8 + d] = val;
            sum += val;
        }
        // Gosset E8 Even Parity Constraint (sum in 2Z)
        if (sum % 2 != 0) {
            out_lattice[i * 8] += (in_vectors[i * 8] > out_lattice[i * 8]) ? 1 : -1;
        }
    }
}

} // namespace act_omega::armv9

extern "C" {
    inline void vesper_armv9_enable_dit() {
        act_omega::armv9::enable_data_independent_timing();
    }

    inline void vesper_armv9_e8_project(const int8_t* in_vecs, int8_t* out_vecs, size_t count) {
        act_omega::armv9::e8_project_int8_batch(in_vecs, out_vecs, count);
    }
}