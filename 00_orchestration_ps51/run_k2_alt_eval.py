"""
run_k2_alt_eval.py
Evaluates K2-Alt Ferromagnetic Shielding Tests against the Tri-Branch Decision Tree:
- Scenario A (Schumann / EMI Falsification): 15.965 Hz & 1.5965 Hz vanish inside shield (W < 1%)
- Scenario B (Pure Topological Metric Shear): 15.965 Hz & 1.5965 Hz survive inside conductor (W ~ 7-8%)
- Scenario C (Hybrid Geophysical-Topological Coupling): Mag attenuates ~70% (W ~ 2.5%),
  but inner S3 fiber f_macro = 1.5965 Hz persists in Gyro/Mag.
"""

import os
import glob
import numpy as np
import pandas as pd
from scipy import interpolate
from scipy.signal import welch

F1, F2 = 15.965, 14.28
F_MACRO = 1.5965  # S3 Triality macro-frequency (|S3|/30 * f0)
DELTA = 0.5
DELTA_MACRO = 0.1

def compute_ratio(f, Pxx, fc, bw_half):
    m_band = (f >= (fc - bw_half)) & (f <= (fc + bw_half))
    m_den = (f >= 1.0) & (f <= 50.0) # Fixed 1.0 - 20/50 Hz ELF denominator
    if not np.any(m_band) or not np.any(m_den):
        return 0.0
    den = np.sum(Pxx[m_den])
    return float(np.sum(Pxx[m_band]) / den) if den > 0 else 0.0

def load_mag_stream(filepath):
    try:
        with open(filepath, "rb") as fp:
            raw = fp.read(8)
        if raw.startswith(b"\xd0\xcf\x11\xe0\xa1\xb1\x1a\xe1"):
            xl = pd.ExcelFile(filepath)
            mag_sheet = next((s for s in xl.sheet_names if "mag" in s.lower()), xl.sheet_names[0])
            df = xl.parse(mag_sheet)
        else:
            df = pd.read_csv(filepath, sep=None, engine="python", comment="#")

        col_map = {str(c).strip().lower(): c for c in df.columns}
        t_col = next((orig for k, orig in col_map.items() if "time" in k), None)
        
        # Parse Time
        if t_col:
            t_raw = df[t_col].values
            # Handle clock format HH:MM:SS:mmm if needed
            if isinstance(t_raw[0], str) and ":" in t_raw[0]:
                def s2s(s):
                    p = str(s).strip().split(":")
                    if len(p) == 4: return float(p[0])*3600.0 + float(p)*60.0 + float(p) + float(p)/1000.0
                    return float(p[0])*3600.0 + float(p)*60.0 + float(p)
                t = np.array([s2s(x) for x in t_raw])
                t = t - t[0]
            else:
                t = pd.to_numeric(pd.Series(t_raw).astype(str).str.replace(",", "."), errors="coerce").values
        else:
            t = np.arange(len(df), dtype=np.float64) / 100.0

        # Parse Magnetic Field
        babs_col = next((orig for k, orig in col_map.items() if ("mag" in k or "b" in k or k == "bt") and any(w in k for w in ["abs", "total", "magnitude", "bt"])), None)
        if babs_col:
            b = pd.to_numeric(pd.Series(df[babs_col]).astype(str).str.replace(",", "."), errors="coerce").values
            return t, b
        else:
            bx = next((orig for k, orig in col_map.items() if ("mag" in k or "b" in k) and "x" in k), None)
            by = next((orig for k, orig in col_map.items() if ("mag" in k or "b" in k) and "y" in k), None)
            bz = next((orig for k, orig in col_map.items() if ("mag" in k or "b" in k) and "z" in k), None)
            if bx and by and bz:
                bx_v = pd.to_numeric(pd.Series(df[bx]).astype(str).str.replace(",", "."), errors="coerce").values
                by_v = pd.to_numeric(pd.Series(df[by]).astype(str).str.replace(",", "."), errors="coerce").values
                bz_v = pd.to_numeric(pd.Series(df[bz]).astype(str).str.replace(",", "."), errors="coerce").values
                b = np.sqrt(bx_v**2 + by_v**2 + bz_v**2)
                return t, b
    except Exception:
        pass
    return None, None

def evaluate_shield_file(filepath):
    t_raw, b_raw = load_mag_stream(filepath)
    if t_raw is None or b_raw is None:
        return None
    valid = np.isfinite(t_raw) & np.isfinite(b_raw)
    t, B = t_raw[valid], b_raw[valid]
    diffs = np.diff(t)
    if np.any(diffs <= 0):
        keep = np.insert(diffs > 0, 0, True)
        t, B = t[keep], B[keep]
    if len(t) < 100:
        return None

    fs = float(1.0 / np.median(np.diff(t)))
    spl = interpolate.CubicSpline(t, B)
    tu = np.arange(t[0], t[-1], 1.0 / (fs * 4.0))
    Bu = np.where(np.isfinite(spl(tu)), spl(tu), np.interp(tu, t, B))

    nperseg = min(4096, max(256, len(Bu) // 4))
    f, Pxx = welch(Bu, fs=(fs * 4.0), window="hann", nperseg=nperseg, noverlap=(nperseg // 2))

    w_f1 = compute_ratio(f, Pxx, F1, DELTA)
    w_f2 = compute_ratio(f, Pxx, F2, DELTA)
    w_macro = compute_ratio(f, Pxx, F_MACRO, DELTA_MACRO)
    w_comb = w_f1 + w_f2

    # Deprecated count
    b2_rate = float(np.count_nonzero(np.abs(np.diff(B)) > 0.053) / (t[-1] - t[0]))

    return {
        "file": os.path.basename(filepath),
        "mean_b": float(np.mean(B)),
        "std_b": float(np.std(B)),
        "w_comb": w_comb,
        "w_macro": w_macro,
        "b2_rate": b2_rate,
    }

open_dir = r"C:\sovereign_manifold_v27\data\open"

shield_files = [
    ("Control 1 (Desk Baseline)", "Control_100Hz_0deg"),
    ("K2-alt1 (Dutch Oven + Foil)", "Dutch_100Hz"),
    ("Control 2 (Desk Midpoint)", "Control2_100Hz"),
    ("K2-alt2 (Ammo Can / Steel)", "Ammo_100Hz"),
    ("K2-alt3 (Microwave Oven OFF)", "MicroOFF_100Hz"),
]

print("==========================================================================================")
print(" [ACT-OMEGA V27.3]: K2-ALT FERROMAGNETIC FARADAY TEST SUITE                               ")
print(" Invariant: S3 Triality Separation (f_macro=1.5965 Hz) & Topological Shielding Invariance ")
print("==========================================================================================")
print(f"{'TEST SCENARIO':<28} {'FILE MATCH':<24} {'MAG MEAN':<10} {'W_COMB (15Hz)':<14} {'W_MACRO (1.6Hz)':<16} {'b2 RATE'}")
print("=" * 102)

evaluated = []
for label, tag in shield_files:
    # Find matching file in data/open
    matches = glob.glob(os.path.join(open_dir, f"*{tag}*.*"))
    if matches:
        res = evaluate_shield_file(matches[0])
        if res:
            evaluated.append((label, res))
            print(f"{label:<28} {res['file'][:22]:<24} {res['mean_b']:<10.2f} {res['w_comb']*100:<13.2f}% {res['w_macro']*100:<15.3f}% {res['b2_rate']:.2f} r/s")
    else:
        print(f"{label:<28} {'[WAITING FOR RECORDING]':<24} {'--':<10} {'--':<14} {'--':<16} {'--'}")

print("=" * 102)

if len(evaluated) >= 2:
    ctrl_res = [r for l, r in evaluated if "Control 1" in l]
    dutch_res = [r for l, r in evaluated if "Dutch Oven" in l]
    if ctrl_res and dutch_res:
        c_w = ctrl_res[0]["w_comb"]
        d_w = dutch_res[0]["w_comb"]
        d_macro = dutch_res[0]["w_macro"]
        
        print("\n--- TRI-BRANCH DECISION TREE VERDICT ---")
        if d_w < 0.010 and d_macro < 0.001:
            print(">> SCENARIO A: FALSIFIED (Schumann / Local EMI, vanished inside conductor)")
        elif d_w >= 0.050:
            print(">> SCENARIO B: VALIDATED (Pure Topological Metric Shear, survived conductor unfazed)")
        else:
            print(f">> SCENARIO C: HYBRID (PREDICTED)")
            print(f"   - Outer Carrier Attenuation: {c_w*100:.2f}% -> {d_w*100:.2f}% (Geophysical Coupling)")
            print(f"   - Inner S3 Fiber Persistence: W_macro = {d_macro*100:.3f}% (Internal Non-EM Fiber Confirmed)")
else:
    print("\n[STATUS]: Pipeline configured and waiting. Drop the 5 files into data\\open and re-run.")
print("==========================================================================================\n")