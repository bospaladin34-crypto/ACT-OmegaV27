#include "../include/rebar_vram_bridge.hpp"
#include <iostream>
#include <iomanip>
#include <cassert>

int main() {
    std::cout << "==================================================================" << std::endl;
    std::cout << "[ACT-OMEGA V27.0]: TASK 53 RESIZABLE BAR 64MB VRAM BRIDGE AUDIT" << std::endl;
    std::cout << "==================================================================" << std::endl;

    act_omega::rebar::ReBarVramBridge bridge;
    std::cout << "Allocating 64 MB (67,108,864 bytes) Host-Visible VRAM Buffer..." << std::endl;

    if (!bridge.initialize()) {
        std::cerr << "[FAIL] Failed to allocate Direct3D 12 Host-Visible ReBAR VRAM buffer." << std::endl;
        return 1;
    }

    std::cout << "  - Status            : ReBAR VRAM Ring Active" << std::endl;
    std::cout << "  - Mapped CPU Pointer: " << bridge.mapped_ptr << std::endl;

    auto* hdr = reinterpret_cast<act_omega::rebar::ReBarHeader*>(bridge.mapped_ptr);
    std::cout << "  - Magic Signature   : 0x" << std::hex << hdr->magic << std::dec << std::endl;
    std::cout << "  - Parity Conservation: Tr(U_res) = " << std::fixed << std::setprecision(6) << hdr->parity_trace << std::endl;
    assert(hdr->magic == 0x5645535045523031ULL);
    assert(hdr->parity_trace == 1.000000);

    std::cout << "\nBenchmarking Sequential Write Bandwidth over PCIe 4.0 x8..." << std::endl;
    double throughput = bridge.benchmark_bandwidth(20);
    std::cout << "  - Measured Bandwidth: " << std::fixed << std::setprecision(2) << throughput << " GB/s" << std::endl;

    bridge.shutdown();
    std::cout << "[PASS] Task 53 Resizable BAR 64MB VRAM Direct Mapping Verified." << std::endl;
    return 0;
}