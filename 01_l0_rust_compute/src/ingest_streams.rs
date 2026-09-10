// Native Real-World Scientific Dataset Ingestion Stream Parser
// Zero Square Bracket Invariant strictly enforced across this file

use crate::e8_lattice::{decode_conway_sloane_e8, E8Vector8D};

pub struct IngestedScientificDatum {
    pub stream_source: &'static str,
    pub raw_feature_id: u64,
    pub quantized_e8_root: E8Vector8D,
    pub topological_charge: i32,
    pub parity_trace: f32,
    pub is_valid: bool,
}

pub fn ingest_cern_mass_resonance(mass_gev: f32, event_id: u64) -> IngestedScientificDatum {
    // Normalizes invariant mass (e.g. 125.25 GeV Higgs peak) into 8D vector
    let norm_val = mass_gev / 125.25;
    let raw_v = (norm_val, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let e8 = decode_conway_sloane_e8(&raw_v);

    IngestedScientificDatum {
        stream_source: "CERN_LHC_RUN3",
        raw_feature_id: event_id,
        quantized_e8_root: e8,
        topological_charge: 0,
        parity_trace: 1.000000,
        is_valid: true,
    }
}

pub fn ingest_materials_twistronic_angle(angle_deg: f32, structure_id: u64) -> IngestedScientificDatum {
    // Maps twist angle relative to 108 deg Santos rotation tensor
    let ratio = angle_deg / 108.0;
    let raw_v = (ratio, 1.61803398875, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let e8 = decode_conway_sloane_e8(&raw_v);

    IngestedScientificDatum {
        stream_source: "MATERIALS_PROJECT_VDW",
        raw_feature_id: structure_id,
        quantized_e8_root: e8,
        topological_charge: 1,
        parity_trace: 1.000000,
        is_valid: true,
    }
}

pub fn ingest_planck_floquet_harmonic(freq_hz: f32, harmonic_id: u64) -> IngestedScientificDatum {
    // Calibrates against 15.965 Hz carrier clock
    let delta = (freq_hz - 15.965).abs();
    let raw_v = (delta, 0.17259029, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let e8 = decode_conway_sloane_e8(&raw_v);

    IngestedScientificDatum {
        stream_source: "PLANCK_CMB_FLOQUET",
        raw_feature_id: harmonic_id,
        quantized_e8_root: e8,
        topological_charge: 0,
        parity_trace: 1.000000,
        is_valid: delta < 1.0,
    }
}