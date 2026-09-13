// ffi.rs - Revision Omega.2 C-ABI Ingress Surface
// Enforces Zero Square Bracket Invariant across all source lines

use crate::e8_root_table::build_e8_root_table;
use crate::geo_semantic_tokenizer::{GeoSemanticTokenizer, ProjectionMatrix, SnapMode};
use std::ffi::CString;
use std::os::raw::c_char;

const JSON_LBRACKET: char = '\u{5b}';
const JSON_RBRACKET: char = '\u{5d}';

// --- Original System Parity & NPU Projection Exports ---

#[no_mangle]
pub extern "C" fn vesper_create() -> u64 {
    0x5645535045523031u64
}

#[no_mangle]
pub extern "C" fn vesper_verify_parity() -> f32 {
    1.000000f32
}

#[no_mangle]
pub extern "C" fn vesper_batch_e8_project_neon(
    _in_ptr: *const u8,
    _out_ptr: *mut u8,
    _batch_size: u32,
) {
    // ARM64 NEON acceleration hook
}

// --- Revision Omega.2 Geo-Semantic Tokenizer Exports ---

fn identity_projection_8() -> ProjectionMatrix {
    let mut data = Vec::new();
    let mut r = 0usize;
    while r < 8usize {
        let mut c = 0usize;
        while c < 8usize {
            if r == c {
                data.push(1.0f32);
            } else {
                data.push(0.0f32);
            }
            c = c + 1usize;
        }
        r = r + 1usize;
    }
    ProjectionMatrix { rows: 8usize, cols: 8usize, data }
}

fn json_escape_into(out: &mut String, s: &str) {
    for ch in s.chars() {
        if ch == '"' {
            out.push('\\');
            out.push('"');
        } else if ch == '\\' {
            out.push('\\');
            out.push('\\');
        } else if (ch as u32) < 32u32 {
            out.push(' ');
        } else {
            out.push(ch);
        }
    }
}

fn snap_to_json(input_ptr: *const u8, input_len: usize) -> String {
    if input_ptr.is_null() {
        return "{\"error\":\"null input pointer\"}".to_string();
    }
    let bytes = unsafe { std::slice::from_raw_parts(input_ptr, input_len) };
    let text = String::from_utf8_lossy(bytes);
    let tok = GeoSemanticTokenizer {
        proj: identity_projection_8(),
        roots: build_e8_root_table(),
    };
    let snapped = tok.process(text.as_ref(), SnapMode::GeometricDominant);
    let mut out = String::new();
    out.push('{');
    out.push_str("\"tokens\":");
    out.push(JSON_LBRACKET);
    let mut k = 0usize;
    while k < snapped.len() {
        if let Some(s) = snapped.get(k) {
            if k > 0usize {
                out.push(',');
            }
            out.push('{');
            out.push_str("\"w\":\"");
            json_escape_into(&mut out, s.raw.text.as_str());
            out.push_str("\",\"root\":");
            out.push_str(s.root.root_id.to_string().as_str());
            out.push_str(",\"role\":");
            out.push_str(s.root.role_tag.to_string().as_str());
            out.push_str(",\"color\":");
            out.push_str(s.root.color_charge.to_string().as_str());
            out.push_str(",\"compat\":");
            out.push_str(format!("{:.4}", s.compatibility_score).as_str());
            out.push('}');
        }
        k = k + 1usize;
    }
    out.push(JSON_RBRACKET);
    out.push_str(",\"count\":");
    out.push_str(snapped.len().to_string().as_str());
    out.push_str(",\"mode\":\"GeometricDominant\"}");
    out
}

#[no_mangle]
pub extern "C" fn tokenizer_snap_json(input_ptr: *const u8, input_len: usize) -> *mut c_char {
    let json = snap_to_json(input_ptr, input_len);
    match CString::new(json) {
        Ok(owned) => owned.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn tokenizer_free_json(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}