// Tier 1: Fundamental Particle & Field Invariant Triplets
// Zero Square Bracket Invariant strictly enforced across this file

use crate::schema::canonical_triplet::{CanonicalTriplet, E8Root8D};

pub const T1_QUARK_UP_LEFT: CanonicalTriplet = CanonicalTriplet::new(
    "T1_Q_U_L_G1", 1, "sigma_1 sigma_2", (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), 2, 2, 0.7208
);

pub const T1_QUARK_DOWN_LEFT: CanonicalTriplet = CanonicalTriplet::new(
    "T1_Q_D_L_G1", 1, "sigma_2 sigma_1", (0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), -1, 2, 0.7208
);

pub const T1_LEPTON_ELECTRON_LEFT: CanonicalTriplet = CanonicalTriplet::new(
    "T1_L_E_L_G1", 1, "sigma_1^-1 sigma_1", (0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0), -1, 0, 0.7208
);

pub const T1_BOSON_PHOTON: CanonicalTriplet = CanonicalTriplet::new(
    "T1_BOSON_PHOTON", 1, "sigma_1 sigma_1^-1", (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), 0, 0, 0.0000
);

pub const T1_HIGGS_SCALAR: CanonicalTriplet = CanonicalTriplet::new(
    "T1_HIGGS_H0", 1, "e", (0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5), 0, 0, 1.4411
);