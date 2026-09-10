// Tier 1: Fundamental Particle & Field Invariant Triplets (SM + GR)
// Zero Square Bracket Invariant strictly enforced across this file

use crate::schema::canonical_triplet::CanonicalTriplet;

// Generation 1 Fermions
pub const T1_QUARK_UP_LEFT: CanonicalTriplet = CanonicalTriplet::new(
    "T1_Q_U_L_G1", 1, "sigma_1 sigma_2", (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), 2, 2, 0.7208
);
pub const T1_QUARK_DOWN_LEFT: CanonicalTriplet = CanonicalTriplet::new(
    "T1_Q_D_L_G1", 1, "sigma_2 sigma_1", (0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), -1, 2, 0.7208
);
pub const T1_LEPTON_ELECTRON_LEFT: CanonicalTriplet = CanonicalTriplet::new(
    "T1_L_E_L_G1", 1, "sigma_1^-1 sigma_1", (0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0), -1, 0, 0.7208
);
pub const T1_LEPTON_NEUTRINO_E_LEFT: CanonicalTriplet = CanonicalTriplet::new(
    "T1_L_NU_E_L_G1", 1, "sigma_1 sigma_1", (0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0), 0, 2, 0.7208
);

// Generation 2 Fermions
pub const T1_QUARK_CHARM_LEFT: CanonicalTriplet = CanonicalTriplet::new(
    "T1_Q_C_L_G2", 1, "sigma_1 sigma_2 sigma_1 sigma_2", (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), 2, 4, 1.1423
);
pub const T1_QUARK_STRANGE_LEFT: CanonicalTriplet = CanonicalTriplet::new(
    "T1_Q_S_L_G2", 1, "sigma_2 sigma_1 sigma_2 sigma_1", (0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), -1, 4, 1.1423
);
pub const T1_LEPTON_MUON_LEFT: CanonicalTriplet = CanonicalTriplet::new(
    "T1_L_MU_L_G2", 1, "sigma_1^-1 sigma_1^-1 sigma_1^-1 sigma_1^-1", (0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0), -1, -4, 1.1423
);

// Generation 3 Fermions
pub const T1_QUARK_TOP_LEFT: CanonicalTriplet = CanonicalTriplet::new(
    "T1_Q_T_L_G3", 1, "sigma_1 sigma_2 sigma_1 sigma_2 sigma_1 sigma_2", (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), 2, 6, 1.4411
);
pub const T1_QUARK_BOTTOM_LEFT: CanonicalTriplet = CanonicalTriplet::new(
    "T1_Q_B_L_G3", 1, "sigma_2 sigma_1 sigma_2 sigma_1 sigma_2 sigma_1", (0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), -1, 6, 1.4411
);
pub const T1_LEPTON_TAU_LEFT: CanonicalTriplet = CanonicalTriplet::new(
    "T1_L_TAU_L_G3", 1, "sigma_1^-1 sigma_1^-1 sigma_1^-1 sigma_1^-1 sigma_1^-1 sigma_1^-1", (0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0), -1, -6, 1.4411
);

// Gauge Bosons & Gravitation
pub const T1_BOSON_PHOTON: CanonicalTriplet = CanonicalTriplet::new(
    "T1_BOSON_PHOTON", 1, "sigma_1 sigma_1^-1", (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), 0, 0, 0.0000
);
pub const T1_BOSON_W_MINUS: CanonicalTriplet = CanonicalTriplet::new(
    "T1_BOSON_W_MINUS", 1, "sigma_1 sigma_2^-1", (1.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), -1, 0, 0.8038
);
pub const T1_BOSON_Z0: CanonicalTriplet = CanonicalTriplet::new(
    "T1_BOSON_Z0", 1, "sigma_2 sigma_2^-1", (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), 0, 0, 0.9118
);
pub const T1_BOSON_GLUON_OCTET: CanonicalTriplet = CanonicalTriplet::new(
    "T1_BOSON_GLUON", 1, "sigma_i sigma_j sigma_i^-1 sigma_j^-1", (1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), 0, 0, 0.0000
);
pub const T1_HIGGS_SCALAR: CanonicalTriplet = CanonicalTriplet::new(
    "T1_HIGGS_H0", 1, "e", (0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5), 0, 0, 1.4411
);