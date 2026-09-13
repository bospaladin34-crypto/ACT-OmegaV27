// tokenizer_bridge.ts - Deno FFI bridge to real L0 Rust geo-semantic tokenizer.
const rustLibPath = "C:\\sovereign_manifold_v27\\01_l0_rust_compute\\target\\release\\vesper_ffi.dll";
const rustSymbols = {
  tokenizer_snap_json: { parameters: ["buffer", "usize"], result: "pointer" },
  tokenizer_free_json: { parameters: ["pointer"], result: "void" },
} as const;

type RustDylib = Deno.DynamicLibrary<typeof rustSymbols>;
let dylib: RustDylib | null = null;

function getRustDylib(): RustDylib | null {
  if (!dylib) {
    try {
      dylib = Deno.dlopen(rustLibPath, rustSymbols);
    } catch (_) {
      dylib = null;
    }
  }
  return dylib;
}

export interface SnappedWord {
  w: string;
  root: number;
  role: number;
  color: number;
  compat: number;
}

export function snapText(text: string): SnappedWord[] {
  const lib = getRustDylib();
  if (!lib) return [];
  const bytes = new TextEncoder().encode(text);
  const ptr = lib.symbols.tokenizer_snap_json(bytes, BigInt(bytes.byteLength));
  if (!ptr) return [];
  try {
    const json = new Deno.UnsafePointerView(ptr).getCString();
    const parsed = JSON.parse(json) as { tokens?: SnappedWord[] };
    if (!parsed || !Array.isArray(parsed.tokens)) return [];
    return parsed.tokens;
  } finally {
    lib.symbols.tokenizer_free_json(ptr);
  }
}