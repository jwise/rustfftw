extern mod std;
extern mod fftw;

use core::num::Zero;
use std::complex::{Cmplx, Complex64};
use std::c_vec;

static epsilon : f64 = 0.01f64;

fn approx(a: f64, b: f64) -> bool {
    if ((a + epsilon) < b) { false }
    else if ((a - epsilon) > b) { false }
    else { true }
}

fn approx_c64(a: Complex64, b: Complex64) -> bool {
    approx(a.re, b.re) && approx(a.im, b.im)
}

#[test]
fn basic() {

    let in = fftw::malloc_cpx64(16);
    let out = fftw::malloc_cpx64(16);

    let p = fftw::Plan::dft_1d(16, in, out, fftw::FORWARD, fftw::MEASURE);

    for uint::range(0, 16) |i| {
        /* Should be in[i] = {r: 0.0f64, i: 0.0f64} ... */
        c_vec::set(in, i, Zero::zero());
        c_vec::set(out, i, Zero::zero());
    }

    c_vec::set(in, 2, Cmplx::new(1.0f64, 0.0f64));

    p.execute();

    assert!(approx_c64(c_vec::get(out, 0u), Cmplx::new(1.0f64, 0.0f64)));
    assert!(approx_c64(c_vec::get(out, 1u), Cmplx::new(0.707f64, -0.707f64)));
    assert!(approx_c64(c_vec::get(out, 2u), Cmplx::new(0.0f64, -1.0f64)));
    assert!(approx_c64(c_vec::get(out, 3u), Cmplx::new(-0.707f64,  -0.707f64)));
}
