// Tier 2: Categorical Rewrite Morphisms & Interaction Vertices (Category C)
// Zero Square Bracket Invariant strictly enforced across this file

use crate::schema::canonical_triplet::CanonicalTriplet;

pub const T2_VERTEX_QCD_EMISSION: CanonicalTriplet = CanonicalTriplet::new(
    "T2_QCD_EMISSION", 2, "q_i -> q_j + g", (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), 0, 0, 0.0520
);
pub const T2_VERTEX_EW_CHARGED_CURRENT: CanonicalTriplet = CanonicalTriplet::new(
    "T2_EW_CHARGED_CURRENT", 2, "u_L + W^- -> d_L", (1.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), -1, 0, 0.0804
);
pub const T2_VERTEX_YUKAWA_CHIRALITY_INVERSION: CanonicalTriplet = CanonicalTriplet::new(
    "T2_YUKAWA_INVERSION", 2, "e_L + H0 -> e_R", (0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0), 0, -2, 0.1441
);
pub const T2_BRAIDSET_MASTER_OMEGA: CanonicalTriplet = CanonicalTriplet::new(
    "T2_BRAIDSET_B_OMEGA", 2, "sigma_2 sigma_3 sigma_1 sigma_4 sigma_5^-1", (1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0), 0, 3, 1.4411
);