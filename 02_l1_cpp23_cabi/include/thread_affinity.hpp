#pragma once

#ifdef _WIN32
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <mmsystem.h>
#pragma comment(lib, "winmm.lib")
#endif

#include <cstdint>
#include <chrono>

namespace act_omega::hardware {

// Intel Core i5-12450HX Heterogeneous Topology:
// Total: 8 Cores / 12 Logical Processors (Threads 0-11)
// - Golden Cove P-Cores: Cores 0-3 (Hyper-Threaded -> Threads 0-7, Mask: 0x00FF)
// - Gracemont E-Cores:  Cores 4-7 (Single-Threaded -> Threads 8-11, Mask: 0x0F00)
constexpr uint64_t PCORE_MASK = 0x00FF;
constexpr uint64_t ECORE_MASK = 0x0F00;
constexpr uint64_t ALL_CORES_MASK = 0x0FFF;

inline bool set_pcore_affinity() {
#ifdef _WIN32
    HANDLE hThread = GetCurrentThread();
    DWORD_PTR prev = SetThreadAffinityMask(hThread, static_cast<DWORD_PTR>(PCORE_MASK));
    if (prev == 0) return false;
    SetThreadPriority(hThread, THREAD_PRIORITY_TIME_CRITICAL);
    return true;
#else
    return true;
#endif
}

inline bool set_ecore_affinity() {
#ifdef _WIN32
    HANDLE hThread = GetCurrentThread();
    DWORD_PTR prev = SetThreadAffinityMask(hThread, static_cast<DWORD_PTR>(ECORE_MASK));
    if (prev == 0) return false;
    SetThreadPriority(hThread, THREAD_PRIORITY_BELOW_NORMAL);
    return true;
#else
    return true;
#endif
}

inline void enable_precision_timer() {
#ifdef _WIN32
    timeBeginPeriod(1);
#endif
}

inline void disable_precision_timer() {
#ifdef _WIN32
    timeEndPeriod(1);
#endif
}

} // namespace act_omega::hardware

extern "C" {
    inline int32_t vesper_set_pcore_affinity() {
        return act_omega::hardware::set_pcore_affinity() ? 0 : -1;
    }

    inline int32_t vesper_set_ecore_affinity() {
        return act_omega::hardware::set_ecore_affinity() ? 0 : -1;
    }

    inline void vesper_enable_precision_timer() {
        act_omega::hardware::enable_precision_timer();
    }

    inline void vesper_disable_precision_timer() {
        act_omega::hardware::disable_precision_timer();
    }
}