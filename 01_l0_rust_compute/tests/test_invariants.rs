// Native Rust Test Suite for Invariants and Algebraic Triplets
// Zero Square Bracket Invariant strictly enforced across this file

use vesper_ffi::braid_engine::ArtinBraidWord;
use vesper_ffi::e8_lattice::{decode_conway_sloane_e8, calculate_e8_norm_squared};
use vesper_ffi::ffi::vesper_verify_parity;
use vesper_ffi::verify_system_invariants;

fn main() {
    let parity = vesper_verify_parity();
    assert!((parity - 1.000000).abs() < 1e-6);

    let mut braid = ArtinBraidWord::new();
    braid.append_crossing(1, false);
    braid.append_crossing(1, true);
    assert_eq!(braid.crossings.len(), 2);
    
    let cancelled = braid.simplify_reidemeister_ii();
    assert_eq!(cancelled, 1);
    assert_eq!(braid.crossings.len(), 0);
    assert_eq!(braid.net_writhe, 0);

    let input = (1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let decoded = decode_conway_sloane_e8(&input);
    let sum = decoded.0 + decoded.1 + decoded.2 + decoded.3 + decoded.4 + decoded.5 + decoded.6 + decoded.7;
    assert_eq!((sum as i32) % 2, 0);

    // Validate E8 root norm squared is an even integer (||r||^2 == 2.0)
    let norm_sq = calculate_e8_norm_squared(&decoded);
    assert!((norm_sq - 2.0).abs() < 1e-6);

    assert!(verify_system_invariants());
    println!("All L0 Invariant Tests PASSED.");
}