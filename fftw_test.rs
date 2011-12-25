use std;
use fftw;

fn main() {
    let in = fftw::malloc_cpx64(32u);
    let out = fftw::malloc_cpx64(32u);
    
    let p = fftw::plan_dft_1d(32u, in, out, fftw::FORWARD, fftw::MEASURE);
    
    uint::range(0u, 32u) { |i|
        /* Should be in[i] = {r: 0.0f64, i: 0.0f64} ... */
        std::c_vec::set(in, i, {r: 0.0f64, i: 0.0f64});
        std::c_vec::set(out, i, {r: 0.0f64, i: 0.0f64});
    }
    
    std::c_vec::set(in, 2u, {r: 1.0f64, i:0.0f64});

    fftw::execute(p);
    
    uint::range(0u, 32u) { |i|
        log(error, (i, std::c_vec::get(in, i), std::c_vec::get(out, i)));
    }
}
