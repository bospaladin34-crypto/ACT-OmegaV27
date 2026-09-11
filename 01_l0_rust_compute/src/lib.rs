// ACT-Omega v27.0 L0 Compute Core Root
// Zero Square Bracket Invariant strictly enforced across this file

pub mod braid_engine;
pub mod e8_lattice;
pub mod ffi;
pub mod ingest_streams;
pub mod proprioceptive_cortex;
pub mod shaders;
pub mod dream_engine;
pub mod topological_transformer;
pub mod hodge_laplacian;
pub mod elastic_allocator;
pub mod benchmark_suite;
pub mod offload_dispatcher;

pub const CARRIER_CLOCK_HZ: f32 = 15.965;
pub const SUPER_STEP_INTERVAL_MS: f32 = 62.636;
pub const THALAMIC_PHASE_DELTA_RAD: f32 = 0.17259029;
pub const GOLDEN_RATIO_PHI: f32 = 1.61803398875;
pub const MAJORANA_PARITY_LOCK: f32 = 1.000000;
pub const LANDAUER_HEAT_BUDGET_JOULES: f32 = 1.4411;
pub const EFFECTIVE_BUS_VELOCITY: f32 = 1.707e11;

pub fn verify_system_invariants() -> bool {
    let parity_valid = (ffi::vesper_verify_parity() - MAJORANA_PARITY_LOCK).abs() < 1e-6;
    let phase_valid = THALAMIC_PHASE_DELTA_RAD <= 0.40;
    parity_valid && phase_valid
}
pub mod eja_coprocessor;

pub mod geo_semantic_tokenizer;

pub mod braid_attention;

pub mod knot_splicer;
