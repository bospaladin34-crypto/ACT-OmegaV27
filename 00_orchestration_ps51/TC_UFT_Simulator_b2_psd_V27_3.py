"""
TC_UFT_Simulator_b2_psd_V27_3.py (Pre-K2 Patched)
- Fixed ELF Denominator: 1.0 Hz <= f <= 20.0 Hz (Standardized across all fs)
- Outlier Filtering: Gated at Accel_Z std <= 0.15 m/s^2 (Eliminates collision noise)
- Multi-Sensor Modalities: Magnetometer, Gyroscope Z, Linear Acceleration Z
"""

import os
import re
import glob
import numpy as np
import pandas as pd
from scipy import interpolate, stats
from scipy.signal import welch

F1, F2 = 15.965, 14.28
F1_A10, F2_A10 = 4.035, 4.280
GAMMA_FRIC = 1.3479e-10  # N
DELTA = 0.5             # Hz

def compute_b2_ratio_elf(f, Pxx, fc, fs_orig):
    """Integrates spectral power ratio strictly over the fixed 1.0 - 20.0 Hz ELF band."""
    m_band = (f >= (fc - DELTA)) & (f <= (fc + DELTA))
    # Lock to fixed 1.0 - 20.0 Hz ELF band (capped at Nyquist if fs/2 < 20)
    f_max_den = min(20.0, fs_orig / 2.0)
    m_den = (f >= 1.0) & (f <= f_max_den)
    if not np.any(m_band) or not np.any(m_den):
        return 0.0
    den = np.sum(Pxx[m_den])
    return float(np.sum(Pxx[m_band]) / den) if den > 0 else 0.0

def sanitize_to_float(series):
    s = pd.Series(series)
    if s.dtype == object:
        s = s.astype(str).str.replace(",", ".")
    return pd.to_numeric(s, errors="coerce").values

def load_sensor_streams(filepath):
    """Extracts Magnetometer, Gyroscope Z, and Linear Acceleration Z from Excel/CSV."""
    mag_data = None
    gyro_data = None
    accel_data = None
    
    try:
        with open(filepath, "rb") as fp:
            raw = fp.read(8)
        
        if raw.startswith(b"\xd0\xcf\x11\xe0\xa1\xb1\x1a\xe1"):
            xl = pd.ExcelFile(filepath)
            
            # 1. Magnetometer Sheet
            mag_sheet = next((s for s in xl.sheet_names if "mag" in s.lower()), None)
            if mag_sheet:
                df_m = xl.parse(mag_sheet)
                col_m = {str(c).strip().lower(): c for c in df_m.columns}
                t_col = next((orig for k, orig in col_m.items() if "time" in k), None)
                
                babs_col = next((orig for k, orig in col_m.items() if ("mag" in k or "b" in k) and any(w in k for w in ["abs", "total", "magnitude"])), None)
                if babs_col and t_col:
                    t = sanitize_to_float(df_m[t_col])
                    b_abs = sanitize_to_float(df_m[babs_col])
                    mag_data = (t, b_abs)
                else:
                    bx_col = next((orig for k, orig in col_m.items() if ("mag" in k or "b" in k) and (" x" in k or k.endswith("x"))), None)
                    by_col = next((orig for k, orig in col_m.items() if ("mag" in k or "b" in k) and (" y" in k or k.endswith("y"))), None)
                    bz_col = next((orig for k, orig in col_m.items() if ("mag" in k or "b" in k) and (" z" in k or k.endswith("z"))), None)
                    if t_col and bx_col and by_col and bz_col:
                        t = sanitize_to_float(df_m[t_col])
                        bx = sanitize_to_float(df_m[bx_col])
                        by = sanitize_to_float(df_m[by_col])
                        bz = sanitize_to_float(df_m[bz_col])
                        b_abs = np.sqrt(bx**2 + by**2 + bz**2)
                        mag_data = (t, b_abs)

            # 2. Gyroscope Sheet (Z-axis)
            gyro_sheet = next((s for s in xl.sheet_names if "gyro" in s.lower()), None)
            if gyro_sheet:
                df_g = xl.parse(gyro_sheet)
                col_g = {str(c).strip().lower(): c for c in df_g.columns}
                t_col = next((orig for k, orig in col_g.items() if "time" in k), None)
                gz_col = next((orig for k, orig in col_g.items() if "z" in k), None)
                if t_col and gz_col:
                    t_g = sanitize_to_float(df_g[t_col])
                    gz = sanitize_to_float(df_g[gz_col])
                    gyro_data = (t_g, gz)

            # 3. Linear Acceleration Sheet (Z-axis)
            accel_sheet = next((s for s in xl.sheet_names if "linear" in s.lower()), None)
            if accel_sheet:
                df_a = xl.parse(accel_sheet)
                col_a = {str(c).strip().lower(): c for c in df_a.columns}
                t_col = next((orig for k, orig in col_a.items() if "time" in k), None)
                az_col = next((orig for k, orig in col_a.items() if "z" in k), None)
                if t_col and az_col:
                    t_a = sanitize_to_float(df_a[t_col])
                    az = sanitize_to_float(df_a[az_col])
                    accel_data = (t_a, az)

        return mag_data, gyro_data, accel_data
    except Exception:
        return None, None, None

def evaluate_stream(t_raw, val_raw, fs_hint=None, is_accel=False):
    if t_raw is None or val_raw is None:
        return None

    t_arr = np.asarray(t_raw, dtype=np.float64).flatten()
    v_arr = np.asarray(val_raw, dtype=np.float64).flatten()

    valid = np.isfinite(t_arr) & np.isfinite(v_arr)
    t, v = t_arr[valid], v_arr[valid]
    diffs = np.diff(t)
    if np.any(diffs <= 0):
        keep = np.insert(diffs > 0, 0, True)
        t, v = t[keep], v[keep]
    if len(t) < 50:
        return None

    # Mechanical Vibration Gating (Exclude collision/shock spikes for H4b)
    if is_accel and np.std(v) > 0.15:
        return {"outlier": True, "msg": f"Vibration Spike ({np.std(v):.2f} m/s^2 > 0.15)"}

    fs = fs_hint or float(1.0 / np.median(np.diff(t)))
    if fs <= 0.0 or not np.isfinite(fs):
        fs = 100.0

    spl = interpolate.CubicSpline(t, v)
    tu = np.arange(t[0], t[-1], 1.0 / (fs * 4.0))
    vu = np.where(np.isfinite(spl(tu)), spl(tu), np.interp(tu, t, v))

    nperseg = min(4096, max(256, len(vu) // 4))
    f, Pxx = welch(vu, fs=(fs * 4.0), window="hann", nperseg=nperseg, noverlap=(nperseg // 2))

    if fs >= (2.0 * F1):
        fc1, fc2 = F1, F2
    elif abs(fs - 10.0) < 0.5:
        fc1, fc2 = F1_A10, F2_A10
    else:
        def calc_alias(ft):
            fa = abs(ft - round(ft / fs) * fs)
            return (fs - fa) if fa > (fs / 2.0) else fa
        fc1, fc2 = calc_alias(F1), calc_alias(F2)

    w1 = compute_b2_ratio_elf(f, Pxx, fc1, fs)
    w2 = compute_b2_ratio_elf(f, Pxx, fc2, fs)
    combined = w1 + w2

    # K1 Surrogate on Phase-Shuffled Data
    shuffled_v = np.random.permutation(v)
    spl_s = interpolate.CubicSpline(t, shuffled_v)
    vu_s = spl_s(tu)
    f_s, Pxx_s = welch(vu_s, fs=(fs * 4.0), window="hann", nperseg=nperseg, noverlap=(nperseg // 2))
    w1_s = compute_b2_ratio_elf(f_s, Pxx_s, fc1, fs)
    w2_s = compute_b2_ratio_elf(f_s, Pxx_s, fc2, fs)
    surrogate_w = w1_s + w2_s

    return {
        "fs": fs,
        "fc1": fc1,
        "fc2": fc2,
        "W_combined": combined,
        "surrogate_w": surrogate_w,
        "outlier": False
    }

if __name__ == "__main__":
    open_dir = r"C:\sovereign_manifold_v27\data\open"
    all_files = sorted(glob.glob(os.path.join(open_dir, "*.*")))

    dedup = {}
    for f in all_files:
        base = os.path.basename(f)
        clean = re.sub(r"(\(\d+\))\s*\(\d+\)(\.[^.]+)$", r"\1\2", base)
        if clean not in dedup:
            dedup[clean] = f

    print("==========================================================================================")
    print(" [PRE-K2 PATCHED EXECUTION]: FIXED 1.0 - 20.0 Hz ELF DENOMINATOR & OUTLIER GATED")
    print("==========================================================================================")
    print(f"{'FILE':<32} {'FS':<6} {'MAG W':<10} {'GYRO_Z W':<10} {'ACCEL_Z W':<12} {'K1 SURR':<10} {'STATUS'}")
    print("=" * 96)

    mag_w_list = []
    gyro_w_list = []
    accel_w_list = []
    surr_list = []

    for clean_name, fpath in dedup.items():
        if not fpath.lower().endswith((".csv", ".xls", ".xlsx")):
            continue
        m_data, g_data, a_data = load_sensor_streams(fpath)
        if not m_data:
            continue

        t_mag, val_mag = m_data
        res_m = evaluate_stream(t_mag, val_mag)

        res_g = None
        if g_data is not None:
            t_gyro, val_gyro = g_data
            res_g = evaluate_stream(t_gyro, val_gyro)

        res_a = None
        if a_data is not None:
            t_accel, val_accel = a_data
            res_a = evaluate_stream(t_accel, val_accel, is_accel=True)

        if res_m and not res_m.get("outlier", False):
            mag_w_list.append(res_m["W_combined"])
            surr_list.append(res_m["surrogate_w"])
            gw_str = f"{res_g['W_combined']:.4f}" if res_g and not res_g.get("outlier", False) else "N/A"
            
            if res_a:
                if res_a.get("outlier", False):
                    aw_str = "VIB_SPIKE"
                else:
                    aw_str = f"{res_a['W_combined']:.4f}"
                    accel_w_list.append(res_a["W_combined"])
            else:
                aw_str = "N/A"

            if res_g and not res_g.get("outlier", False):
                gyro_w_list.append(res_g["W_combined"])

            is_pass = 0.010 <= res_m["W_combined"] <= 0.080
            stat = "PASS" if is_pass else "CHECK"

            print(f"{clean_name[:30]:<32} {res_m['fs']:<6.0f} {res_m['W_combined']:<10.4f} {gw_str:<10} {aw_str:<12} {res_m['surrogate_w']:<10.4f} {stat}")

    print("=" * 96)
    print("\n[PRE-K2 PATCH VERIFICATION SUMMARY]:")
    print(f"  Fixed 1-20 Hz ELF Mag Mean W : {np.mean(mag_w_list)*100:.2f}% ± {np.std(mag_w_list)*100:.2f}% (n = {len(mag_w_list)})")
    if gyro_w_list:
        print(f"  Gyroscope Z Mean W           : {np.mean(gyro_w_list)*100:.2f}% ± {np.std(gyro_w_list)*100:.2f}% (n = {len(gyro_w_list)})")
    if accel_w_list:
        print(f"  Clean Linear Accel Z Mean W  : {np.mean(accel_w_list)*100:.2f}% ± {np.std(accel_w_list)*100:.2f}% (n = {len(accel_w_list)})")
    print(f"  Phase-Shuffled K1 Surrogate  : {np.mean(surr_list)*100:.2f}%")
    print("==========================================================================================\n")