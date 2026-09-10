#pragma once
#include <cstdint>
#include <atomic>
#include <cstring>

#define SHADOW_QUEUE_CAPACITY_BYTES 65536 // 64 KB

struct ShadowStagingQueue {
    uint8_t  buffer[SHADOW_QUEUE_CAPACITY_BYTES];
    std::atomic<uint32_t> write_head{0};
    std::atomic<uint32_t> read_tail{0};

    void push_frame(const uint8_t* data, size_t len) {
        if (len == 0 || len > SHADOW_QUEUE_CAPACITY_BYTES) return;
        uint32_t current_head = write_head.load(std::memory_order_relaxed);
        uint32_t next_head = (current_head + len) % SHADOW_QUEUE_CAPACITY_BYTES;
        std::memcpy(buffer + current_head, data, len);
        write_head.store(next_head, std::memory_order_release);
    }

    uint32_t drain_all(uint8_t* dest_buffer) {
        uint32_t head = write_head.load(std::memory_order_acquire);
        uint32_t tail = read_tail.load(std::memory_order_relaxed);
        if (head == tail) return 0;

        uint32_t count = (head >= tail) ? (head - tail) : (SHADOW_QUEUE_CAPACITY_BYTES - tail + head);
        std::memcpy(dest_buffer, buffer + tail, count);
        read_tail.store(head, std::memory_order_release);
        return count;
    }

    void reset() {
        write_head.store(0, std::memory_order_relaxed);
        read_tail.store(0, std::memory_order_relaxed);
    }
};