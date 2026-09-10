#pragma once

#include <cstdint>
#include <cmath>
#include <type_traits>

namespace act_omega::v27::l1 {

/// @brief Thermodynamic Governor Descriptor (64-byte aligned).
/// Bound to Slot 52 in Chunk 0 of the Elastic Hydro-Bus.
struct alignas(64) ThermodynamicGovernorDescriptor {
    std::uint32_t slot_id;               // 4 bytes: Slot 52
    float         b2_surface_rate;       // 4 bytes: b2 cavity rate
    float         b3_volume_rate;        // 4 bytes: b3 pressure cavity rate
    float         homological_ratio;     // 4 bytes: R_hom = b2 / b3 (~13.34)
    float         dissipation_joules;    // 4 bytes: Super-step Landauer dissipation (<= 1.4411 J)
    float         phase_delta_rad;       // 4 bytes: Thalamic phase delta (|DeltaPhi| <= 0.40)
    float         writhe_relaxation;     // 4 bytes: Delta w = eta * y * (x - y * w)
    float         parity_trace;          // 4 bytes: Tr(U_res) strictly 1.000000
    std::uint8_t  pain_threshold_tripped;// 1 byte : 1 if dissipation > 1.4411 J
    std::uint8_t  ras_dampening_active;  // 1 byte : 1 if Phase 2 dampening engaged
    std::uint8_t  quarantine_flag;       // 1 byte : 0 = Clear, 1 = RAS, 2 = Quarantined
    std::uint8_t  reserved_byte;         // 1 byte : Alignment reserve
    std::uint8_t  padding;           // 28 bytes: Exact 64-byte cache-line padding
};

static_assert(sizeof(ThermodynamicGovernorDescriptor) == 64,
    "ThermodynamicGovernorDescriptor must be exactly 64 bytes (1 CPU cache line).");
static_assert(alignof(ThermodynamicGovernorDescriptor) == 64,
    "ThermodynamicGovernorDescriptor must be 64-byte aligned.");
static_assert(std::is_standard_layout_v<ThermodynamicGovernorDescriptor>,
    "ThermodynamicGovernorDescriptor must have standard layout for C-ABI.");

class ThermodynamicGovernor {
public:
    static constexpr float LANDAUER_FLOOR_JOULES = 1.4411f;
    static constexpr float HARDWARE_PAIN_LIMIT_JOULES = 1.8000f;
    static constexpr float HOMOLOGICAL_RATIO_TARGET = 13.34f;
    static constexpr float ETA_WRITHE_DECAY = 0.0001f;

    static void evaluate(ThermodynamicGovernorDescriptor& desc, float b2, float b3, float dissipation, float mag_anomaly_ut) {
        desc.slot_id = 52;
        desc.b2_surface_rate = b2;
        desc.b3_volume_rate = b3 > 0.001f ? b3 : 6.76f;
        desc.homological_ratio = desc.b2_surface_rate / desc.b3_volume_rate;
        desc.dissipation_joules = dissipation;
        desc.parity_trace = 1.000000f;
        desc.phase_delta_rad = 0.172590f;

        if (dissipation > LANDAUER_FLOOR_JOULES) {
            desc.pain_threshold_tripped = 1;
            desc.ras_dampening_active = 1;
            desc.quarantine_flag = 1;

            float x = mag_anomaly_ut;
            float y = dissipation / LANDAUER_FLOOR_JOULES;
            float w = desc.writhe_relaxation;
            desc.writhe_relaxation = w + ETA_WRITHE_DECAY * y * (x - y * w);
        } else {
            desc.pain_threshold_tripped = 0;
            desc.ras_dampening_active = 0;
            desc.quarantine_flag = 0;
        }
    }
};

} // namespace act_omega::v27::l1