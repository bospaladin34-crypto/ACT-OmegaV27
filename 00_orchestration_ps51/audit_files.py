import os
import glob
import pandas as pd

open_dir = r"C:\sovereign_manifold_v27\data\open"
files = glob.glob(os.path.join(open_dir, "*.*"))
print(f"Total files currently in data\\open: {len(files)}\n")

file_table = []
for f in files:
    bname = os.path.basename(f)
    if not bname.lower().endswith((".csv", ".xls", ".xlsx")):
        continue
    try:
        with open(f, "rb") as fp:
            raw = fp.read(8)
        if raw.startswith(b"\xd0\xcf\x11\xe0\xa1\xb1\x1a\xe1"):
            df = pd.read_excel(f, nrows=3)
        else:
            df = pd.read_csv(f, sep=None, engine="python", nrows=3)
        file_table.append({
            "file": bname,
            "cols": list(df.columns)
        })
    except Exception as e:
        file_table.append({"file": bname, "cols": [f"ERROR: {e}"]})

print(f"{'FILE':<42} {'COLUMNS'}")
print("=" * 100)
for item in file_table:
    col_str = ", ".join([str(c) for c in item['cols'][:4]])
    if len(item['cols']) > 4: 
        col_str += "..."
    print(f"{item['file'][:40]:<42} {col_str}")