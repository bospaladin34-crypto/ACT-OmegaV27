import os
import pandas as pd

open_dir = r"C:\sovereign_manifold_v27\data\open"
file_path = os.path.join(open_dir, "My Experiment (22).xls")

if os.path.exists(file_path):
    df = pd.read_excel(file_path, nrows=2)
    print(f"=== ALL COLUMNS IN {os.path.basename(file_path)} ===")
    for idx, col in enumerate(df.columns):
        print(f"  Col {idx:2d}: {repr(col)}")
else:
    print(f"File not found: {file_path}")