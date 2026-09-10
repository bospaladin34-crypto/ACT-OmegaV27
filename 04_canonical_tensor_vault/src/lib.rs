// 5-Tier Canonical Topological Tensor Vault Root
// Zero Square Bracket Invariant strictly enforced across this file

pub mod schema;
pub mod tables;
pub mod rabitq_hnsw;

use schema::canonical_triplet::CanonicalTriplet;

pub fn get_canonical_tensor_by_id(id: &str) -> Option<&CanonicalTriplet> {
    // Tier 1
    if id == "T1_Q_U_L_G1" { return Some(&tables::tier1_particles::T1_QUARK_UP_LEFT); }
    if id == "T1_Q_D_L_G1" { return Some(&tables::tier1_particles::T1_QUARK_DOWN_LEFT); }
    if id == "T1_L_E_L_G1" { return Some(&tables::tier1_particles::T1_LEPTON_ELECTRON_LEFT); }
    if id == "T1_L_NU_E_L_G1" { return Some(&tables::tier1_particles::T1_LEPTON_NEUTRINO_E_LEFT); }
    if id == "T1_Q_C_L_G2" { return Some(&tables::tier1_particles::T1_QUARK_CHARM_LEFT); }
    if id == "T1_Q_S_L_G2" { return Some(&tables::tier1_particles::T1_QUARK_STRANGE_LEFT); }
    if id == "T1_L_MU_L_G2" { return Some(&tables::tier1_particles::T1_LEPTON_MUON_LEFT); }
    if id == "T1_Q_T_L_G3" { return Some(&tables::tier1_particles::T1_QUARK_TOP_LEFT); }
    if id == "T1_Q_B_L_G3" { return Some(&tables::tier1_particles::T1_QUARK_BOTTOM_LEFT); }
    if id == "T1_L_TAU_L_G3" { return Some(&tables::tier1_particles::T1_LEPTON_TAU_LEFT); }
    if id == "T1_BOSON_PHOTON" { return Some(&tables::tier1_particles::T1_BOSON_PHOTON); }
    if id == "T1_BOSON_W_MINUS" { return Some(&tables::tier1_particles::T1_BOSON_W_MINUS); }
    if id == "T1_BOSON_Z0" { return Some(&tables::tier1_particles::T1_BOSON_Z0); }
    if id == "T1_BOSON_GLUON" { return Some(&tables::tier1_particles::T1_BOSON_GLUON_OCTET); }
    if id == "T1_HIGGS_H0" { return Some(&tables::tier1_particles::T1_HIGGS_SCALAR); }

    // Tier 2
    if id == "T2_QCD_EMISSION" { return Some(&tables::tier2_morphisms::T2_VERTEX_QCD_EMISSION); }
    if id == "T2_EW_CHARGED_CURRENT" { return Some(&tables::tier2_morphisms::T2_VERTEX_EW_CHARGED_CURRENT); }
    if id == "T2_YUKAWA_INVERSION" { return Some(&tables::tier2_morphisms::T2_VERTEX_YUKAWA_CHIRALITY_INVERSION); }
    if id == "T2_BRAIDSET_B_OMEGA" { return Some(&tables::tier2_morphisms::T2_BRAIDSET_MASTER_OMEGA); }

    // Tier 3
    if id == "T3_TANGRAM_LARGE_TRIANGLE" { return Some(&tables::tier3_primitives::T3_TANGRAM_MANIFOLD_INGESTION); }
    if id == "T3_TANGRAM_SQUARE" { return Some(&tables::tier3_primitives::T3_TANGRAM_PRIME_PROJECTION); }
    if id == "T3_TANGRAM_PARALLELOGRAM" { return Some(&tables::tier3_primitives::T3_TANGRAM_AEGIS_BYPASS); }
    if id == "T3_TANGRAM_SMALL_TRIANGLE" { return Some(&tables::tier3_primitives::T3_TANGRAM_REIDEMEISTER_OPTIMIZER); }
    if id == "T3_SANTOS_ROTATION_108" { return Some(&tables::tier3_primitives::T3_OPERATOR_SANTOS_ROTATION); }

    // Tier 4
    if id == "T4_BOM_SINX_NANOPORE" { return Some(&tables::tier4_bom::T4_BOM_SINX_NANOPORE); }
    if id == "T4_BOM_DNA_ORIGAMI_ROTOR" { return Some(&tables::tier4_bom::T4_BOM_DNA_ORIGAMI_ROTOR); }
    if id == "T4_BOM_AU_MICROELECTRODES" { return Some(&tables::tier4_bom::T4_BOM_AU_MICROELECTRODES); }
    if id == "T4_ENTROPIC_STATIC_COMPENSATOR" { return Some(&tables::tier4_bom::T4_ENTROPIC_STATIC_COMPENSATOR); }
    if id == "T4_ENTROPIC_KINETIC_MARTINGALE" { return Some(&tables::tier4_bom::T4_ENTROPIC_KINETIC_MARTINGALE); }
    if id == "T4_GRAVITY_CALIBRATED_ZERO" { return Some(&tables::tier4_bom::T4_GRAVITY_CALIBRATED_ZERO); }
    if id == "T4_HEARTBEAT_RECORD_QUANTUM" { return Some(&tables::tier4_bom::T4_HEARTBEAT_RECORD_QUANTUM); }

    // Tier 5
    if id == "T5_ROUTE_EXPLORATORY" { return Some(&tables::tier5_stomachion::T5_STOMACHION_EXPLORATORY); }
    if id == "T5_ROUTE_CONSTRUCTIVE" { return Some(&tables::tier5_stomachion::T5_STOMACHION_CONSTRUCTIVE); }
    if id == "T5_ROUTE_REVERSIBLE" { return Some(&tables::tier5_stomachion::T5_STOMACHION_REVERSIBLE); }
    if id == "T5_GRAPH_K15_SYNC" { return Some(&tables::tier5_stomachion::T5_GRAPH_K15_SYNC); }
    if id == "T5_POLYTOPE_7777D_WINDOW" { return Some(&tables::tier5_stomachion::T5_POLYTOPE_7777D_WINDOW); }

    None
}