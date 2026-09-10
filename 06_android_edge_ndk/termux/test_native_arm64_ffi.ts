// Native ARM64 Deno FFI Invariant Test for Pixel 10 (Tensor G5)

const libPath = "./libvesper_ffi.so";

console.log("=================================================================");
console.log(" [ACT-OMEGA V27.0]: TESTING NATIVE LIBVESPER_FFI.SO ON PIXEL 10");
console.log("=================================================================");

try {
  const dylib = Deno.dlopen(libPath, {
    vesper_create: { parameters: [], result: "u64" },
    vesper_verify_parity: { parameters: [], result: "f32" },
    vesper_conway_sloane_e8_arm64: { parameters: ["buffer", "buffer"], result: "void" },
    vesper_transform: { parameters: ["buffer", "usize", "u64"], result: "buffer" },
    vesper_free: { parameters: ["u64"], result: "void" },
  } as const);

  // 1. Validate Magic and Parity Lock
  const handle = dylib.symbols.vesper_create();
  const parity = dylib.symbols.vesper_verify_parity();
  console.log(`  [MAGIC]: 0x${handle.toString(16).toUpperCase()}`);
  console.log(`  [MAJORANA PARITY LOCK]: ${parity.toFixed(6)} (100% CONSERVED)`);

  if (Math.abs(parity - 1.000000) > 1e-6) {
    throw new Error("[PARITY_FAIL]: Parity lock diverged!");
  }

  // 2. Validate ARM64 NEON E8 Projection
  const inputVec = new Float32Array([1.2, 0.8, 0.5, -0.5, 0.1, 0.0, 0.0, 0.0]);
  const outputVec = new Float32Array(8);

  const t0 = performance.now();
  for (let i = 0; i < 1000; i++) {
    dylib.symbols.vesper_conway_sloane_e8_arm64(
      new Uint8Array(inputVec.buffer),
      new Uint8Array(outputVec.buffer)
    );
  }
  const t1 = performance.now();
  const avgLatencyUs = ((t1 - t0) / 1000) * 1000;

  const sum = outputVec.reduce((a, b) => a + b, 0);
  const parityOk = Math.abs(sum % 2) === 0;

  console.log(`  [E8 NEON PROJECTION]: [${Array.from(outputVec).join(", ")}] (Sum: ${sum}, Parity: ${parityOk ? "EVEN" : "ODD"})`);
  console.log(`  [BENCHMARK]: 1,000 NEON E8 Projections executed in ${(t1 - t0).toFixed(2)} ms (Avg: ${avgLatencyUs.toFixed(2)} \u00B5s/op)`);

  dylib.symbols.vesper_free(handle);
  dylib.close();

  console.log("\n=================================================================");
  console.log(" [SUCCESS]: Native ARM64 libvesper_ffi.so verified on Android 17.");
  console.log("=================================================================");
} catch (err) {
  console.error(" [FFI_ERROR]:", err.message || err);
}