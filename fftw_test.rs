use std;
use fftw;

const epsilon : f64 = 0.01f64;

pure fn approx(a: f64, b: f64) -> bool {
    if ((a + epsilon) < b) { false }
    else if ((a - epsilon) > b) { false }
    else { true }
}

pure fn approx_c64(a: fftw::complex64, b: fftw::complex64) -> bool {
    approx(a.r, b.r) && approx(a.i, b.i)
}

#[test]
fn basic() {
    let in = fftw::malloc_cpx64(16u);
    let out = fftw::malloc_cpx64(16u);
    
    let p = fftw::plan_dft_1d(16u, in, out, fftw::FORWARD, fftw::MEASURE);
    
    uint::range(0u, 16u) { |i|
        /* Should be in[i] = {r: 0.0f64, i: 0.0f64} ... */
        std::c_vec::set(in, i, {r: 0.0f64, i: 0.0f64});
        std::c_vec::set(out, i, {r: 0.0f64, i: 0.0f64});
    }
    
    std::c_vec::set(in, 2u, {r: 1.0f64, i:0.0f64});

    fftw::execute(p);
    
    assert approx_c64(std::c_vec::get(out, 0u), {r: 1.0f64, i: 0.0f64});
    assert approx_c64(std::c_vec::get(out, 1u), {r: 0.707f64, i: -0.707f64});
    assert approx_c64(std::c_vec::get(out, 2u), {r: 0.0f64, i: -1.0f64});
    assert approx_c64(std::c_vec::get(out, 3u), {r: -0.707f64, i: -0.707f64});
}
