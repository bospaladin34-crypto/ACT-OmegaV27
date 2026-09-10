// Tier 5: 536 Stomachion Combinatorial Routing & Simplicial States
// Zero Square Bracket Invariant strictly enforced across this file

use crate::schema::canonical_triplet::CanonicalTriplet;

pub const T5_STOMACHION_EXPLORATORY: CanonicalTriplet = CanonicalTriplet::new(
    "T5_ROUTE_EXPLORATORY", 5, "sigma_2 sigma_4 sigma_6", (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), 0, 3, 0.1200
);
pub const T5_STOMACHION_CONSTRUCTIVE: CanonicalTriplet = CanonicalTriplet::new(
    "T5_ROUTE_CONSTRUCTIVE", 5, "sigma_2 sigma_3 sigma_1 sigma_4 sigma_5^-1", (1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0), 0, 3, 1.4411
);
pub const T5_STOMACHION_REVERSIBLE: CanonicalTriplet = CanonicalTriplet::new(
    "T5_ROUTE_REVERSIBLE", 5, "Delta_Garside", (1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0), 0, 0, 0.0000
);
pub const T5_GRAPH_K15_SYNC: CanonicalTriplet = CanonicalTriplet::new(
    "T5_GRAPH_K15_SYNC", 5, "K15_105_channels_lambda2_15", (1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0), 0, 15, 0.1500
);
pub const T5_POLYTOPE_7777D_WINDOW: CanonicalTriplet = CanonicalTriplet::new(
    "T5_POLYTOPE_7777D_WINDOW", 5, "P7777_cut_and_project_M48", (1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0), 0, 48, 1.4411
);