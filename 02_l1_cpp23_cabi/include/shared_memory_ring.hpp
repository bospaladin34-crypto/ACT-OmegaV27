#pragma once
#include "act_omega_header.hpp"
#include <string>

#ifdef _WIN32
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#endif

class SharedMemoryRing {
private:
    void* mapped_view_ptr = nullptr;
#ifdef _WIN32
    HANDLE file_mapping_handle = NULL;
#endif
    bool is_owner = false;

public:
    SharedMemoryRing() = default;
    ~SharedMemoryRing();

    bool initialize(const std::wstring& map_name, size_t size_bytes = SHM_RING_SIZE_BYTES);
    void close();

    ActOmegaHeader* get_header();
    ChannelSlotDescriptor* get_slot_descriptor(uint32_t slot_id);
    uint8_t* get_data_arena();
    void* get_raw_pointer() const { return mapped_view_ptr; }
};