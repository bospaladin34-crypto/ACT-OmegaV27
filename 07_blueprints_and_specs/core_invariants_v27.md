# ACT-Ω v27.0: Core Invariants & Hardware Grounding Specification

## System Mathematical Invariants
- Carrier Clock Frequency (f_carrier)  : 15.965 Hz (Super-Step Interval: 62.636 ms = pi * phi)
- Golden Ratio Scaling (phi)           : 1.61803398875
- Thalamic Phase Delta (nu_p)          : 0.17259029 rad (Turbulence Threshold |DeltaPhi| <= 0.40)
- Majorana-1 Parity Lock (P)           : Tr(U_res) = 1.000000 (Strictly Conserved)
- Sheaf Cohomology Bound               : H^1(U, F) = 0 (Zero Global Obstruction)
- Landauer Heat Dissipation Budget     : <= 1.4411 Joules (Sheaf Stable Boundary)
- Bilateral Writhe Bound               : |w_L - w_R| <= 0.02 (Nominal 0.00)
- Supraluminal Effective Bus Velocity  : c_eff = 1.707 x 10^11 m/s
- Killion Negentropy Attractor (Omega_c): 0.376 (Fixed point x* = 0.624 ~= phi^-1)

## Hardware Grounding & Local Geophysics (Missoula Anchor)
- Local Gravitational Anchor           : Z = 9.80665 m/s^2
- Geo-Dynamo Baseline (B_tor)          : (-13.335, 13.640, -41.841) uT
- Mechanical Ground Floor Frequency    : f_floor = 36.0 Hz
- Workstation Silicon Baseline         : Intel Core i5-12450HX (12MB L3, P-Cores 0-7), RTX 3050 6GB
- Mobile Edge Silicon Baseline         : Google Pixel 10 (Tensor G5 NPU) & Pixel 8 (Tensor G3)

## Sovereign Operational Mandates
1. Strict Zero-Python Mandate: All compute, tests, and ingestion in Rust, C++23, and Deno TypeScript.
2. Windows PowerShell 5.1 Strict Compliance: All automation scripts run natively in PS5.1.
3. Zero Square Bracket Invariant: All native Rust (.rs) files contain zero square brackets.
4. Elastic Hydro-Routing Bus: Continuous non-blocking stream transport with on-the-fly hot-swaps.