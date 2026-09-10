// Native Rust Integration Test Suite for Real-World Ingestion Streams
// Zero Square Bracket Invariant strictly enforced across this file

use vesper_ffi::ingest_streams::{
    ingest_cern_mass_resonance,
    ingest_materials_twistronic_angle,
    ingest_planck_floquet_harmonic
};

fn main() {
    // 1. Verify CERN Higgs Invariant Mass Ingestion (125.25 GeV)
    let cern_event = ingest_cern_mass_resonance(125.25, 990142);
    assert!(cern_event.is_valid);
    assert_eq!(cern_event.stream_source, "CERN_LHC_RUN3");
    assert!((cern_event.parity_trace - 1.000000).abs() < 1e-6);

    // 2. Verify Materials Project 108 deg Twistronic Angle Ingestion
    let vdw_structure = ingest_materials_twistronic_angle(108.0, 772911);
    assert!(vdw_structure.is_valid);
    assert_eq!(vdw_structure.stream_source, "MATERIALS_PROJECT_VDW");
    assert_eq!(vdw_structure.topological_charge, 1);

    // 3. Verify Planck CMB 15.965 Hz Floquet Frequency Calibration
    let cmb_mode = ingest_planck_floquet_harmonic(15.965, 104);
    assert!(cmb_mode.is_valid);
    assert_eq!(cmb_mode.stream_source, "PLANCK_CMB_FLOQUET");

    println!("All Real-World Scientific Ingestion Invariant Tests PASSED.");
}