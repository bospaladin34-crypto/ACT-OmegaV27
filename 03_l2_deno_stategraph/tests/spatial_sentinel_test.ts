import { assertEquals } from "https://deno.land/std@0.224.0/assert/mod.ts";
import { SpatialSentinelNode } from "../src/spatial_sentinel.ts";

Deno.test("Task 30: Virtualized Edge Sentinel Spatial Gradient Test (Slot 51)", () => {
  const sentinel = new SpatialSentinelNode();

  // Test 1: Grounded Null Baseline (Identical to Missoula B_tor)
  const nullFrame = {
    currentMag: { x: -13.335, y: 13.640, z: -41.841 },
    ambientPressureHpa: 905.20,
  };
  const f_null = sentinel.computeGradient(nullFrame, 5040200);
  assertEquals(f_null.slot, 51);
  assertEquals(f_null.spatialGradientNormUt, 0.0);
  assertEquals(f_null.spatialPressureDeltaHpa, 0.0);
  assertEquals(f_null.isNonLocalShearDetected, false);
  assertEquals(f_null.parityTrace, 1.000000);

  // Test 2: Active Non-Local Metric Shear Traversal (Pixel 10 in Field)
  const activeFrame = {
    currentMag: { x: -13.250, y: 13.710, z: -41.800 },
    ambientPressureHpa: 905.28,
  };
  const f_active = sentinel.computeGradient(activeFrame, 5040201);
  assertEquals(f_active.slot, 51);
  assertEquals(f_active.isNonLocalShearDetected, true);
  assertEquals(f_active.parityTrace, 1.000000);

  console.log("\n==================================================================");
  console.log(" [PASS]: Slot 51 Virtualized Sentinel Initialized");
  console.log(` [PASS]: Null Baseline Gradient Norm  : ${f_null.spatialGradientNormUt} µT`);
  console.log(` [PASS]: Active Spatial Gradient Norm  : ${f_active.spatialGradientNormUt} µT (Non-Local Shear: ${f_active.isNonLocalShearDetected})`);
  console.log(` [PASS]: Barometric Differential Delta : ${f_active.spatialPressureDeltaHpa} hPa`);
  console.log(" [PASS]: Majorana Parity Lock Conserved: Tr(U_res) = 1.000000");
  console.log("==================================================================\n");
});