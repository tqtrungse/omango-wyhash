use crate::platform::mum;

#[inline(always)]
pub(crate) fn mix(a: u64, b: u64) -> u64 {
    let (lo, hi) = mum(a, b);
    lo ^ hi
}

#[inline(always)]
pub(crate) fn r3(p: &[u8]) -> u64 {
    let k = p.len();

    ((p[0] as u64) << 16) |
        ((p[k >> 1] as u64) << 8) |
        (p[k - 1] as u64)
}

pub(crate) fn is_prime(n: u64) -> bool {
    if n < 2 || (n & 1) == 0 { return false; }
    if n < 4 { return true; }
    if !sprp(n, 2) { return false; }
    if n < 2047 { return true; }
    if !sprp(n, 3) { return false; }
    if !sprp(n, 5) { return false; }
    if !sprp(n, 7) { return false; }
    if !sprp(n, 11) { return false; }
    if !sprp(n, 13) { return false; }
    if !sprp(n, 17) { return false; }
    if !sprp(n, 19) { return false; }
    if !sprp(n, 23) { return false; }
    if !sprp(n, 29) { return false; }
    if !sprp(n, 31) { return false; }
    if !sprp(n, 37) { return false; }
    true
}

/// modified from https://github.com/going-digital/Prime64
#[inline(always)]
fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    let mut r = 0;
    let mut _b = b;
    let mut _a = a;
    while _b > 0 {
        if (_b & 1) > 0 {
            let mut r2 = r + a;
            if r2 < r {
                r2 -= m;
            }
            r = r2 % m;
        }
        _b >>= 1;
        if _b > 0 {
            let mut a2 = _a + _a;
            if a2 < _a {
                a2 -= m;
            }
            _a = a2 % m;
        }
    }
    r
}

#[inline(always)]
fn pow_mod(a: u64, b: u64, m: u64) -> u64 {
    let mut r = 1;
    let mut _b = b;
    let mut _a = a;

    while _b > 0 {
        if _b & 1 > 0 {
            r = mul_mod(r, a, m);
        }

        _b >>= 1;
        if b > 0 {
            _a = mul_mod(_a, _a, m);
        };
    }
    r
}

fn sprp(n: u64, a: u64) -> bool {
    let mut d = n - 1;
    let mut s = 0u8;

    while (d & 0xff) == 0 {
        d >>= 8;
        s += 8;
    }

    if (d & 0xf) == 0 {
        d >>= 4;
        s += 4;
    }
    if (d & 0x3) == 0 {
        d >>= 2;
        s += 2;
    }
    if (d & 0x1) == 0 {
        d >>= 1;
        s += 1;
    }

    let mut b = pow_mod(a, d, n);
    if b == 1 || b == (n - 1) {
        return true;
    }

    for _ in (1..s).step_by(1) {
        b = mul_mod(b, b, n);
        if b <= 1 {
            return false;
        }
        if b == n - 1 {
            return true;
        }
    }
    false
}