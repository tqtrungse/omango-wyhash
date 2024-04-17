// Copyright (c) 2024 Trung Tran <tqtrungse@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! The implementation is based on the Wang Yi hash final version 4.2.
//!
//! Source: '<https://github.com/wangyi-fudan/wyhash/blob/master/wyhash.h>'

use lazy_static::lazy_static;
use omango_util::hint::{likely, unlikely};

use crate::{
    platform::{
        mum,
        rot::{r4, r8},
    },
    util::{is_prime, mix, r3},
};

mod platform;
mod util;

lazy_static! {
    pub static ref SECRET: [u64; 4] = [
        0x2d358dccaa6c78a5, 
        0x8bb84b93962eacc9, 
        0x4b33a62ed433d4a3, 
        0x4d5a2da51de1aa47,
    ];
}

#[inline(always)]
pub fn hash(key: &[u8], seed: u64, secret: &[u64]) -> u64 {
    let len = key.len();
    let mut a: u64 = 0;
    let mut b: u64 = 0;
    let mut see = seed ^ mix(seed ^ secret[0], secret[1]);

    if likely(len <= 16) {
        if likely(len >= 4) {
            a = (r4(key) << 32) | r4(&key[((len >> 3) << 2)..]);
            b = (r4(&key[len - 4..]) << 32) | r4(&key[len - 4 - ((len >> 3) << 2)..]);
        } else if likely(len > 0) {
            a = r3(key);
        }
    } else {
        let mut i = len;
        let mut p = key;

        if unlikely(i >= 48) {
            let mut see1 = see;
            let mut see2 = see;

            loop {
                see = mix(r8(p) ^ secret[1], r8(&p[8..]) ^ see);
                see1 = mix(r8(&p[16..]) ^ secret[2], r8(&p[24..]) ^ see1);
                see2 = mix(r8(&p[32..]) ^ secret[3], r8(&p[40..]) ^ see2);

                p = &p[48..];
                i -= 48;

                if unlikely(i < 48) {
                    break;
                }
            }
            see ^= see1 ^ see2;
        }

        while unlikely(i > 16) {
            see = mix(r8(p) ^ secret[1], r8(&p[8..]) ^ see);
            p = &p[16..];
            i -= 16;
        }
        a = r8(&key[len - i - 16..]);
        b = r8(&key[len - i - 8..]);
    }

    a ^= secret[1];
    b ^= see;
    (a, b) = mum(a, b);

    mix(a ^ secret[0] ^ (len as u64), b ^ secret[1])
}

/// Make your own secret.
///
/// Own secret must have the same length of default secret is 4.
#[inline(always)]
pub fn make_secret(seed: u64, out: &mut [u64]) {
    let c: [u8; 70] = [
        15, 23, 27, 29, 30, 39, 43,
        45, 46, 51, 53, 54, 57, 58,
        60, 71, 75, 77, 78, 83, 85,
        86, 89, 90, 92, 99, 101, 102,
        105, 106, 108, 113, 114, 116, 120,
        135, 139, 141, 142, 147, 149, 150,
        153, 154, 156, 163, 165, 166, 169,
        170, 172, 177, 178, 180, 184, 195,
        197, 198, 201, 202, 204, 209, 210,
        212, 216, 225, 226, 228, 232, 240
    ];
    let mut see = seed;

    for i in (0..4).step_by(1) {
        let mut ok;
        loop {
            ok = true;
            out[i] = 0;

            for j in (0..64).step_by(8) {
                out[i] |= (c[(unsafe { rand(&mut see) } % (std::mem::size_of_val(&c) as u64)) as usize] as u64) << j;
            }

            if out[i] % 2 == 0 {
                continue;
            }

            for j in (0..i).step_by(1) {
                if (out[j] ^ out[i]).count_ones() != 32 {
                    ok = false;
                    break;
                }
            }

            if ok && !is_prime(out[i]) {
                ok = false;
            }

            if ok {
                break;
            }
        }
    }
}

/// A useful 64bit-64bit mix function to produce deterministic pseudo random numbers 
/// that can pass BigCrush and PractRand.
#[inline(always)]
pub fn hash64(a: u64, b: u64) -> u64 {
    let (lo, hi) = mum(a ^ SECRET[0], b ^ SECRET[1]);
    mix(lo ^ SECRET[0], hi ^ SECRET[1])
}

/// The rand PRNG that pass BigCrush and PractRand.
#[inline(always)]
pub unsafe fn rand(seed: *mut u64) -> u64 {
    *seed += SECRET[0];
    mix(*seed, (*seed) ^ SECRET[1])
}

/// Convert any 64-bit pseudo random numbers to uniform distribution [0,1). 
/// It can be combined with [rand], [hash64] or [hash].
#[inline(always)]
pub fn to_u01(r: u64) -> f64 {
    let norm = 1.0 / (((1u64) << 52) as f64);
    ((r >> 12) as f64) * norm
}

/// Convert any 64-bit pseudo random numbers to APPROXIMATE Gaussian distribution. 
/// It can be combined with [rand], [hash64] or [hash].
#[inline(always)]
pub fn to_gau(r: u64) -> f64 {
    let norm = 1.0 / ((1 << 20) as f64);
    (((r & 0x1fffff) + ((r >> 21) & 0x1fffff) + ((r >> 42) & 0x1fffff)) as f64) * norm - 3.0
}

/// Fast range integer random number generation on [0,k) credit to Daniel Lemire. 
/// It can be combined with [rand], [hash64] or [hash].
#[inline(always)]
pub fn to_u0k(r: u64, k: u64) -> u64 {
    let (_, hi) = mum(r, k);
    hi
}

mod test {
    #[test]
    fn test_vector() {
        let messages: [&str; 7] = [
            "",
            "a",
            "abc",
            "message digest",
            "abcdefghijklmnopqrstuvwxyz",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
            "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
        ];

        let mut h: u64;
        for i in (0..7).step_by(1) {
            h = crate::hash(messages[i].as_bytes(), i as u64, crate::SECRET.as_slice());
            println!("{}-{}", h, messages[i]);
        }
    }
}