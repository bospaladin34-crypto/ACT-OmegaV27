#include "../include/shared_memory_ring.hpp"
#include "../include/simd_kernels.hpp"
#include "../include/hotswap_vtable.hpp"
#include "../include/shadow_staging_queue.hpp"
#include "../include/chunk_allocator.hpp"

static SharedMemoryRing g_shm_ring;
static HotSwapVTableController g_hotswap_slots[64];
static ShadowStagingQueue g_shadow_queues[64];
static DynamicChunkAllocator g_chunk_allocator;

#ifdef _WIN32
#define EXPORT_C extern "C" __declspec(dllexport)
#else
#define EXPORT_C extern "C"
#endif

EXPORT_C uint64_t vesper_create() {
    if (!g_shm_ring.get_raw_pointer()) {
        g_shm_ring.initialize(L"Global\\ACT_OMEGA_E8_HYPER_MANIFOLD");
    }
    return ACT_OMEGA_MAGIC;
}

EXPORT_C float vesper_verify_parity() {
    auto* header = g_shm_ring.get_header();
    if (header && header->magic == ACT_OMEGA_MAGIC) {
        return header->parity_trace;
    }
    return 1.000000f;
}

EXPORT_C uint8_t* vesper_transform(uint8_t* input, size_t len, uint64_t handle) {
    return input;
}

EXPORT_C uint32_t vesper_compute_distance_avx2(const uint8_t* vec_a, const uint8_t* vec_b) {
    return compute_rabitq_hamming_distance_avx2(vec_a, vec_b);
}

EXPORT_C uint32_t vesper_hotswap_engage_bypass(uint32_t slot_id) {
    if (slot_id >= 64) return 0;
    auto* slot = g_shm_ring.get_slot_descriptor(slot_id);
    if (slot) {
        slot->status_flags = 0x02;
        g_shadow_queues[slot_id].reset();
        return 1;
    }
    return 0;
}

EXPORT_C uint32_t vesper_hotswap_commit_latch(uint32_t slot_id, const ModuleVTable* new_vtable) {
    if (slot_id >= 64 || !new_vtable) return 0;
    g_hotswap_slots[slot_id].commit_atomic_swap(new_vtable);
    auto* slot = g_shm_ring.get_slot_descriptor(slot_id);
    if (slot) {
        slot->status_flags = 0x01;
        return 1;
    }
    return 0;
}

EXPORT_C uint32_t vesper_hotswap_abort_rollback(uint32_t slot_id) {
    if (slot_id >= 64) return 0;
    auto* slot = g_shm_ring.get_slot_descriptor(slot_id);
    if (slot) {
        slot->status_flags = 0x01;
        g_shadow_queues[slot_id].reset();
        return 1;
    }
    return 0;
}

// Multi-Chunk Elastic Allocator APIs
EXPORT_C uint32_t vesper_chunk_claim_slot(uint64_t module_hash) {
    return g_chunk_allocator.claim_next_slot(module_hash);
}

EXPORT_C uint32_t vesper_chunk_get_capacity() {
    return g_chunk_allocator.get_total_capacity();
}

EXPORT_C uint32_t vesper_chunk_get_active_count() {
    return g_chunk_allocator.get_active_count();
}

EXPORT_C void vesper_free(uint64_t handle) {
    g_shm_ring.close();
}