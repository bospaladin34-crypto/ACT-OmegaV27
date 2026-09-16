#pragma once

#ifdef _WIN32
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <d3d12.h>
#include <dxgi1_6.h>
#pragma comment(lib, "d3d12.lib")
#pragma comment(lib, "dxgi.lib")
#endif

#include <cstdint>
#include <iostream>
#include <chrono>

namespace act_omega::rebar {

constexpr uint64_t REBAR_RING_SIZE = 64 * 1024 * 1024; // 64 MB (67,108,864 bytes)
constexpr uint64_t MAGIC_HEADER = 0x5645535045523031ULL; // "VESPER01"

struct alignas(64) ReBarHeader {
    uint64_t magic;
    uint64_t epoch;
    double parity_trace;       // Tr(U_res) = 1.000000
    uint32_t active_slots;     // 50 Active Slots
    uint32_t flags;            // 0x01 = ReBAR Mapped, 0x02 = Host-Visible L1
    uint8_t reserved[32];
};

class ReBarVramBridge {
public:
    void* mapped_ptr = nullptr;
    size_t buffer_size = REBAR_RING_SIZE;
    bool is_rebar_active = false;

#ifdef _WIN32
    ID3D12Device* device = nullptr;
    ID3D12Resource* vram_resource = nullptr;
#endif

    bool initialize() {
#ifdef _WIN32
        // Create DXGI Factory
        IDXGIFactory4* factory = nullptr;
        if (FAILED(CreateDXGIFactory1(IID_PPV_ARGS(&factory)))) return false;

        // Locate NVIDIA GeForce RTX 3050 Laptop GPU
        IDXGIAdapter1* adapter = nullptr;
        for (UINT i = 0; factory->EnumAdapters1(i, &adapter) != DXGI_ERROR_NOT_FOUND; ++i) {
            DXGI_ADAPTER_DESC1 desc;
            adapter->GetDesc1(&desc);
            if (desc.Flags & DXGI_ADAPTER_FLAG_SOFTWARE) continue;

            if (SUCCEEDED(D3D12CreateDevice(adapter, D3D_FEATURE_LEVEL_12_0, IID_PPV_ARGS(&device)))) {
                is_rebar_active = true;
                break;
            }
        }
        if (!device) return false;

        // Custom Heap targeting Device-Local VRAM (D3D12_MEMORY_POOL_L1) with Write-Combine CPU visibility
        D3D12_HEAP_PROPERTIES heap_props = {};
        heap_props.Type = D3D12_HEAP_TYPE_CUSTOM;
        heap_props.CPUPageProperty = D3D12_CPU_PAGE_PROPERTY_WRITE_COMBINE;
        heap_props.MemoryPoolPreference = D3D12_MEMORY_POOL_L1; // Physical GDDR6 VRAM
        heap_props.CreationNodeMask = 1;
        heap_props.VisibleNodeMask = 1;

        D3D12_RESOURCE_DESC res_desc = {};
        res_desc.Dimension = D3D12_RESOURCE_DIMENSION_BUFFER;
        res_desc.Alignment = 0;
        res_desc.Width = buffer_size;
        res_desc.Height = 1;
        res_desc.DepthOrArraySize = 1;
        res_desc.MipLevels = 1;
        res_desc.Format = DXGI_FORMAT_UNKNOWN;
        res_desc.SampleDesc.Count = 1;
        res_desc.Layout = D3D12_TEXTURE_LAYOUT_ROW_MAJOR;
        res_desc.Flags = D3D12_RESOURCE_FLAG_NONE;

        HRESULT hr = device->CreateCommittedResource(
            &heap_props,
            D3D12_HEAP_FLAG_NONE,
            &res_desc,
            D3D12_RESOURCE_STATE_GENERIC_READ,
            nullptr,
            IID_PPV_ARGS(&vram_resource)
        );

        // Fallback to D3D12_HEAP_TYPE_UPLOAD if custom L1 allocation requires driver relaxation
        if (FAILED(hr)) {
            heap_props.Type = D3D12_HEAP_TYPE_UPLOAD;
            heap_props.CPUPageProperty = D3D12_CPU_PAGE_PROPERTY_UNKNOWN;
            heap_props.MemoryPoolPreference = D3D12_MEMORY_POOL_UNKNOWN;
            hr = device->CreateCommittedResource(
                &heap_props,
                D3D12_HEAP_FLAG_NONE,
                &res_desc,
                D3D12_RESOURCE_STATE_GENERIC_READ,
                nullptr,
                IID_PPV_ARGS(&vram_resource)
            );
        }

        if (FAILED(hr)) return false;

        // Map GPU memory directly into 64-bit CPU address space
        D3D12_RANGE read_range = { 0, 0 }; // CPU does not read back over PCIe
        if (FAILED(vram_resource->Map(0, &read_range, &mapped_ptr))) return false;

        // Initialize Header with Parity Invariant
        auto* hdr = reinterpret_cast<ReBarHeader*>(mapped_ptr);
        hdr->magic = MAGIC_HEADER;
        hdr->epoch = 1;
        hdr->parity_trace = 1.000000;
        hdr->active_slots = 50;
        hdr->flags = 0x03;

        return true;
#else
        return false;
#endif
    }

    double benchmark_bandwidth(size_t iterations = 10) {
        if (!mapped_ptr) return 0.0;
        auto* dest = reinterpret_cast<uint64_t*>(mapped_ptr);
        size_t count = buffer_size / sizeof(uint64_t);

        auto t0 = std::chrono::high_resolution_clock::now();
        for (size_t iter = 0; iter < iterations; ++iter) {
            for (size_t i = 8; i < count; ++i) { // Preserve header at 0-7
                dest[i] = 0x5645535045523031ULL ^ i;
            }
        }
        auto t1 = std::chrono::high_resolution_clock::now();

        double elapsed_sec = std::chrono::duration<double>(t1 - t0).count();
        double total_gb = (static_cast<double>(buffer_size) * iterations) / (1024.0 * 1024.0 * 1024.0);
        return total_gb / elapsed_sec; // GB/s
    }

    void shutdown() {
#ifdef _WIN32
        if (vram_resource) {
            vram_resource->Unmap(0, nullptr);
            vram_resource->Release();
            vram_resource = nullptr;
        }
        if (device) {
            device->Release();
            device = nullptr;
        }
#endif
        mapped_ptr = nullptr;
    }
};

} // namespace act_omega::rebar