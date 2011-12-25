use std;

export plan;

export malloc_cpx64;
export plan_dft_1d;
export execute;

export complex64;

export FORWARD, BACKWARD, MEASURE, DESTROY_INPUT, UNALIGNED, CONSERVE_MEMORY, EXHAUSTIVE, PRESERVE_INPUT, PATIENT, ESTIMATE;

const FORWARD : int = -1;
const BACKWARD : int = 1;

const MEASURE : uint = 0u;
const DESTROY_INPUT : uint = (1u << 0u);
const UNALIGNED : uint = (1u << 1u);
const CONSERVE_MEMORY : uint = (1u << 2u);
const EXHAUSTIVE : uint = (1u << 3u);
const PRESERVE_INPUT : uint = (1u << 4u);
const PATIENT : uint = (1u << 5u);
const ESTIMATE : uint = (1u << 6u);

type complex64 = { r: f64, i: f64 };

#[abi = "cdecl"]
#[link_name = "fftw3"]
native mod native {
    type plan;
    type complexp;

    fn fftw_malloc(sz: uint) -> *mutable u8;
    fn fftw_free(p: *mutable u8);
    fn fftw_plan_dft_1d(n: uint, in: *mutable complex64, out: *mutable complex64, sign: int, flags: uint) -> plan;
    fn fftw_destroy_plan(p: plan);
    fn fftw_execute(p: plan);
}

fn malloc_cpx64(n: uint) -> std::c_vec::t<complex64> unsafe {
    let mem = native::fftw_malloc(n * 16u);

    assert mem as int != 0;
    
    ret std::c_vec::create_with_dtor(
        mem as *mutable complex64,
        n,
        bind native::fftw_free(mem));
}

resource plan_res(p: native::plan) {
    native::fftw_destroy_plan(p);
}

type plan = { plan: native::plan,
              /* Have to hold on to them here to replicate the object lifecycle in C-land. */
              in: std::c_vec::t<complex64>,
              out: std::c_vec::t<complex64>,
              dtor: @plan_res };

fn plan_dft_1d(n: uint, in: std::c_vec::t<complex64>, out: std::c_vec::t<complex64>, sign: int, flags: uint) -> plan {
    assert std::c_vec::size(in) >= n;
    assert std::c_vec::size(out) >= n;

    let p = native::fftw_plan_dft_1d(n, unsafe { std::c_vec::ptr(in) }, unsafe { std::c_vec::ptr(out) }, sign, flags);
    
    assert p as int != 0;
    
    ret { plan: p, in: in, out: out, dtor: @plan_res(p) };
}

fn execute(p: plan) {
    native::fftw_execute(p.plan);
}
