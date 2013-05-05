#[link(name = "fftw",
       vers = "0.1",
       url = "https://www.github.com/jwise/rustfftw/",
       uuid = "deddc100-2ed9-11e1-9b99-50e54950d806")];
#[desc = "FFTW bindings for Rust"];
#[license = "GPLv3"];
#[crate_type = "lib"];

extern mod std;

use std::complex::Complex64;
use std::c_vec;

pub static FORWARD : i32 = -1;
pub static BACKWARD : i32 = 1;

pub static MEASURE : u32 = 0;
pub static DESTROY_INPUT : u32 = (1 << 0);
pub static UNALIGNED : u32 = (1 << 1);
pub static CONSERVE_MEMORY: u32 = (1 << 2);
pub static EXHAUSTIVE : u32 = (1 << 3);
pub static PRESERVE_INPUT : u32 = (1 << 4);
pub static PATIENT : u32 = (1 << 5);
pub static ESTIMATE : u32 = (1 << 6);

#[abi = "cdecl"]
mod native {
    use core::libc::{c_int, c_uint};
    use std::complex::Complex64;

    #[allow(non_camel_case_types)]
    pub enum plan_opaque {}
    #[allow(non_camel_case_types)]
    pub type plan = *plan_opaque;

    #[link_args = "-lfftw3"]
    pub extern {
        unsafe fn fftw_malloc(sz: c_uint) -> *mut u8;
        unsafe fn fftw_free(p: *mut u8);
        unsafe fn fftw_plan_dft_1d(n: c_uint,
                                   in: *mut Complex64, out: *mut Complex64,
                                   sign: c_int, flags: c_uint) -> plan;
        unsafe fn fftw_destroy_plan(p: plan);
        unsafe fn fftw_execute(p: plan);
    }
}

pub fn malloc_cpx64(n: u32) -> c_vec::CVec<Complex64> {
    let mem = native::fftw_malloc(n * (sys::size_of::<Complex64>() as u32));

    assert!(mem.is_not_null());

    c_vec::c_vec_with_dtor(
        mem as *mut Complex64,
        n as uint,
        || { unsafe { native::fftw_free(mem) } })
}

pub struct Plan {
    plan: native::plan,
    /* Have to hold on to them here to replicate the object lifecycle in C-land. */
    in: c_vec::CVec<Complex64>,
    out: c_vec::CVec<Complex64>,
}

#[unsafe_destructor]
impl Drop for Plan {
    fn finalize(&self) {
        unsafe { native::fftw_destroy_plan(self.plan); }
    }
}

pub impl Plan {
    pub fn dft_1d(n: u32,
                  in: c_vec::CVec<Complex64>,
                  out: c_vec::CVec<Complex64>,
                  sign: i32, flags: u32) -> Plan {
        assert!(c_vec::len(in) >= n as uint);
        assert!(c_vec::len(out) >= n as uint);

        let p = unsafe {
            native::fftw_plan_dft_1d(
                n,
                c_vec::ptr(in),
                c_vec::ptr(out),
                sign, flags)
        };

        assert!(p.is_not_null());

        Plan { plan: p, in: in, out: out }
    }

    pub fn execute(&self) {
        unsafe { native::fftw_execute(self.plan) };
    }
}
