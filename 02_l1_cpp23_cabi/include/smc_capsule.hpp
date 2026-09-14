#pragma once
// smc_capsule.hpp - ACT-Omega v27.0 Domain 3 Sovereign Manifold Capsule Specification
#include <cstdint>
#include <array>

namespace act_omega::capsule {

struct alignas(64) SmcHeader {
    uint64_t magic;              // 0x534D435041434B30 ("SMCPACK0")
    uint32_t version;            // 27
    uint32_t target_arch;        // 3 = Dual-Silicon (x86_64 + aarch64)
    uint64_t total_capsule_size; // Total capsule size in bytes
    uint64_t vault_offset;        // Byte offset to verified_scientific_vault section
    uint64_t vault_size;          // Byte size of vault section
    uint64_t binary_offset;       // Byte offset to native binary payload section
    uint64_t binary_size;         // Byte size of native binary payload section
    std::array<uint8_t, 8> reserved; // 8 bytes padding (Sum = 64 bytes)
};

static_assert(sizeof(SmcHeader) == 64, "SmcHeader must be exactly 64 bytes");

struct alignas(64) SmcTrailer {
    uint64_t trailer_magic;      // 0x5645535045523031 ("VESPER01")
    double parity_trace;         // 1.000000 (Majorana-1 Parity Lock)
    uint32_t chech_h1;           // 0
    uint32_t reserved_flags;
    std::array<char, 40> sha256_seal;
};

static_assert(sizeof(SmcTrailer) == 64, "SmcTrailer must be exactly 64 bytes");

} // namespace act_omega::capsule