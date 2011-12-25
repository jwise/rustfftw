fftw_test: libfftw.dylib fftw_test.rs
	rustc -L . fftw_test.rs

# Not actually called 'libfftw.dylib' anymore.  Sigh.	
libfftw.dylib: fftw.rc fftw.rs
	rustc -L . --lib fftw.rc
