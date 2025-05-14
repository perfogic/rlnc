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

    fn gf_degree(a: u16) -> i32 {
        let branch = [(u16::BITS - a.leading_zeros()) as i32, -1i32];
        branch[(a == 0) as usize]
    }

    pub fn inv(self) -> Option<Self> {
        if self.val == 0 {
            return None;
        }

        let mut r0 = IRREDUCIBLE_POLYNOMIAL;
        let mut r1 = self.val as u16;
        let mut s0 = 0;
        let mut s1 = 1;

        (0..u16::BITS).for_each(|_| {
            let r1_nonzero = if r1 != 0 { 1 } else { 0 };
            let r1_mask = (r1_nonzero as u32).wrapping_neg();

            let deg0 = Self::gf_degree(r0);
            let deg1 = Self::gf_degree(r1);

            let shift = deg0 - deg1;
            let cond = if deg0 >= deg1 && r1_nonzero != 0 {
                1
            } else {
                0
            };
            let cond_mask = (cond as u32).wrapping_neg();

            let q_times_r1 = (r1 << shift) & cond_mask as u16;
            let q_times_s1 = (s1 << shift) & cond_mask;

            let new_r0 = r1;
            let new_r1 = r0 ^ q_times_r1;
            let new_s0 = s1;
            let new_s1 = s0 ^ q_times_s1;

            r0 = (r1_mask as u16 & new_r0) | (!(r1_mask as u16) & r0);
            r1 = (r1_mask as u16 & new_r1) | (!(r1_mask as u16) & r1);
            s0 = (r1_mask & new_s0) | (!r1_mask & s0);
            s1 = (r1_mask & new_s1) | (!r1_mask & s1);
        });

        if r0 != 1 {
            None
        } else {
            Some(Gf256 { val: s0 as u8 })
        }
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
