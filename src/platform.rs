#[cfg(target_pointer_width = "128")]
#[inline(always)]
pub(crate) fn mum(a: u64, b: u64) -> (u64, u64) {
    let r: u128 = (a as u128) * (b as u128);
    (r as u64, (r >> 64) as u64)
}

#[cfg(all(target_env = "msvc", target_arch = "x86_64"))]
#[inline(always)]
pub(crate) fn mum(a: u64, b: u64) -> (u64, u64) {
    let lo: u64;
    let mut hi: u64 = 0;

    unsafe {
        lo = core::arch::x86_64::_mulx_u64(a, b, &mut hi);
    }
    (lo, hi)
}

#[cfg(not(all(target_env = "msvc", target_arch = "x86_64")))]
#[cfg(not(target_pointer_width = "128"))]
#[inline(always)]
pub(crate) fn mum(a: u64, b: u64) -> (u64, u64) {
    let ha = a >> 32;
    let hb = b >> 32;
    let la = (a as u32) as u64;
    let lb = (b as u32) as u64;
    let rh = ha * hb;
    let rm0 = ha * lb;
    let rm1 = hb * la;
    let rl = la * lb;
    let t = rl + (rm0 << 32);
    let mut c: u64 = 0;
    if t < rl {
        c = 1;
    }

    let lo = t.wrapping_add((rm1 << 32));
    if lo < t {
        c += 1;
    }

    (lo, rh.wrapping_add((rm0 >> 32) + (rm1 >> 32) + c))
}

#[cfg(any(target_os = "windows", target_endian = "little"))]
pub(crate) mod rot {
    #[inline(always)]
    pub(crate) fn r8(p: &[u8]) -> u64 {
        let mut out: u64 = 0;
        for i in (0..8).step_by(1) {
            out |= (p[i] as u64) << (i * 8);
        }
        out
    }

    #[inline(always)]
    pub(crate) fn r4(p: &[u8]) -> u64 {
        let mut out: u64 = 0;
        for i in (0..4).step_by(1) {
            out |= (p[i] as u64) << (i * 8);
        }
        out
    }
}

#[cfg(target_endian = "big")]
#[cfg(any(
    target_env = "msvc",
    all(target_env = "gnu", target_vendor = "apple"),
    all(target_env = "gnu", target_vendor = "unknown"),
    all(target_env = "gnu", target_vendor = "pc-windows-msvc"),
    all(target_env = "gnu", target_vendor = "rumprun"),
    all(target_env = "gnu", target_vendor = "uwp"),
    all(target_env = "gnu", target_vendor = "androideabi"),
    all(target_env = "gnu", target_vendor = "linux"),
    all(target_env = "gnu", target_vendor = "netbsd"),
    all(target_env = "gnu", target_vendor = "openbsd"),
    all(target_env = "gnu", target_vendor = "freebsd"),
    all(target_env = "gnu", target_vendor = "fuchsia"),
    all(target_env = "gnu", target_vendor = "unknown-linux-gnu"),
    all(target_env = "gnu", target_vendor = "unknown-freebsd"),
    all(target_env = "gnu", target_vendor = "cloudabi"),
    all(target_env = "gnu", target_vendor = "android"),
    all(target_env = "gnu", target_vendor = "nvptx"),
    all(target_env = "gnu", target_vendor = "cuda"),
    all(target_env = "gnu", target_vendor = "riscv64"),
    all(target_env = "gnu", target_vendor = "fortanix"),
    all(target_env = "gnu", target_vendor = "hermit"),
    all(target_env = "gnu", target_vendor = "redox"),
    all(target_env = "gnu", target_vendor = "l4re"),
    all(target_env = "gnu", target_vendor = "wasi"),
    all(target_env = "gnu", target_vendor = "wasm32"),
    all(target_env = "gnu", target_vendor = "tizen"),
    all(target_env = "gnu", target_vendor = "uefi"),
    all(target_env = "gnu", target_vendor = "solaris"),
    all(target_env = "gnu", target_vendor = "haiku"),
    all(target_env = "gnu", target_vendor = "phoenix"),
    all(target_env = "gnu", target_vendor = "vxworks"),
    all(target_env = "gnu", target_vendor = "uclibc"),
    all(target_env = "gnu", target_vendor = "krux"),
    all(target_env = "gnu", target_vendor = "trustedbsd"),
    all(target_env = "gnu", target_vendor = "baremetal"),
    all(target_env = "gnu", target_vendor = "bluegene"),
    all(target_env = "gnu", target_vendor = "contiki"),
    all(target_env = "gnu", target_vendor = "openwrt"),
    all(target_env = "gnu", target_vendor = "philips"),
    all(target_env = "gnu", target_vendor = "tinker"),
    all(target_env = "gnu", target_vendor = "nucleus"),
    all(target_env = "gnu", target_vendor = "pnacl"),
    all(target_env = "gnu", target_vendor = "wasmcloud"),
    all(target_env = "gnu", target_vendor = "sony"),
    all(target_env = "gnu", target_vendor = "savannah"),
    all(target_env = "gnu", target_vendor = "trustzone"),
    all(target_env = "gnu", target_vendor = "none"),
))]
pub(crate) mod rot {
    #[inline(always)]
    pub(crate) fn r8(p: &[u8]) -> u64 {
        let mut out: u64 = 0;
        for i in (0..8).step_by(1) {
            out |= (p[i] as u64) << (i * 8);
        }
        u64::swap_bytes(out)
    }

    #[inline(always)]
    pub(crate) fn r4(p: &[u8]) -> u64 {
        let mut out: u64 = 0;
        for i in (0..4).step_by(1) {
            out |= (p[i] as u64) << (i * 8);
        }
        u64::swap_bytes(out)
    }
}

#[cfg(target_endian = "big")]
#[cfg(not(any(
    target_env = "msvc",
    all(target_env = "gnu", target_vendor = "apple"),
    all(target_env = "gnu", target_vendor = "unknown"),
    all(target_env = "gnu", target_vendor = "pc-windows-msvc"),
    all(target_env = "gnu", target_vendor = "rumprun"),
    all(target_env = "gnu", target_vendor = "uwp"),
    all(target_env = "gnu", target_vendor = "androideabi"),
    all(target_env = "gnu", target_vendor = "linux"),
    all(target_env = "gnu", target_vendor = "netbsd"),
    all(target_env = "gnu", target_vendor = "openbsd"),
    all(target_env = "gnu", target_vendor = "freebsd"),
    all(target_env = "gnu", target_vendor = "fuchsia"),
    all(target_env = "gnu", target_vendor = "unknown-linux-gnu"),
    all(target_env = "gnu", target_vendor = "unknown-freebsd"),
    all(target_env = "gnu", target_vendor = "cloudabi"),
    all(target_env = "gnu", target_vendor = "android"),
    all(target_env = "gnu", target_vendor = "nvptx"),
    all(target_env = "gnu", target_vendor = "cuda"),
    all(target_env = "gnu", target_vendor = "riscv64"),
    all(target_env = "gnu", target_vendor = "fortanix"),
    all(target_env = "gnu", target_vendor = "hermit"),
    all(target_env = "gnu", target_vendor = "redox"),
    all(target_env = "gnu", target_vendor = "l4re"),
    all(target_env = "gnu", target_vendor = "wasi"),
    all(target_env = "gnu", target_vendor = "wasm32"),
    all(target_env = "gnu", target_vendor = "tizen"),
    all(target_env = "gnu", target_vendor = "uefi"),
    all(target_env = "gnu", target_vendor = "solaris"),
    all(target_env = "gnu", target_vendor = "haiku"),
    all(target_env = "gnu", target_vendor = "phoenix"),
    all(target_env = "gnu", target_vendor = "vxworks"),
    all(target_env = "gnu", target_vendor = "uclibc"),
    all(target_env = "gnu", target_vendor = "krux"),
    all(target_env = "gnu", target_vendor = "trustedbsd"),
    all(target_env = "gnu", target_vendor = "baremetal"),
    all(target_env = "gnu", target_vendor = "bluegene"),
    all(target_env = "gnu", target_vendor = "contiki"),
    all(target_env = "gnu", target_vendor = "openwrt"),
    all(target_env = "gnu", target_vendor = "philips"),
    all(target_env = "gnu", target_vendor = "tinker"),
    all(target_env = "gnu", target_vendor = "nucleus"),
    all(target_env = "gnu", target_vendor = "pnacl"),
    all(target_env = "gnu", target_vendor = "wasmcloud"),
    all(target_env = "gnu", target_vendor = "sony"),
    all(target_env = "gnu", target_vendor = "savannah"),
    all(target_env = "gnu", target_vendor = "trustzone"),
    all(target_env = "gnu", target_vendor = "none"),
)))]
pub(crate) mod rot {
    #[inline(always)]
    pub(crate) fn r8(p: &[u8]) -> u64 {
        let mut out: u64 = 0;
        for i in (0..8).step_by(1) {
            out |= (p[i] as u64) << (i * 8);
        }

        (((out >> 56) & 0xff) |
        ((out >> 40) & 0xff00) |
        ((out >> 24) & 0xff0000) |
        ((out >> 8) & 0xff000000) |
        ((out << 8) & 0xff00000000) |
        ((out << 24) & 0xff0000000000) |
        ((out << 40) & 0xff000000000000) |
        ((out << 56) & 0xff00000000000000))
    }

    #[inline(always)]
    pub(crate) fn r4(p: &[u8]) -> u64 {
        let mut out: u64 = 0;
        for i in (0..4).step_by(1) {
            out |= (p[i] as u64) << (i * 8);
        }

        (((out >> 24) & 0xff) | 
        ((out >>  8) & 0xff00) | 
        ((out <<  8) & 0xff0000) | 
        ((out << 24) & 0xff000000))
    }
}