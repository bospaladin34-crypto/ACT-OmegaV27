// Tier 3: E8 Roots & Tangram Geometric Primitives
// Zero Square Bracket Invariant strictly enforced across this file

use crate::schema::canonical_triplet::{CanonicalTriplet, E8Root8D};

pub const T3_TANGRAM_MANIFOLD_INGESTION: CanonicalTriplet = CanonicalTriplet::new(
    "T3_TANGRAM_LARGE_TRIANGLE", 3, "sigma_4 sigma_2", (1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), 0, 2, 0.0421
);

pub const T3_TANGRAM_PRIME_PROJECTION: CanonicalTriplet = CanonicalTriplet::new(
    "T3_TANGRAM_SQUARE", 3, "P_prime", (1.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), 0, 0, 0.0155
);

pub const T3_TANGRAM_AEGIS_BYPASS: CanonicalTriplet = CanonicalTriplet::new(
    "T3_TANGRAM_PARALLELOGRAM", 3, "sigma_1", (0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0), 0, 1, 0.0100
);

pub const T3_TANGRAM_REIDEMEISTER_OPTIMIZER: CanonicalTriplet = CanonicalTriplet::new(
    "T3_TANGRAM_SMALL_TRIANGLE", 3, "sigma_i sigma_i^-1 -> e", (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), 0, 0, 0.0000
);