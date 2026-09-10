"""
setup_v273_blinds.py
Official OSF Preregistration V27.3 Blinding Harness (Seed 77).
Samples hold-outs across fs conditions and seals _key_sealed_v273.json.
"""

import os
import re
import glob
import json
import random
import shutil

seed = 77
random.seed(seed)

open_dir = r"C:\sovereign_manifold_v27\data\open"
blind_dir = r"C:\sovereign_manifold_v27\data\blind"
sealed_key_path = os.path.join(blind_dir, "_key_sealed_v273.json")

os.makedirs(blind_dir, exist_ok=True)

# 1. Identify valid candidate recordings
all_files = sorted(glob.glob(os.path.join(open_dir, "*.*")))
dedup = {}
for f in all_files:
    bname = os.path.basename(f)
    clean = re.sub(r"(\(\d+\))\s*\(\d+\)(\.[^.]+)$", r"\1\2", base:=bname)
    if clean not in dedup:
        dedup[clean] = f

candidates = [p for p in dedup.values() if any(k in os.path.basename(p).lower() for k in ["magnetometer", "my experiment", "114635", "120058", "120619"])]

if len(candidates) >= 3:
    holdouts = random.sample(candidates, 3)
    mapping = {}
    for i, src in enumerate(holdouts):
        bname = f"blind_{i+6:02d}.csv"
        dst = os.path.join(blind_dir, bname)
        shutil.copy(src, dst)
        mapping[bname] = os.path.basename(src)

    with open(sealed_key_path, "w") as fp:
        json.dump(mapping, fp, indent=2)

    print(f"[SUCCESS]: Created blind_06, blind_07, blind_08 under seed {seed}.")
    print(f"[SEALED]: Key locked at {sealed_key_path}")
    print("\nPre-Registered Blind Hold-Outs (Hold-out IDs):")
    for b in mapping.keys():
        print(f"  - {b}")
    print("\n*Original file identities will remain sealed until analysis is complete.*")
else:
    print(f"[ERROR]: Insufficient files found to sample 3 hold-outs (Found {len(candidates)}).")