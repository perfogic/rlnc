use rand::Rng;
use rand::distr::{Distribution, StandardUniform};
use std::ops::{Add, Mul, Neg, Sub};

const IRREDUCIBLE_POLYNOMIAL: u16 = 0x11B;

#[derive(Default)]
pub struct Gf256 {
    val: u8,
}

impl Gf256 {
    pub fn new(val: u8) -> Self {
        Gf256 { val }
    }
}

impl Add for Gf256 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Gf256 {
            val: self.val ^ rhs.val,
        }
    }
}

impl Neg for Gf256 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Gf256 { val: self.val }
    }
}

impl Sub for Gf256 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Gf256 {
            val: self.val ^ rhs.val,
        }
    }
}

impl Mul for Gf256 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mul_res_16b = (0..u8::BITS).fold(0u16, |acc, bit_idx| {
            let selected_bit = (rhs.val >> bit_idx) & 1;
            let bit_mask = (selected_bit as u16).wrapping_neg();

            acc ^ ((self.val as u16) << bit_idx) & bit_mask
        });

        let reduced = (u8::BITS..u16::BITS)
            .rev()
            .fold(mul_res_16b, |acc, bit_idx| {
                let selected_bit = (acc >> bit_idx) & 1;
                let bit_mask = (selected_bit as u16).wrapping_neg();

                acc ^ (IRREDUCIBLE_POLYNOMIAL << (bit_idx - u8::BITS)) & bit_mask
            });

        Gf256 { val: reduced as u8 }
    }
}

impl Distribution<Gf256> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Gf256 {
        Gf256 { val: rng.random() }
    }
}
