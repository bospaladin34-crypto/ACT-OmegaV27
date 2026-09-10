import os
import pandas as pd

open_dir = r"C:\sovereign_manifold_v27\data\open"
file_path = os.path.join(open_dir, "My Experiment (22).xls")

if os.path.exists(file_path):
    xl = pd.ExcelFile(file_path)
    print(f"=== SHEETS IN {os.path.basename(file_path)} ===")
    for i, name in enumerate(xl.sheet_names):
        print(f"  Sheet {i}: '{name}'")
        
    # Preview Magnetometer sheet if found
    for name in xl.sheet_names:
        if "mag" in name.lower():
            df_mag = xl.parse(name, nrows=3)
            print(f"\n[MAGNETOMETER SHEET: '{name}'] Columns:")
            for j, c in enumerate(df_mag.columns):
                print(f"    Col {j}: {repr(c)}")
            break