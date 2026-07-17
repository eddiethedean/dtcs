//! Portable PRNG: `xorshift64star/1` for seeded sampling and random ops.

/// Seeded xorshift64* generator (`xorshift64star/1`).
#[derive(Debug, Clone, Copy)]
pub struct XorShift64Star {
    state: u64,
}

impl XorShift64Star {
    /// Create a generator from a nonzero seed (zero is remapped).
    #[must_use]
    pub fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 0xD7C5_3A21_u64 } else { seed },
        }
    }

    /// Next u64 in the sequence.
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Next value in `[0.0, 1.0)`.
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / ((1u64 << 53) as f64)
    }
}

/// Stable ranking hash for seeded sample: mix seed, index, and row bytes via xorshift.
#[must_use]
pub fn sample_rank(seed: u64, index: u64, row_bytes: &[u8]) -> u64 {
    let mut state = seed
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .wrapping_add(index.wrapping_mul(0xC2B2_AE3D_27D4_EB4F));
    for chunk in row_bytes.chunks(8) {
        let mut word = 0u64;
        for (i, b) in chunk.iter().enumerate() {
            word |= (*b as u64) << (8 * i);
        }
        state ^= word.wrapping_mul(0x100_0000_01B3);
        let mut rng = XorShift64Star::new(state);
        state = rng.next_u64();
    }
    XorShift64Star::new(state).next_u64()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xorshift_is_deterministic() {
        let mut a = XorShift64Star::new(42);
        let mut b = XorShift64Star::new(42);
        assert_eq!(a.next_u64(), b.next_u64());
        assert_eq!(a.next_f64(), b.next_f64());
    }

    #[test]
    fn sample_rank_stable() {
        let row = br#"{"a":1}"#;
        assert_eq!(sample_rank(7, 0, row), sample_rank(7, 0, row));
        assert_ne!(sample_rank(7, 0, row), sample_rank(7, 1, row));
    }
}
