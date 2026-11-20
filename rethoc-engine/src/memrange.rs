//! SIMD-accelerated byte-range search.
//!
//! `memrange(lo, hi, haystack)` finds the index of the first byte `b` in
//! `haystack` satisfying `lo <= b <= hi`.
//! The pair prefilter compares adjacent bytes against two ranges, retaining
//! a candidate at the end of a chunk when its successor is still unknown.
//!
//! **Core insight**: `clamp(x, lo, hi) == x` iff `lo <= x <= hi`.
//! In vector ops: `cmpeq(min(max(x, lo), hi), x)` — 3 SIMD ops per chunk.
//!
//! Provides SSE2/AVX2 paths on x86_64, NEON on aarch64, and a scalar
//! fallback for other targets or short haystacks.

/// Find the first byte in `haystack` in the inclusive range `[lo, hi]`.
///
/// Returns `None` if `lo > hi`, the haystack is empty, or no byte is in range.
///
/// Uses SIMD acceleration (NEON on aarch64, SSE2/AVX2 on x86_64) when
/// available, falling back to a scalar loop for short haystacks or
/// unsupported targets.
#[doc(hidden)]
#[inline]
pub fn memrange(lo: u8, hi: u8, haystack: &[u8]) -> Option<usize> {
    if lo > hi || haystack.is_empty() {
        return None;
    }
    imp::memrange(lo, hi, haystack)
}

/// Find the first possible start of two adjacent inclusive ranges. A final
/// byte in the first range is a candidate even without its successor, so a
/// streaming caller can consume it before the next chunk arrives.
#[inline]
pub(crate) fn memrange_pair(
    lo: u8,
    hi: u8,
    next_lo: u8,
    next_hi: u8,
    haystack: &[u8],
) -> Option<usize> {
    debug_assert!(lo <= hi && next_lo <= next_hi);
    imp::memrange_pair(lo, hi, next_lo, next_hi, haystack)
}

#[inline]
fn memrange_pair_fallback(
    lo: u8,
    hi: u8,
    next_lo: u8,
    next_hi: u8,
    haystack: &[u8],
) -> Option<usize> {
    let mut rest = haystack;
    while let Some((&byte, tail)) = rest.split_first() {
        if (lo..=hi).contains(&byte)
            && tail
                .first()
                .is_none_or(|next| (next_lo..=next_hi).contains(next))
        {
            return Some(haystack.len() - rest.len());
        }
        rest = tail;
    }
    None
}

/// Scalar fallback: simple byte-at-a-time loop.
#[inline]
fn memrange_fallback(lo: u8, hi: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().position(|&b| b >= lo && b <= hi)
}

/// Scalar byte-at-a-time implementation, exposed for benchmarking against
/// the SIMD-accelerated [`memrange`].
#[doc(hidden)]
#[inline]
pub fn memrange_scalar(lo: u8, hi: u8, haystack: &[u8]) -> Option<usize> {
    if lo > hi || haystack.is_empty() {
        return None;
    }
    memrange_fallback(lo, hi, haystack)
}

// ---------------------------------------------------------------------------
// x86_64: SSE2 (16 B) + AVX2 (32 B)
// ---------------------------------------------------------------------------
#[cfg(target_arch = "x86_64")]
mod imp {
    use super::{memrange_fallback, memrange_pair_fallback};
    use core::arch::x86_64::*;

    #[inline]
    pub(super) fn memrange(lo: u8, hi: u8, haystack: &[u8]) -> Option<usize> {
        // Short skips are common when a range is used as a start prefilter.
        // Check the first vector before dispatching to the long scan.
        let Some((first, rest)) = haystack.split_first_chunk::<SSE2_WIDTH>() else {
            return memrange_fallback(lo, hi, haystack);
        };
        // SAFETY: SSE2 is baseline on x86_64. The array holds a full vector
        // and loadu does not require it to be aligned.
        let bits = unsafe {
            let chunk = _mm_loadu_si128(first.as_ptr().cast());
            test_sse2(chunk, _mm_set1_epi8(lo as i8), _mm_set1_epi8(hi as i8))
        };
        if bits != 0 {
            return Some(bits.trailing_zeros() as usize);
        }

        let offset = if rest.len() >= AVX2_WIDTH && is_x86_feature_detected!("avx2") {
            // SAFETY: AVX2 detected at runtime.
            unsafe { memrange_avx2(lo, hi, rest) }
        } else {
            // SAFETY: SSE2 is baseline on x86_64.
            unsafe { memrange_sse2(lo, hi, rest) }
        };
        offset.map(|i| SSE2_WIDTH + i)
    }

    #[inline]
    pub(super) fn memrange_pair(
        lo: u8,
        hi: u8,
        next_lo: u8,
        next_hi: u8,
        haystack: &[u8],
    ) -> Option<usize> {
        if haystack.len() > AVX2_WIDTH && is_x86_feature_detected!("avx2") {
            // SAFETY: AVX2 was detected at runtime.
            return unsafe { memrange_pair_avx2(lo, hi, next_lo, next_hi, haystack) };
        }
        memrange_pair_sse2(lo, hi, next_lo, next_hi, haystack)
    }

    #[inline]
    pub(super) fn memrange_pair_sse2(
        lo: u8,
        hi: u8,
        next_lo: u8,
        next_hi: u8,
        haystack: &[u8],
    ) -> Option<usize> {
        let mut offset = 0;
        while haystack.len() - offset > SSE2_WIDTH {
            // SAFETY: SSE2 is baseline on x86_64 and both unaligned loads
            // fit in the SSE2_WIDTH + 1 remaining bytes.
            let bits = unsafe {
                let ptr = haystack.as_ptr().add(offset);
                let first = _mm_loadu_si128(ptr.cast());
                let second = _mm_loadu_si128(ptr.add(1).cast());
                test_sse2(first, _mm_set1_epi8(lo as i8), _mm_set1_epi8(hi as i8))
                    & test_sse2(
                        second,
                        _mm_set1_epi8(next_lo as i8),
                        _mm_set1_epi8(next_hi as i8),
                    )
            };
            if bits != 0 {
                return Some(offset + bits.trailing_zeros() as usize);
            }
            offset += SSE2_WIDTH;
        }
        memrange_pair_fallback(lo, hi, next_lo, next_hi, &haystack[offset..]).map(|i| offset + i)
    }

    #[target_feature(enable = "avx2")]
    unsafe fn memrange_pair_avx2(
        lo: u8,
        hi: u8,
        next_lo: u8,
        next_hi: u8,
        haystack: &[u8],
    ) -> Option<usize> {
        let mut offset = 0;
        while haystack.len() - offset > AVX2_WIDTH {
            // SAFETY: AVX2 is enabled and both unaligned loads fit in the
            // AVX2_WIDTH + 1 remaining bytes.
            let bits = unsafe {
                let ptr = haystack.as_ptr().add(offset);
                let first = _mm256_loadu_si256(ptr.cast());
                let second = _mm256_loadu_si256(ptr.add(1).cast());
                test_avx2(
                    first,
                    _mm256_set1_epi8(lo as i8),
                    _mm256_set1_epi8(hi as i8),
                ) & test_avx2(
                    second,
                    _mm256_set1_epi8(next_lo as i8),
                    _mm256_set1_epi8(next_hi as i8),
                )
            };
            if bits != 0 {
                return Some(offset + bits.trailing_zeros() as usize);
            }
            offset += AVX2_WIDTH;
        }
        memrange_pair_fallback(lo, hi, next_lo, next_hi, &haystack[offset..]).map(|i| offset + i)
    }

    // -- SSE2 (16 bytes per iteration) --

    const SSE2_WIDTH: usize = 16;

    #[target_feature(enable = "sse2")]
    unsafe fn memrange_sse2(lo: u8, hi: u8, haystack: &[u8]) -> Option<usize> {
        let len = haystack.len();
        if len < SSE2_WIDTH {
            return memrange_fallback(lo, hi, haystack);
        }

        // SAFETY: SSE2 is enabled on this function; every load stays within `haystack` (len >= 16): the first and last loads cover [0, 16) and [len-16, len), aligned loads stay below `aligned_end`.
        unsafe {
            let ptr = haystack.as_ptr();
            let v_lo = _mm_set1_epi8(lo as i8);
            let v_hi = _mm_set1_epi8(hi as i8);

            // First unaligned load: [0, 16)
            let chunk = _mm_loadu_si128(ptr as *const __m128i);
            let bits = test_sse2(chunk, v_lo, v_hi);
            if bits != 0 {
                return Some(bits.trailing_zeros() as usize);
            }

            // Aligned middle loop.
            let aligned_start = (ptr as usize + SSE2_WIDTH) & !(SSE2_WIDTH - 1);
            let aligned_end = (ptr as usize + len) & !(SSE2_WIDTH - 1);
            let mut p = aligned_start as *const __m128i;
            let p_end = aligned_end as *const __m128i;

            while p < p_end {
                let chunk = _mm_load_si128(p);
                let bits = test_sse2(chunk, v_lo, v_hi);
                if bits != 0 {
                    let offset = (p as usize) - (ptr as usize);
                    return Some(offset + bits.trailing_zeros() as usize);
                }
                p = p.add(1);
            }

            // Final unaligned load: [len-16, len)
            let chunk = _mm_loadu_si128(ptr.add(len - SSE2_WIDTH) as *const __m128i);
            let bits = test_sse2(chunk, v_lo, v_hi);
            if bits != 0 {
                return Some(len - SSE2_WIDTH + bits.trailing_zeros() as usize);
            }

            None
        }
    }

    /// Test 16 bytes: returns a bitmask where bit `i` is set if byte `i`
    /// is in `[lo, hi]`.
    #[inline(always)]
    unsafe fn test_sse2(chunk: __m128i, v_lo: __m128i, v_hi: __m128i) -> i32 {
        // SAFETY: the caller has the required target feature enabled; these are register-only intrinsics.
        unsafe {
            let clamped = _mm_min_epu8(_mm_max_epu8(chunk, v_lo), v_hi);
            _mm_movemask_epi8(_mm_cmpeq_epi8(clamped, chunk))
        }
    }

    // -- AVX2 (32 bytes per iteration) --

    const AVX2_WIDTH: usize = 32;

    #[target_feature(enable = "avx2")]
    unsafe fn memrange_avx2(lo: u8, hi: u8, haystack: &[u8]) -> Option<usize> {
        let len = haystack.len();
        if len < AVX2_WIDTH {
            // Fall through to SSE2 for 16–31 byte inputs.
            // SAFETY: AVX2 implies SSE2 and SSSE3.
            return unsafe { memrange_sse2(lo, hi, haystack) };
        }

        // SAFETY: AVX2 is enabled on this function; every load stays within `haystack` (len >= 32): the first and last loads cover [0, 32) and [len-32, len), aligned loads stay below `aligned_end`.
        unsafe {
            let ptr = haystack.as_ptr();
            let v_lo = _mm256_set1_epi8(lo as i8);
            let v_hi = _mm256_set1_epi8(hi as i8);

            // First unaligned load: [0, 32)
            let chunk = _mm256_loadu_si256(ptr as *const __m256i);
            let bits = test_avx2(chunk, v_lo, v_hi);
            if bits != 0 {
                return Some(bits.trailing_zeros() as usize);
            }

            // Aligned middle loop.
            let aligned_start = (ptr as usize + AVX2_WIDTH) & !(AVX2_WIDTH - 1);
            let aligned_end = (ptr as usize + len) & !(AVX2_WIDTH - 1);
            let mut p = aligned_start as *const __m256i;
            let p_end = aligned_end as *const __m256i;

            while p < p_end {
                let chunk = _mm256_load_si256(p);
                let bits = test_avx2(chunk, v_lo, v_hi);
                if bits != 0 {
                    let offset = (p as usize) - (ptr as usize);
                    return Some(offset + bits.trailing_zeros() as usize);
                }
                p = p.add(1);
            }

            // Final unaligned load: [len-32, len)
            let chunk = _mm256_loadu_si256(ptr.add(len - AVX2_WIDTH) as *const __m256i);
            let bits = test_avx2(chunk, v_lo, v_hi);
            if bits != 0 {
                return Some(len - AVX2_WIDTH + bits.trailing_zeros() as usize);
            }

            None
        }
    }

    /// Test 32 bytes: returns a bitmask where bit `i` is set if byte `i`
    /// is in `[lo, hi]`.
    #[inline(always)]
    unsafe fn test_avx2(chunk: __m256i, v_lo: __m256i, v_hi: __m256i) -> i32 {
        // SAFETY: the caller has the required target feature enabled; these are register-only intrinsics.
        unsafe {
            let clamped = _mm256_min_epu8(_mm256_max_epu8(chunk, v_lo), v_hi);
            _mm256_movemask_epi8(_mm256_cmpeq_epi8(clamped, chunk))
        }
    }
}

// ---------------------------------------------------------------------------
// aarch64: NEON (16 B)
// ---------------------------------------------------------------------------
// Following memchr's approach: NEON is mandatory in ARMv8-A and enabled by
// default in all standard Rust aarch64 targets.  Use compile-time
// `target_feature = "neon"` (not runtime detection).  Exotic targets
// compiled without NEON fall through to the scalar fallback.
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
mod imp {
    use super::{memrange_fallback, memrange_pair_fallback};
    use core::arch::aarch64::*;

    const NEON_WIDTH: usize = 16;

    #[inline]
    pub(super) fn memrange(lo: u8, hi: u8, haystack: &[u8]) -> Option<usize> {
        // SAFETY: NEON availability guaranteed by cfg(target_feature = "neon").
        unsafe { memrange_neon(lo, hi, haystack) }
    }

    #[inline]
    pub(super) fn memrange_pair(
        lo: u8,
        hi: u8,
        next_lo: u8,
        next_hi: u8,
        haystack: &[u8],
    ) -> Option<usize> {
        // Packing byte flags into nibbles below assumes little-endian lanes.
        if cfg!(target_endian = "big") {
            return memrange_pair_fallback(lo, hi, next_lo, next_hi, haystack);
        }
        let Some(last) = haystack.len().checked_sub(NEON_WIDTH) else {
            return memrange_pair_fallback(lo, hi, next_lo, next_hi, haystack);
        };
        // SAFETY: NEON is enabled by this module's cfg. While offset < last
        // at least NEON_WIDTH + 1 bytes remain, so both unaligned loads fit.
        // The final load covers exactly the last NEON_WIDTH bytes.
        unsafe {
            let lo_vec = vdupq_n_u8(lo);
            let span = vdupq_n_u8(hi - lo);
            let next_lo_vec = vdupq_n_u8(next_lo);
            let next_span = vdupq_n_u8(next_hi - next_lo);
            let ptr = haystack.as_ptr();
            let mut offset = 0;
            while offset < last {
                let first = vld1q_u8(ptr.add(offset));
                let second = vld1q_u8(ptr.add(offset + 1));
                // Unsigned wrapping subtraction maps the inclusive range to
                // zero through its span, including when the range crosses 127.
                let a = vcleq_u8(vsubq_u8(first, lo_vec), span);
                let b = vcleq_u8(vsubq_u8(second, next_lo_vec), next_span);
                if let Some(lane) = first_pair_lane(vandq_u8(a, b)) {
                    return Some(offset + lane);
                }
                offset += NEON_WIDTH;
            }

            // Overlap the last block to avoid a scalar tail on short lines.
            // Earlier starts in the overlap have already been ruled out.
            // Shifting the final B mask leaves an unconditional B in lane
            // 15, retaining the possible cross-chunk start at the last byte.
            let tail = vld1q_u8(ptr.add(last));
            let a = vcleq_u8(vsubq_u8(tail, lo_vec), span);
            let b = vcleq_u8(vsubq_u8(tail, next_lo_vec), next_span);
            let next = vextq_u8::<1>(b, vdupq_n_u8(0xff));
            first_pair_lane(vandq_u8(a, next)).map(|lane| last + lane)
        }
    }

    /// Pack the sixteen byte flags into nibbles in one scalar register.
    #[inline(always)]
    unsafe fn first_pair_lane(mask: uint8x16_t) -> Option<usize> {
        // SAFETY: NEON is enabled by this module's cfg; only register lanes are accessed.
        unsafe {
            let nibbles = vshrn_n_u16::<4>(vreinterpretq_u16_u8(mask));
            let bits = vget_lane_u64::<0>(vreinterpret_u64_u8(nibbles));
            (bits != 0).then(|| bits.trailing_zeros() as usize / 4)
        }
    }

    /// NEON: 16 bytes per iteration.
    ///
    /// Match extraction uses u64 lane inspection instead of a movemask
    /// (NEON has no direct movemask equivalent).  Each matched byte is
    /// 0xFF in the comparison result; we extract two u64 halves and use
    /// `trailing_zeros() / 8` to find the first set byte.
    #[target_feature(enable = "neon")]
    unsafe fn memrange_neon(lo: u8, hi: u8, haystack: &[u8]) -> Option<usize> {
        let len = haystack.len();
        if len < NEON_WIDTH {
            return memrange_fallback(lo, hi, haystack);
        }

        unsafe {
            let ptr = haystack.as_ptr();
            let v_lo = vdupq_n_u8(lo);
            let v_hi = vdupq_n_u8(hi);

            // First load: [0, 16)
            let chunk = vld1q_u8(ptr);
            if let Some(pos) = test_neon(chunk, v_lo, v_hi) {
                return Some(pos);
            }

            // Aligned middle loop.
            let aligned_start = (ptr as usize + NEON_WIDTH) & !(NEON_WIDTH - 1);
            let aligned_end = (ptr as usize + len) & !(NEON_WIDTH - 1);
            let mut p = aligned_start as *const u8;
            let p_end = aligned_end as *const u8;

            while p < p_end {
                let chunk = vld1q_u8(p);
                if let Some(pos) = test_neon(chunk, v_lo, v_hi) {
                    let offset = (p as usize) - (ptr as usize);
                    return Some(offset + pos);
                }
                p = p.add(NEON_WIDTH);
            }

            // Final load: [len-16, len)
            let chunk = vld1q_u8(ptr.add(len - NEON_WIDTH));
            if let Some(pos) = test_neon(chunk, v_lo, v_hi) {
                return Some(len - NEON_WIDTH + pos);
            }

            None
        }
    }

    /// Test 16 bytes: returns `Some(pos)` of the first byte in `[lo, hi]`,
    /// or `None` if no byte matches.
    #[inline(always)]
    unsafe fn test_neon(chunk: uint8x16_t, v_lo: uint8x16_t, v_hi: uint8x16_t) -> Option<usize> {
        unsafe {
            let clamped = vminq_u8(vmaxq_u8(chunk, v_lo), v_hi);
            let eq = vceqq_u8(clamped, chunk);

            // Extract first match position via u64 lane inspection.
            // Each matching byte is 0xFF (all bits set), non-matching is 0x00.
            // On little-endian aarch64, byte 0 occupies the LSB of lane 0.
            let eq_u64 = vreinterpretq_u64_u8(eq);
            let lo64 = vgetq_lane_u64::<0>(eq_u64);
            if lo64 != 0 {
                return Some((lo64.trailing_zeros() / 8) as usize);
            }
            let hi64 = vgetq_lane_u64::<1>(eq_u64);
            if hi64 != 0 {
                return Some(8 + (hi64.trailing_zeros() / 8) as usize);
            }
            None
        }
    }
}

// ---------------------------------------------------------------------------
// Fallback for other architectures (or aarch64 without NEON)
// ---------------------------------------------------------------------------
#[cfg(not(any(
    target_arch = "x86_64",
    all(target_arch = "aarch64", target_feature = "neon")
)))]
mod imp {
    use super::{memrange_fallback, memrange_pair_fallback};

    #[inline]
    pub(super) fn memrange(lo: u8, hi: u8, haystack: &[u8]) -> Option<usize> {
        memrange_fallback(lo, hi, haystack)
    }

    #[inline]
    pub(super) fn memrange_pair(
        lo: u8,
        hi: u8,
        next_lo: u8,
        next_hi: u8,
        haystack: &[u8],
    ) -> Option<usize> {
        memrange_pair_fallback(lo, hi, next_lo, next_hi, haystack)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::{memrange, memrange_pair, memrange_pair_fallback};

    fn check_pair(lo: u8, hi: u8, next_lo: u8, next_hi: u8, hay: &[u8]) {
        let expected = hay
            .windows(2)
            .position(|pair| (lo..=hi).contains(&pair[0]) && (next_lo..=next_hi).contains(&pair[1]))
            .or_else(|| {
                hay.last()
                    .filter(|b| (lo..=hi).contains(b))
                    .map(|_| hay.len() - 1)
            });
        let check = |name, result| {
            assert_eq!(
                expected, result,
                "{name}({lo}, {hi}, {next_lo}, {next_hi}, {hay:?})"
            )
        };
        check("dispatch", memrange_pair(lo, hi, next_lo, next_hi, hay));
        check(
            "scalar",
            memrange_pair_fallback(lo, hi, next_lo, next_hi, hay),
        );
        #[cfg(target_arch = "x86_64")]
        check(
            "sse2",
            super::imp::memrange_pair_sse2(lo, hi, next_lo, next_hi, hay),
        );
    }

    #[test]
    fn pairs_at_vector_boundaries() {
        let mut buf = [0; 128];
        for shift in 0..32 {
            for len in [0, 1, 2, 15, 16, 17, 31, 32, 33, 63, 64, 65] {
                let hay = &mut buf[shift..shift + len];
                hay.fill(b'x');
                check_pair(b'0', b'9', b'/', b'9', hay);
                for i in 0..len {
                    // First range alone, possibly at the unconfirmed final byte.
                    hay[i] = b'0';
                    check_pair(b'0', b'9', b'/', b'9', hay);
                    if i + 1 < len {
                        hay[i + 1] = b'/';
                        check_pair(b'0', b'9', b'/', b'9', hay);
                        hay[i + 1] = b'x';
                    }
                    hay[i] = b'x';
                }
            }
        }
    }

    #[test]
    fn pair_ranges_agree_with_specification() {
        let mut buf = [0; 256];
        let mut state = 0xDEAD_BEEFu32;
        for b in &mut buf {
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            *b = (state >> 16) as u8;
        }
        for (lo, hi, next_lo, next_hi) in [
            (0, 0, 255, 255),
            (128, 255, 0, 127),
            (0, 255, 127, 128),
            (127, 128, 0, 255),
            (0, 31, 10, 40),
        ] {
            for start in 0..32 {
                for len in [0, 1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 128, 192] {
                    check_pair(lo, hi, next_lo, next_hi, &buf[start..start + len]);
                }
            }
        }
        // A and B occur in unrelated lanes before two overlapping real
        // pairs and a final A. Only the first real pair should be returned.
        buf.fill(b'x');
        buf[3] = b'0';
        buf[8] = b'/';
        buf[31..34].copy_from_slice(b"987");
        buf[255] = b'9';
        check_pair(b'0', b'9', b'/', b'9', &buf);
    }

    #[test]
    fn empty_haystack() {
        assert_eq!(memrange(0, 255, &[]), None);
    }

    #[test]
    fn inverted_range() {
        assert_eq!(memrange(10, 5, &[7]), None);
    }

    #[test]
    fn single_byte_exact() {
        assert_eq!(memrange(5, 5, &[5]), Some(0));
        assert_eq!(memrange(5, 5, &[4]), None);
        assert_eq!(memrange(5, 5, &[6]), None);
    }

    #[test]
    fn digit_range() {
        assert_eq!(memrange(b'0', b'9', b"hello world 42!"), Some(12));
    }

    #[test]
    fn no_match() {
        let hay = vec![b'A'; 128];
        assert_eq!(memrange(b'0', b'9', &hay), None);
    }

    #[test]
    fn match_first_byte() {
        assert_eq!(memrange(b'0', b'9', b"5hello"), Some(0));
    }

    #[test]
    fn match_last_byte() {
        assert_eq!(memrange(b'0', b'9', b"hello5"), Some(5));
    }

    #[test]
    fn full_range() {
        assert_eq!(memrange(0, 255, &[42]), Some(0));
    }

    #[test]
    fn boundary_values() {
        // lo and hi themselves must match.
        assert_eq!(memrange(10, 20, &[10]), Some(0));
        assert_eq!(memrange(10, 20, &[20]), Some(0));
        assert_eq!(memrange(10, 20, &[9]), None);
        assert_eq!(memrange(10, 20, &[21]), None);
    }

    #[test]
    fn high_byte_range() {
        // Range in the upper half of u8.
        let hay = [0x00, 0x7F, 0x80, 0xFE, 0xFF];
        assert_eq!(memrange(0x80, 0xFF, &hay), Some(2));
        assert_eq!(memrange(0xFE, 0xFF, &hay), Some(3));
        assert_eq!(memrange(0xFF, 0xFF, &hay), Some(4));
    }

    #[test]
    fn various_lengths() {
        // Test lengths around SIMD boundaries (16, 32).
        for len in 1..=128 {
            let mut hay = vec![b'A'; len];
            // No match.
            assert_eq!(memrange(b'0', b'9', &hay), None, "len={len}");
            // Match at last position.
            hay[len - 1] = b'5';
            assert_eq!(memrange(b'0', b'9', &hay), Some(len - 1), "len={len}");
            // Match at first position.
            hay[0] = b'3';
            assert_eq!(memrange(b'0', b'9', &hay), Some(0), "len={len}");
        }
    }

    #[test]
    fn alignment_offsets() {
        // Test at every alignment offset within a large buffer.
        let mut buf = vec![b'A'; 256 + 64];
        for offset in 0..64 {
            let hay = &mut buf[offset..offset + 128];
            for b in hay.iter_mut() {
                *b = b'A';
            }
            for pos in [0, 1, 15, 16, 17, 31, 32, 33, 63, 64, 127] {
                hay[pos] = b'5';
                assert_eq!(
                    memrange(b'0', b'9', hay),
                    Some(pos),
                    "offset={offset}, pos={pos}",
                );
                hay[pos] = b'A';
            }
        }
    }

    #[test]
    fn compare_with_scalar() {
        // Compare SIMD result against a known-correct scalar implementation.
        let scalar = |lo: u8, hi: u8, hay: &[u8]| -> Option<usize> {
            hay.iter().position(|&b| b >= lo && b <= hi)
        };

        // Deterministic pseudo-random data (LCG).
        let mut data = vec![0u8; 1024];
        let mut state: u32 = 0xDEAD_BEEF;
        for b in data.iter_mut() {
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            *b = (state >> 16) as u8;
        }

        let ranges: &[(u8, u8)] = &[
            (b'0', b'9'),
            (b'a', b'z'),
            (b'A', b'F'),
            (0, 31),
            (128, 255),
            (100, 100),
            (0, 0),
            (255, 255),
        ];

        for &(lo, hi) in ranges {
            for start in 0..64 {
                for len in [1, 2, 15, 16, 17, 31, 32, 33, 63, 64, 128, 256, 512] {
                    if start + len > data.len() {
                        continue;
                    }
                    let hay = &data[start..start + len];
                    let expected = scalar(lo, hi, hay);
                    let actual = memrange(lo, hi, hay);
                    assert_eq!(
                        actual, expected,
                        "lo={lo}, hi={hi}, start={start}, len={len}",
                    );
                }
            }
        }
    }
}
