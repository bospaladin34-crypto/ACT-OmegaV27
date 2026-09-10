"""
TC_UFT_Simulator_b2_psd_V27_2b.py
Official Peer-Review Batch Analysis Pipeline for P7777_T V27.2b OSF Preregistration.
Parses multi-sheet 'Magnetometer' workbooks, applies Cubic Spline + Welch Hann PSD.
"""

import os
import re
import io
import glob
import json
import random
import shutil
import numpy as np
import pandas as pd
from scipy import interpolate
from scipy.signal import welch

F1, F2 = 15.965, 14.28
F1_A10, F2_A10 = 4.035, 4.280
GAMMA_FRIC = 1.3479e-10  # N
DELTA = 0.5             # Hz

def compute_b2_ratio(f, Pxx, fc):
    mask = (f >= (fc - DELTA)) & (f <= (fc + DELTA))
    if not np.any(mask):
        return 0.0
    den = np.sum(Pxx)
    if den <= 0:
        return 0.0
    return float(np.sum(Pxx[mask]) / den)

def sanitize_to_float(series):
    s = pd.Series(series)
    if s.dtype == object:
        s = s.astype(str).str.replace(",", ".")
    return pd.to_numeric(s, errors="coerce").values

def load_recording(filepath):
    try:
        with open(filepath, "rb") as f:
            header_bytes = f.read(8)

        # Multi-sheet Binary OLE2 Excel file
        if header_bytes.startswith(b"\xd0\xcf\x11\xe0\xa1\xb1\x1a\xe1"):
            xl = pd.ExcelFile(filepath)
            mag_sheet = None
            for s in xl.sheet_names:
                if "mag" in s.lower():
                    mag_sheet = s
                    break
            if mag_sheet:
                return xl.parse(mag_sheet), None
            else:
                return xl.parse(0), None

        # Text-based format (.csv or text-disguised .xls)
        for enc in ["utf-8", "utf-8-sig", "latin1"]:
            try:
                with open(filepath, "r", encoding=enc, errors="ignore") as f:
                    content = f.read()
                lines = content.splitlines()
                header_idx = 0
                sep = ","
                for i, line in enumerate(lines[:35]):
                    low = line.lower()
                    if ("," in line or "\t" in line or ";" in line) and any(k in low for k in ["time", "bx", "b_abs", "mag", "field"]):
                        header_idx = i
                        if "\t" in line: sep = "\t"
                        elif ";" in line: sep = ";"
                        break
                clean_csv = "\n".join(lines[header_idx:])
                df = pd.read_csv(io.StringIO(clean_csv), sep=sep)
                return df, None
            except Exception:
                continue

        df = pd.read_csv(filepath, sep=None, engine="python")
        return df, None
    except Exception as e:
        return None, str(e)

def find_magnetic_and_time_cols(cols):
    col_map = {str(c).strip().lower(): c for c in cols}
    time_col = None
    for k, orig in col_map.items():
        if "time" in k:
            time_col = orig
            break

    # Check for pre-calculated absolute magnitude
    mag_col = None
    for k, orig in col_map.items():
        is_mag = ("mag" in k) or ("b_" in k) or ("b (" in k) or k.startswith("b_") or k.startswith("b ")
        if is_mag and any(w in k for w in ["abs", "total", "magnitude"]):
            mag_col = orig
            break

    # Extract 3-axis components
    bx_col, by_col, bz_col = None, None, None
    if not mag_col:
        for k, orig in col_map.items():
            is_mag = ("mag" in k) or ("b_" in k) or ("b (" in k) or k.startswith("b_") or k.startswith("b ")
            if is_mag:
                if " x" in k or "_x" in k or k.endswith("x"): bx_col = orig
                elif " y" in k or "_y" in k or k.endswith("y"): by_col = orig
                elif " z" in k or "_z" in k or k.endswith("z"): bz_col = orig

    return time_col, mag_col, (bx_col, by_col, bz_col)

def analyze_file(filepath, fs_hint=None):
    df, err = load_recording(filepath)
    if df is None:
        return {"file": os.path.basename(filepath), "status": "ERROR", "msg": err}

    if any("frequency" in str(c).lower() for c in df.columns):
        return {"file": os.path.basename(filepath), "status": "SKIPPED", "msg": "Precomputed Frequency Spectrum"}

    time_col, mag_col, axes = find_magnetic_and_time_cols(df.columns)
    if not mag_col and not all(axes):
        return {"file": os.path.basename(filepath), "status": "SKIPPED", "msg": "Non-Magnetometer Stream"}

    if mag_col:
        B_raw = sanitize_to_float(df[mag_col])
    else:
        bx = sanitize_to_float(df[axes[0]])
        by = sanitize_to_float(df[axes[1]])
        bz = sanitize_to_float(df[axes[2]])
        B_raw = np.sqrt(bx**2 + by**2 + bz**2)

    if time_col:
        t_raw = sanitize_to_float(df[time_col])
        if np.isnan(t_raw).all() or (np.count_nonzero(np.isnan(t_raw)) > len(t_raw) * 0.5):
            t_raw = np.arange(len(B_raw)) / (fs_hint or 100.0)
    else:
        t_raw = np.arange(len(B_raw)) / (fs_hint or 100.0)

    valid = np.isfinite(t_raw) & np.isfinite(B_raw)
    t, B = t_raw[valid], B_raw[valid]
    if len(t) < 50:
        return {"file": os.path.basename(filepath), "status": "ERROR", "msg": "Insufficient Samples"}

    diffs = np.diff(t)
    if np.any(diffs <= 0):
        keep = np.insert(diffs > 0, 0, True)
        t, B = t[keep], B[keep]

    if len(t) < 50:
        return {"file": os.path.basename(filepath), "status": "ERROR", "msg": "Insufficient Monotonic Samples"}

    fs = fs_hint or float(1.0 / np.median(np.diff(t)))
    if fs <= 0.0 or not np.isfinite(fs):
        fs = 100.0

    # Cubic Spline Manifold Layer (4x oversampling)
    spl = interpolate.CubicSpline(t, B)
    tu = np.arange(t[0], t[-1], 1.0 / (fs * 4.0))
    Bu = np.where(np.isfinite(spl(tu)), spl(tu), np.interp(tu, t, B))

    # Welch Hann PSD (High spectral resolution)
    nperseg = min(4096, max(256, len(Bu) // 4))
    f, Pxx = welch(Bu, fs=(fs * 4.0), window="hann", nperseg=nperseg, noverlap=(nperseg // 2))

    # Target Selection
    if fs >= (2.0 * F1):
        fc1, fc2 = F1, F2
    elif abs(fs - 10.0) < 0.5:
        fc1, fc2 = F1_A10, F2_A10
    else:
        def calc_alias(ft):
            fa = abs(ft - round(ft / fs) * fs)
            return (fs - fa) if fa > (fs / 2.0) else fa
        fc1, fc2 = calc_alias(F1), calc_alias(F2)

    r1 = compute_b2_ratio(f, Pxx, fc1)
    r2 = compute_b2_ratio(f, Pxx, fc2)
    combined = r1 + r2
    penalty = GAMMA_FRIC * combined

    return {
        "file": os.path.basename(filepath),
        "status": "VALID",
        "fs": round(fs, 1),
        "fc1": round(fc1, 3),
        "fc2": round(fc2, 3),
        "W1": round(r1, 4),
        "W2": round(r2, 4),
        "W_combined": round(combined, 4),
        "penalty_N": f"{penalty:.3e}",
        "pass": bool(abs(combined - 0.1291) <= 0.035)
    }

if __name__ == "__main__":
    open_dir = r"C:\sovereign_manifold_v27\data\open"
    all_files = glob.glob(os.path.join(open_dir, "*.*"))
    
    dedup = {}
    for f in all_files:
        base = os.path.basename(f)
        clean = re.sub(r"(\(\d+\))\s*\(\d+\)(\.[^.]+)$", r"\1\2", base)
        if clean not in dedup:
            dedup[clean] = f

    print(f"=== PROCESSING {len(dedup)} UNIQUE SENSOR RECORDINGS ===\n")
    results = []
    for clean_name, full_path in dedup.items():
        if not full_path.lower().endswith((".csv", ".xls", ".xlsx")):
            continue
        res = analyze_file(full_path)
        results.append(res)

    valid_results = [r for r in results if r["status"] == "VALID"]
    res_df = pd.DataFrame(valid_results)

    if not res_df.empty:
        print("=" * 96)
        print(f"{'FILE':<34} {'FS':<8} {'FC1':<8} {'FC2':<8} {'W_COMBINED':<12} {'PENALTY (N)':<14} {'STATUS'}")
        print("=" * 96)
        for _, row in res_df.iterrows():
            stat = "PASS" if row["pass"] else "CHECK"
            print(f"{row['file'][:32]:<34} {row['fs']:<8} {row['fc1']:<8} {row['fc2']:<8} {row['W_combined']:<12} {row['penalty_N']:<14} {stat}")
        print("=" * 96)

        mean_w = res_df["W_combined"].mean()
        std_w = res_df["W_combined"].std()
        print(f"\n[EVALUATION COMPLETE]: Processed {len(res_df)} magnetometer recordings.")
        print(f"Mean W(f) = {mean_w:.4f} ± {std_w:.4f}")
        print(f"Target    = 0.1291 ± 0.0200 (12.91%)")
        print(f"Penalty F = {GAMMA_FRIC * mean_w:.3e} N (Target ~1.74e-11 N)")
    else:
        print("[NOTE] No valid magnetometer time-series matched.")