// Zero-Copy Deno FFI Bridge to vesper_cabi.dll

const libPath = "C:\\sovereign_manifold_v27\\02_l1_cpp23_cabi\\vesper_cabi.dll";

export const vesperSymbols = {
  vesper_create: { parameters: [], result: "u64" },
  vesper_verify_parity: { parameters: [], result: "f32" },
  vesper_transform: { parameters: ["buffer", "usize", "u64"], result: "buffer" },
  vesper_free: { parameters: ["u64"], result: "void" },
} as const;

export type VesperDylib = Deno.DynamicLibrary<typeof vesperSymbols>;

let dylib: VesperDylib | null = null;

export function getVesperDylib(): VesperDylib {
  if (!dylib) {
    dylib = Deno.dlopen(libPath, vesperSymbols);
  }
  return dylib;
}

export class VesperEngineHandle implements Disposable {
  public readonly handle: bigint;
  private lib: VesperDylib;

  constructor() {
    this.lib = getVesperDylib();
    this.handle = this.lib.symbols.vesper_create();
  }

  verifyParity(): number {
    return this.lib.symbols.vesper_verify_parity();
  }

  transformZeroCopy(buffer: Uint8Array): Uint8Array {
    const ptr = this.lib.symbols.vesper_transform(
      buffer,
      BigInt(buffer.byteLength),
      this.handle
    );
    if (!ptr) return buffer;
    return new Uint8Array(Deno.UnsafePointerView.getArrayBuffer(ptr, buffer.byteLength));
  }

  [Symbol.dispose](): void {
    this.lib.symbols.vesper_free(this.handle);
  }
}