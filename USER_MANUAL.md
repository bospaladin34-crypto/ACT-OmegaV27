# ACT-Ω v27.0: Sovereign Manifold Operational User Manual

## 1. System Requirements & Prerequisites
* **OS**: Windows 11 Pro (Host Workstation) & Android 17 (Pixel 10 Edge Node).
* **Hardware**: Intel Core i5-12450HX, NVIDIA RTX 3050 6GB with 8GB Resizable BAR enabled.
* **Toolchains**: Rust (stable), Deno (v2.0+), Node.js (v20+) & npm, Android Platform Tools (`adb.exe`).

---

## 2. Launching the Cockpit Server
1. Open PowerShell 5.1 as Administrator at `C:\sovereign_manifold_v27`.
2. Start the unified Deno server:
   .\00_orchestration_ps51\Launch-HUD.ps1
3. Open the desktop cockpit in your browser:
   http://127.0.0.1:8098

---

## 3. Operating the UARM Cognitive Chat
1. Navigate to Page 2 (ATCC Cognitive Chat).
2. Select your transducer model (`VESPER-RESEARCH`, `VESPER-CODER`, or `VESPER-BASE`).
3. Click PRE-WARM VRAM to lock weights into Host-Visible VRAM (`VRAM HOT`).
4. Check UARM CHAIN (ReBAR) to engage the automated 4-phase cascade.

---

## 4. Toggling the Pixel 10 Edge Node
1. Connect your Pixel 10 via USB 3.2.
2. Reverse the telemetry port:
   adb reverse tcp:8098 tcp:8098
3. Launch the mobile sensor HUD:
   adb shell am start -a android.intent.action.VIEW -d "http://localhost:8098/mobile"
4. Switch to Page 4 (3D Vector Scope) on the host to see the physical magnetic dipole gimbal rotate in real-time.

---

## 5. Benchmarking Leech Lattice Quantization
To benchmark real files through the 0.75 bpw Voronoi quantizer:
.\leech_file_compressor.exe "path\to\your\file.ext"
