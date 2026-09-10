#include "../include/shared_memory_ring.hpp"
#include <iostream>

SharedMemoryRing::~SharedMemoryRing() {
    close();
}

bool SharedMemoryRing::initialize(const std::wstring& map_name, size_t size_bytes) {
#ifdef _WIN32
    // Create or open the named file mapping in system pagefile
    file_mapping_handle = CreateFileMappingW(
        INVALID_HANDLE_VALUE,
        NULL,
        PAGE_READWRITE,
        0,
        static_cast<DWORD>(size_bytes),
        map_name.c_str()
    );

    if (!file_mapping_handle) {
        return false;
    }

    is_owner = (GetLastError() != ERROR_ALREADY_EXISTS);

    mapped_view_ptr = MapViewOfFile(
        file_mapping_handle,
        FILE_MAP_ALL_ACCESS,
        0,
        0,
        size_bytes
    );

    if (!mapped_view_ptr) {
        CloseHandle(file_mapping_handle);
        file_mapping_handle = NULL;
        return false;
    }

    // Initialize header if owner
    if (is_owner) {
        auto* header = static_cast<ActOmegaHeader*>(mapped_view_ptr);
        header->magic = ACT_OMEGA_MAGIC;
        header->heartbeat_epoch = 0;
        header->parity_trace = 1.000000f;
        header->landauer_joules = 0.0000f;
        header->bilateral_writhe_diff = 0.0000f;
        header->active_slots = 0;
        header->total_capacity_slots = 64;
    }

    return true;
#else
    return false;
#endif
}

void SharedMemoryRing::close() {
#ifdef _WIN32
    if (mapped_view_ptr) {
        UnmapViewOfFile(mapped_view_ptr);
        mapped_view_ptr = nullptr;
    }
    if (file_mapping_handle) {
        CloseHandle(file_mapping_handle);
        file_mapping_handle = NULL;
    }
#endif
}

ActOmegaHeader* SharedMemoryRing::get_header() {
    return static_cast<ActOmegaHeader*>(mapped_view_ptr);
}

ChannelSlotDescriptor* SharedMemoryRing::get_slot_descriptor(uint32_t slot_id) {
    if (!mapped_view_ptr || slot_id >= 64) return nullptr;
    // Slot descriptors follow the 64-byte ActOmegaHeader
    auto* base = reinterpret_cast<uint8_t*>(mapped_view_ptr) + sizeof(ActOmegaHeader);
    return reinterpret_cast<ChannelSlotDescriptor*>(base + (slot_id * sizeof(ChannelSlotDescriptor)));
}

uint8_t* SharedMemoryRing::get_data_arena() {
    if (!mapped_view_ptr) return nullptr;
    // Data arena starts at offset 65536 (64 KB)
    return reinterpret_cast<uint8_t*>(mapped_view_ptr) + 65536;
}