"""
inspect_recordings.py - Inspects headers, columns, and data formats in data/open
"""
import os
import glob
import pandas as pd

open_dir = r"C:\sovereign_manifold_v27\data\open"
files = glob.glob(os.path.join(open_dir, "*.*"))
print(f"Total files in data\\open: {len(files)}\n")

for filepath in files[:12]:
    bname = os.path.basename(filepath)
    print("=" * 70)
    print(f"FILE: {bname}")
    try:
        with open(filepath, "rb") as fp:
            raw = fp.read(400)
        
        is_binary = raw.startswith(b"\xd0\xcf\x11\xe0\xa1\xb1\x1a\xe1")
        print(f"FORMAT: {'Binary OLE2 Excel (.xls)' if is_binary else 'Plain Text (.csv / text .xls)'}")
        
        if not is_binary:
            lines = raw.decode("utf-8", errors="ignore").splitlines()[:6]
            print("RAW FIRST 4 LINES:")
            for l in lines[:4]:
                print(f"  | {l}")
        else:
            df = pd.read_excel(filepath)
            print(f"EXCEL COLUMNS: {list(df.columns)}")
            print(f"EXCEL SHAPE  : {df.shape}")
            print(f"FIRST ROW    : {df.iloc[0].to_dict()}")
    except Exception as e:
        print(f"ERROR READING: {e}")
print("=" * 70)