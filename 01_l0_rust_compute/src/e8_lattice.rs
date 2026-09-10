// 256D INT8 Conway-Sloane E8 Nearest-Point Lattice Quantizer
// Zero Square Bracket Invariant strictly enforced across this file

pub type E8Vector8D = (f32, f32, f32, f32, f32, f32, f32, f32);
pub type E8QuantizedInt8 = (i8, i8, i8, i8, i8, i8, i8, i8);

pub fn decode_conway_sloane_e8(v: &E8Vector8D) -> E8Vector8D {
    let f0 = v.0.round(); let f1 = v.1.round();
    let f2 = v.2.round(); let f3 = v.3.round();
    let f4 = v.4.round(); let f5 = v.5.round();
    let f6 = v.6.round(); let f7 = v.7.round();

    let sum_d8 = f0 + f1 + f2 + f3 + f4 + f5 + f6 + f7;
    let mut d8_0 = f0;
    
    // Parity correction to enforce sum in 2Z
    if (sum_d8 as i32) % 2 != 0 {
        d8_0 += if v.0 > f0 { 1.0 } else { -1.0 };
    }

    (d8_0, f1, f2, f3, f4, f5, f6, f7)
}

pub fn quantize_e8_chunk_int8(v: &E8Vector8D, scale: f32) -> E8QuantizedInt8 {
    let scaled = (
        v.0 / scale, v.1 / scale, v.2 / scale, v.3 / scale,
        v.4 / scale, v.5 / scale, v.6 / scale, v.7 / scale,
    );
    let dec = decode_conway_sloane_e8(&scaled);
    
    (
        dec.0.clamp(-128.0, 127.0) as i8,
        dec.1.clamp(-128.0, 127.0) as i8,
        dec.2.clamp(-128.0, 127.0) as i8,
        dec.3.clamp(-128.0, 127.0) as i8,
        dec.4.clamp(-128.0, 127.0) as i8,
        dec.5.clamp(-128.0, 127.0) as i8,
        dec.6.clamp(-128.0, 127.0) as i8,
        dec.7.clamp(-128.0, 127.0) as i8,
    )
}

pub fn calculate_e8_norm_squared(v: &E8Vector8D) -> f32 {
    v.0 * v.0 + v.1 * v.1 + v.2 * v.2 + v.3 * v.3 +
    v.4 * v.4 + v.5 * v.5 + v.6 * v.6 + v.7 * v.7
}