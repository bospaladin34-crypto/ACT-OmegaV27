#pragma once
#include "act_omega_header.hpp"
#include <cstdint>
#include <atomic>

#define SLOTS_PER_CHUNK 64
#define MAX_CHUNKS 16
#define MAX_TOTAL_SLOTS (SLOTS_PER_CHUNK * MAX_CHUNKS) // 1024 Slots Max

struct alignas(4096) ChunkDescriptorPage {
    ChannelSlotDescriptor slots[SLOTS_PER_CHUNK];
};

static_assert(sizeof(ChunkDescriptorPage) == 4096, "ChunkDescriptorPage must match 4 KB OS page");

class DynamicChunkAllocator {
private:
    std::atomic<uint32_t> total_committed_slots{SLOTS_PER_CHUNK};
    std::atomic<uint32_t> active_registered_slots{0};
    std::atomic<uint32_t> committed_chunks{1};
    ChunkDescriptorPage* chunk_pages[MAX_CHUNKS]{nullptr};

public:
    DynamicChunkAllocator() {
        // Initialize base Chunk 0 (Slots 0..63)
        chunk_pages[0] = new ChunkDescriptorPage();
        std::memset(chunk_pages[0], 0, sizeof(ChunkDescriptorPage));
    }

    ~DynamicChunkAllocator() {
        uint32_t count = committed_chunks.load(std::memory_order_relaxed);
        for (uint32_t i = 0; i < count && i < MAX_CHUNKS; ++i) {
            if (chunk_pages[i]) {
                delete chunk_pages[i];
                chunk_pages[i] = nullptr;
            }
        }
    }

    // Dynamic inductive expansion (+64 slots via 4 KB page commit)
    bool expand_chunk() {
        uint32_t current_chunks = committed_chunks.load(std::memory_order_relaxed);
        if (current_chunks >= MAX_CHUNKS) return false;

        auto* new_page = new ChunkDescriptorPage();
        std::memset(new_page, 0, sizeof(ChunkDescriptorPage));

        chunk_pages[current_chunks] = new_page;
        committed_chunks.fetch_add(1, std::memory_order_release);
        total_committed_slots.fetch_add(SLOTS_PER_CHUNK, std::memory_order_release);
        return true;
    }

    // Atomic O(1) slot claim with auto-expansion
    uint32_t claim_next_slot(uint64_t module_hash) {
        uint32_t slot_id = active_registered_slots.fetch_add(1, std::memory_order_seq_cst);
        
        while (slot_id >= total_committed_slots.load(std::memory_order_acquire)) {
            expand_chunk();
        }

        uint32_t chunk_idx = slot_id >> 6;       // slot_id / 64
        uint32_t offset = slot_id & 0x3F;         // slot_id % 64

        auto* slot = &chunk_pages[chunk_idx]->slots[offset];
        slot->slot_id = slot_id;
        slot->status_flags = 0x01; // Active
        slot->module_hash = module_hash;
        slot->allocated_bytes = 65536;
        slot->landauer_budget = 1.4411f;
        slot->phase_delta = 0.17259029f;

        return slot_id;
    }

    ChannelSlotDescriptor* get_slot(uint32_t slot_id) {
        if (slot_id >= total_committed_slots.load(std::memory_order_acquire)) return nullptr;
        uint32_t chunk_idx = slot_id >> 6;
        uint32_t offset = slot_id & 0x3F;
        return &chunk_pages[chunk_idx]->slots[offset];
    }

    uint32_t get_total_capacity() const { return total_committed_slots.load(std::memory_order_relaxed); }
    uint32_t get_active_count() const { return active_registered_slots.load(std::memory_order_relaxed); }
    uint32_t get_chunks_count() const { return committed_chunks.load(std::memory_order_relaxed); }
};