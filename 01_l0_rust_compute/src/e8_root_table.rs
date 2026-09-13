// e8_root_table.rs - Revision Omega.1
// Deterministic generator for all 240 roots of the E8 lattice.
// 112 vector roots: signed permutations of (1, 1, 0, 0, 0, 0, 0, 0)
// 128 spinor roots: eight halves (+-0.5) with an even number of minus signs.

use super::geo_semantic_tokenizer::{E8Point, LatticeRoot};

fn set_coord(p: &mut E8Point, idx: usize, v: f32) {
    match idx {
        0 => { p.c0 = v; }
        1 => { p.c1 = v; }
        2 => { p.c2 = v; }
        3 => { p.c3 = v; }
        4 => { p.c4 = v; }
        5 => { p.c5 = v; }
        6 => { p.c6 = v; }
        7 => { p.c7 = v; }
        _ => {}
    }
}

fn get_coord(p: &E8Point, idx: usize) -> f32 {
    match idx {
        0 => p.c0,
        1 => p.c1,
        2 => p.c2,
        3 => p.c3,
        4 => p.c4,
        5 => p.c5,
        6 => p.c6,
        7 => p.c7,
        _ => 0.0f32,
    }
}

fn count_bits(mut x: u32) -> u32 {
    let mut n = 0u32;
    while x > 0u32 {
        n = n + (x & 1u32);
        x = x >> 1u32;
    }
    n
}

fn tags_for(root_id: u16) -> (u8, u8, u8) {
    let r = root_id as u32;
    (((r % 8u32) + 1u32) as u8, 0u8, ((r % 3u32) + 1u32) as u8)
}

pub fn build_e8_root_table() -> Vec<LatticeRoot> {
    let mut roots = Vec::new();
    let mut root_id = 0u16;

    // 112 vector roots: 28 position pairs, 4 sign patterns each.
    let mut i = 0usize;
    while i < 8usize {
        let mut j = i + 1usize;
        while j < 8usize {
            let mut s = 0u32;
            while s < 4u32 {
                let vi = if (s & 1u32) == 0u32 { 1.0f32 } else { -1.0f32 };
                let vj = if (s & 2u32) == 0u32 { 1.0f32 } else { -1.0f32 };
                let mut p = E8Point::zero();
                set_coord(&mut p, i, vi);
                set_coord(&mut p, j, vj);
                let (cc, ds, rt) = tags_for(root_id);
                roots.push(LatticeRoot {
                    root_id,
                    coords: p,
                    color_charge: cc,
                    decay_state: ds,
                    role_tag: rt,
                });
                root_id = root_id + 1u16;
                s = s + 1u32;
            }
            j = j + 1usize;
        }
        i = i + 1usize;
    }

    // 128 spinor roots: even-parity sign masks over eight halves.
    let mut mask = 0u32;
    while mask < 256u32 {
        if count_bits(mask) % 2u32 == 0u32 {
            let mut p = E8Point::zero();
            let mut k = 0usize;
            while k < 8usize {
                let v = if ((mask >> (k as u32)) & 1u32) == 1u32 { -0.5f32 } else { 0.5f32 };
                set_coord(&mut p, k, v);
                k = k + 1usize;
            }
            let (cc, ds, rt) = tags_for(root_id);
            roots.push(LatticeRoot {
                root_id,
                coords: p,
                color_charge: cc,
                decay_state: ds,
                role_tag: rt,
            });
            root_id = root_id + 1u16;
        }
        mask = mask + 1u32;
    }
    roots
}

pub fn verify_e8_root_table(roots: &Vec<LatticeRoot>) -> bool {
    if roots.len() != 240 {
        return false;
    }
    let mut vector_n = 0u32;
    let mut spinor_n = 0u32;
    let mut idx = 0usize;
    while idx < roots.len() {
        let mut ok = false;
        if let Some(r) = roots.get(idx) {
            if (r.root_id as usize) == idx {
                let d = r.coords.norm_sq() - 2.0f32;
                let ad = if d < 0.0f32 { -d } else { d };
                if ad < 0.0001f32 {
                    let mut nz = 0u32;
                    let mut k = 0usize;
                    while k < 8usize {
                        let v = get_coord(&r.coords, k);
                        let av = if v < 0.0f32 { -v } else { v };
                        if av > 0.0001f32 {
                            nz = nz + 1;
                        }
                        k = k + 1usize;
                    }
                    if nz == 2u32 {
                        vector_n = vector_n + 1u32;
                        ok = true;
                    } else if nz == 8u32 {
                        spinor_n = spinor_n + 1u32;
                        ok = true;
                    }
                }
            }
        }
        if !ok {
            return false;
        }
        idx = idx + 1usize;
    }
    vector_n == 112u32 && spinor_n == 128u32
}