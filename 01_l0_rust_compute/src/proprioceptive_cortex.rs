// Proprioceptive Cortex, 4-Phase Recovery State Machine & Oja-Hebbian Decay
// Zero Square Bracket Invariant strictly enforced across this file

pub const MISSOULA_MECHANICAL_GROUND_HZ: f32 = 36.0;
pub const LOCAL_GRAVITATIONAL_ANCHOR: f32 = 9.80665;
pub const GEO_DYNAMO_BTOR: (f32, f32, f32) = (-13.335, 13.640, -41.841);

pub enum RecoveryStatePhase {
    Phase1StableFlow,
    Phase2RasDampening,
    Phase3StomachionQuarantine,
    Phase4TopologicalReboot,
}

impl Copy for RecoveryStatePhase {}

impl Clone for RecoveryStatePhase {
    fn clone(&self) -> Self {
        *self
    }
}

impl PartialEq for RecoveryStatePhase {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (RecoveryStatePhase::Phase1StableFlow, RecoveryStatePhase::Phase1StableFlow) => true,
            (RecoveryStatePhase::Phase2RasDampening, RecoveryStatePhase::Phase2RasDampening) => true,
            (RecoveryStatePhase::Phase3StomachionQuarantine, RecoveryStatePhase::Phase3StomachionQuarantine) => true,
            (RecoveryStatePhase::Phase4TopologicalReboot, RecoveryStatePhase::Phase4TopologicalReboot) => true,
            _ => false,
        }
    }
}

pub struct ProprioceptiveCortex {
    pub current_phase: RecoveryStatePhase,
    pub measured_phase_delta: f32,
    pub bilateral_writhe_l: f32,
    pub bilateral_writhe_r: f32,
    pub parity_trace: f32,
}

impl ProprioceptiveCortex {
    pub fn new() -> Self {
        ProprioceptiveCortex {
            current_phase: RecoveryStatePhase::Phase1StableFlow,
            measured_phase_delta: 0.17259029,
            bilateral_writhe_l: 0.0,
            bilateral_writhe_r: 0.0,
            parity_trace: 1.000000,
        }
    }

    // Oja-Hebbian structural writhe decay: Delta w = eta * y * (x - y * w)
    pub fn apply_oja_writhe_decay(&self, current_writhe: f32, input_x: f32, output_y: f32, eta: f32) -> f32 {
        let delta_w = eta * output_y * (input_x - output_y * current_writhe);
        current_writhe + delta_w
    }

    // Evaluates 1N4148 virtual diode bilateral writhe symmetry bound: |w_L - w_R| <= 0.02
    pub fn verify_bilateral_writhe(&self, w_l: f32, w_r: f32) -> bool {
        let diff = (w_l - w_r).abs();
        diff <= 0.02
    }

    // Executes 4-Phase State Recovery step
    pub fn evaluate_recovery_cycle(&mut self, injected_delta_phi: f32) -> RecoveryStatePhase {
        self.measured_phase_delta = injected_delta_phi;

        if self.measured_phase_delta <= 0.40 {
            self.current_phase = RecoveryStatePhase::Phase1StableFlow;
        } else if self.measured_phase_delta <= 0.60 {
            // Phase 2: Apply 30% RAS phase dampening (contraction scalar 0.70)
            self.measured_phase_delta *= 0.70;
            self.current_phase = RecoveryStatePhase::Phase2RasDampening;
        } else if self.measured_phase_delta <= 0.80 {
            // Phase 3: Stomachion quarantine; zero-flux boundary on sigma_5^-1
            self.current_phase = RecoveryStatePhase::Phase3StomachionQuarantine;
        } else {
            // Phase 4: Full topological reboot; snap to nearest E8 root
            self.measured_phase_delta = 0.17259029;
            self.parity_trace = 1.000000;
            self.current_phase = RecoveryStatePhase::Phase4TopologicalReboot;
        }
        self.current_phase
    }
}