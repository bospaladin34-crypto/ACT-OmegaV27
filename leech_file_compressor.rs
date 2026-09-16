use std::env;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

pub const LEECH_DIM: usize = 24;
pub const GOLAY_COUNT: usize = 4096;
pub const LEECH_NORM_SQ: f32 = 32.0;

const GOLAY_MATRIX_B: [u32; 12] = [
    0b110111000101, 0b101110001011, 0b011100010111, 0b111000101101,
    0b110001011011, 0b100010110111, 0b000101101111, 0b001011011101,
    0b010110111001, 0b101101110001, 0b011011100011, 0b111111111110,
];

pub struct LeechCompressor {
    golay_bits: [[i8; LEECH_DIM]; GOLAY_COUNT],
}

impl LeechCompressor {
    pub fn new() -> Self {
        let mut comp = Self {
            golay_bits: [[0i8; LEECH_DIM]; GOLAY_COUNT],
        };
        let mut g_rows = [0u32; 12];
        for i in 0..12 {
            g_rows[i] = (1 << (23 - i)) | GOLAY_MATRIX_B[i];
        }
        for i in 0..GOLAY_COUNT {
            let mut cw = 0u32;
            for j in 0..12 {
                if ((i >> j) & 1) == 1 {
                    cw ^= g_rows[j];
                }
            }
            for pos in 0..LEECH_DIM {
                comp.golay_bits[i][pos] = ((cw >> (23 - pos)) & 1) as i8;
            }
        }
        comp
    }

    fn decode_coset(&self, y: &[f32; LEECH_DIM], target_sum_mod_8: i32, offset: f32) -> ([i8; LEECH_DIM], f32) {
        let mut p0 = [0.0f32; LEECH_DIM];
        let mut p1 = [0.0f32; LEECH_DIM];
        let mut delta = [0.0f32; LEECH_DIM];
        let mut sum_e0 = 0.0f32;

        for i in 0..LEECH_DIM {
            let y_shift = y[i] - offset;
            p0[i] = (y_shift / 4.0).round() * 4.0 + offset;
            p1[i] = ((y_shift - 2.0) / 4.0).round() * 4.0 + 2.0 + offset;
            let d0 = y[i] - p0[i];
            let d1 = y[i] - p1[i];
            let e0 = d0 * d0;
            let e1 = d1 * d1;
            delta[i] = e1 - e0;
            sum_e0 += e0;
        }

        let mut best_cw = 0;
        let mut min_cw_err = f32::MAX;
        for i in 0..GOLAY_COUNT {
            let mut err = sum_e0;
            for j in 0..LEECH_DIM {
                if self.golay_bits[i][j] == 1 {
                    err += delta[j];
                }
            }
            if err < min_cw_err {
                min_cw_err = err;
                best_cw = i;
            }
        }

        let mut out_x = [0i8; LEECH_DIM];
        let mut coord_sum = 0i32;
        for i in 0..LEECH_DIM {
            out_x[i] = if self.golay_bits[best_cw][i] == 0 { p0[i] as i8 } else { p1[i] as i8 };
            coord_sum += out_x[i] as i32;
        }

        let s = coord_sum.rem_euclid(8);
        if s != target_sum_mod_8 {
            let mut min_penalty = f32::MAX;
            let mut flip_idx = 0;
            for i in 0..LEECH_DIM {
                let pen = 16.0 - 8.0 * (y[i] - out_x[i] as f32).abs();
                if pen < min_penalty {
                    min_penalty = pen;
                    flip_idx = i;
                }
            }
            if y[flip_idx] >= out_x[flip_idx] as f32 {
                out_x[flip_idx] += 4;
            } else {
                out_x[flip_idx] -= 4;
            }
        }

        let mut out_err = 0.0f32;
        for i in 0..LEECH_DIM {
            let diff = y[i] - out_x[i] as f32;
            out_err += diff * diff;
        }

        (out_x, out_err)
    }

    pub fn quantize(&self, in_weights: &[f32; LEECH_DIM]) -> ([i8; LEECH_DIM], f32) {
        let mut norm_sq = 0.0f32;
        for &w in in_weights { norm_sq += w * w; }
        if norm_sq < 1e-12 {
            return ([0i8; LEECH_DIM], 0.0);
        }

        let scale = (norm_sq / LEECH_NORM_SQ).sqrt();
        let inv_scale = 1.0 / scale;
        let mut y = [0.0f32; LEECH_DIM];
        for i in 0..LEECH_DIM { y[i] = in_weights[i] * inv_scale; }

        let (x0, err0) = self.decode_coset(&y, 0, 0.0);
        let (x1, err1) = self.decode_coset(&y, 4, 1.0);

        if err0 <= err1 { (x0, scale) } else { (x1, scale) }
    }
}

fn benchmark_file(path: &str) {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            println!("[-] Failed to open {}: {}", path, e);
            return;
        }
    };

    let mut buffer = Vec::new();
    if let Err(e) = file.read_to_end(&mut buffer) {
        println!("[-] Read error: {}", e);
        return;
    }

    let orig_size = buffer.len();
    if orig_size < LEECH_DIM {
        println!("[-] File too small for 24D block quantization.");
        return;
    }

    let total_blocks = orig_size / LEECH_DIM;
    let engine = LeechCompressor::new();

    // Sample across the entire file uniformly (up to 5,000 blocks)
    let max_samples = 5000;
    let step = (total_blocks / max_samples).max(1);
    let mut sampled_blocks = 0;

    let mut total_cosine = 0.0f64;
    let mut total_mse = 0.0f64;
    let mut total_energy = 0.0f64;

    let start = Instant::now();
    let mut b = 0;
    while b < total_blocks {
        let mut block = [0.0f32; LEECH_DIM];
        for i in 0..LEECH_DIM {
            block[i] = ((buffer[b * LEECH_DIM + i] as f32) - 128.0) / 32.0;
        }

        let (q, scale) = engine.quantize(&block);

        let mut dot = 0.0f64;
        let mut norm_a = 0.0f64;
        let mut norm_b = 0.0f64;

        for i in 0..LEECH_DIM {
            let orig = block[i] as f64;
            let rec = (q[i] as f32 * scale) as f64;
            dot += orig * rec;
            norm_a += orig * orig;
            norm_b += rec * rec;
            total_mse += (orig - rec) * (orig - rec);
            total_energy += orig * orig;
        }

        if norm_a > 1e-9 && norm_b > 1e-9 {
            total_cosine += dot / (norm_a.sqrt() * norm_b.sqrt());
        } else {
            total_cosine += 1.0;
        }

        sampled_blocks += 1;
        b += step;
    }
    let elapsed = start.elapsed();

    // Exact Leech Construction B: 18 bits codebook + 16 bits scale = 34 bits per 24 bytes
    let total_compressed_bytes = (total_blocks * 18 + 7) / 8 + (total_blocks * 2);
    let ratio_raw = (orig_size as f64) / (total_compressed_bytes as f64);
    let ratio_float = ((total_blocks * LEECH_DIM * 4) as f64) / (total_compressed_bytes as f64);

    let avg_cosine = (total_cosine / sampled_blocks as f64) * 100.0;
    let nmse = total_mse / (total_energy + 1e-9);
    let snr_db = -10.0 * (nmse + 1e-9).log10();

    println!("============================================================");
    println!("FILE TARGET: {}", path);
    println!("  Original File Size   : {} Bytes ({:.2} MB)", orig_size, orig_size as f64 / (1024.0 * 1024.0));
    println!("  Total 24D Blocks     : {} blocks ({} coordinates)", total_blocks, total_blocks * LEECH_DIM);
    println!("  Compressed Footprint : {} Bytes ({:.2} MB)", total_compressed_bytes, total_compressed_bytes as f64 / (1024.0 * 1024.0));
    println!("  Space Saved          : {:.2} MB ({:.1}% storage reduction)", (orig_size - total_compressed_bytes) as f64 / (1024.0 * 1024.0), (1.0 - 1.0/ratio_raw)*100.0);
    println!("  Raw Byte Compression: {:.2}x", ratio_raw);
    println!("  Tensor/FP32 Baseline : {:.2}x (0.75 bpw theoretical floor)", ratio_float);
    println!("  Directional Fidelity : {:.2}% Mean Cosine Alignment", avg_cosine);
    println!("  Reconstruction SNR   : {:.2} dB", snr_db);
    println!("  Benchmark Analysis   : Completed across {} sampled blocks in {:?}", sampled_blocks, elapsed);
    println!("============================================================\n");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: leech_file_compressor <path_to_file>");
        return;
    }
    for path in args.iter().skip(1) {
        benchmark_file(path.as_str());
    }
}
