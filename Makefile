check: fftw_test
	./fftw_test

fftw_test: libfftw* fftw_test.rs
	rustc -L . fftw_test.rs --test

libfftw*: fftw.rs
	rustc -L . --lib fftw.rs
