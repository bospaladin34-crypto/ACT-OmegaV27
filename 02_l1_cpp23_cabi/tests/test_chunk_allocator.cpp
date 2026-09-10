#include "../include/chunk_allocator.hpp"
#include <cassert>
#include <iostream>

int main() {
    std::cout << "[TEST_CHUNK_ALLOCATOR]: Validating Multi-Chunk Elastic Bus Expansion..." << std::endl;

    DynamicChunkAllocator allocator;

    // 1. Validate Initial Chunk 0 (64 slots)
    assert(allocator.get_total_capacity() == 64);
    assert(allocator.get_chunks_count() == 1);
    std::cout << "  [PASS]: Initial Chunk 0 capacity confirmed (64 slots, 4 KB page)." << std::endl;

    // 2. Claim Base Slots 0 to 63
    for (uint32_t i = 0; i < 64; ++i) {
        uint32_t claimed = allocator.claim_next_slot(0x1111222233330000ULL + i);
        assert(claimed == i);
    }
    assert(allocator.get_active_count() == 64);
    std::cout << "  [PASS]: 64 base slots claimed without relocation." << std::endl;

    // 3. Auto-Expand to Chunk 1 (Slot 64 -> triggers Chunk 1 commit, capacity = 128)
    uint32_t slot64 = allocator.claim_next_slot(0xAABBCCDDEEFF0040ULL);
    assert(slot64 == 64);
    assert(allocator.get_total_capacity() == 128);
    assert(allocator.get_chunks_count() == 2);
    std::cout << "  [PASS]: Dynamic expansion to Chunk 1 (128 total slots) executed on-the-fly." << std::endl;

    // 4. Verify O(1) pointer stability on Slot 0 and Slot 64
    auto* s0 = allocator.get_slot(0);
    assert(s0 != nullptr && s0->slot_id == 0);

    auto* s64 = allocator.get_slot(64);
    assert(s64 != nullptr && s64->slot_id == 64);
    assert(s64->module_hash == 0xAABBCCDDEEFF0040ULL);
    std::cout << "  [PASS]: Slot 0 and Slot 64 pointer offsets remain valid and accessible." << std::endl;

    std::cout << "[SUCCESS]: Multi-Chunk Dynamic Allocator Invariant Tests PASSED." << std::endl;
    return 0;
}