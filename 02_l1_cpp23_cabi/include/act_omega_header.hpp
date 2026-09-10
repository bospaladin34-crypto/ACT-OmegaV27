#pragma once
#include <cstdint>
#include <type_traits>
#include <atomic>

// Valid 64-bit Hex Constant (16 hex digits)
#define ACT_OMEGA_MAGIC 0xAC7000E802700000ULL
#define SHM_RING_SIZE_BYTES (64 * 1024 * 1024) // 64 MB

// 64-byte cache-line aligned master manifold header
struct alignas(64) ActOmegaHeader {
    uint64_t magic;                 // 8 bytes (Offset 0..7)
    uint64_t heartbeat_epoch;       // 8 bytes (Offset 8..15)
    float    parity_trace;          // 4 bytes (Offset 16..19)
    float    landauer_joules;       // 4 bytes (Offset 20..23)
    float    bilateral_writhe_diff; // 4 bytes (Offset 24..27)
    uint32_t active_slots;          // 4 bytes (Offset 28..31)
    uint32_t total_capacity_slots;  // 4 bytes (Offset 32..35)
    uint8_t  reserved[28];          // 28 bytes (Offset 36..63) -> Total: 64 bytes
};

static_assert(sizeof(ActOmegaHeader) == 64, "ActOmegaHeader must be exactly 64 bytes");
static_assert(alignof(ActOmegaHeader) == 64, "ActOmegaHeader must be 64-byte cache aligned");

// 64-byte Hydro-Bus Slot Descriptor
struct alignas(64) ChannelSlotDescriptor {
    uint32_t slot_id;               // 4 bytes (Offset 0..3)
    uint32_t status_flags;          // 4 bytes (Offset 4..7)
    uint64_t module_hash;           // 8 bytes (Offset 8..15)
    uint64_t ring_buffer_offset;    // 8 bytes (Offset 16..23)
    uint32_t allocated_bytes;       // 4 bytes (Offset 24..27)
    uint32_t fill_level;            // 4 bytes (Offset 28..31)
    float    landauer_budget;       // 4 bytes (Offset 32..35)
    float    phase_delta;           // 4 bytes (Offset 36..39)
    uint8_t  reserved[24];          // 24 bytes (Offset 40..63) -> Total: 64 bytes
};

static_assert(sizeof(ChannelSlotDescriptor) == 64, "ChannelSlotDescriptor must be exactly 64 bytes");
static_assert(alignof(ChannelSlotDescriptor) == 64, "ChannelSlotDescriptor must be 64-byte cache aligned");