#pragma once
#include <cstdint>
#include <immintrin.h>

// Computes 256D INT8 popcount Hamming distance between two 256-byte chunks using AVX2
inline uint32_t compute_rabitq_hamming_distance_avx2(const uint8_t* a, const uint8_t* b) {
    __m256i acc = _mm256_setzero_si256();
    
    for (int i = 0; i < 8; ++i) { // 8 * 32 = 256 bytes
        __m256i va = _mm256_loadu_si256(reinterpret_cast<const __m256i*>(a + i * 32));
        __m256i vb = _mm256_loadu_si256(reinterpret_cast<const __m256i*>(b + i * 32));
        __m256i diff = _mm256_xor_si256(va, vb);
        
        // Sum of absolute differences against zero to count bit differences
        __m256i sad = _mm256_sad_epu8(diff, _mm256_setzero_si256());
        acc = _mm256_add_epi64(acc, sad);
    }
    
    alignas(32) uint64_t result[4];
    _mm256_store_si256(reinterpret_cast<__m256i*>(result), acc);
    return static_cast<uint32_t>(result[0] + result[1] + result[2] + result[3]);
}