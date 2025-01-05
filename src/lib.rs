use libc::{c_float, size_t};
use std::ptr;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FftComplex {
    pub r: c_float,
    pub j: c_float,
}

extern "C" {
    fn meow_fft_generate_workset_real(N: libc::c_int, workset: *mut u8) -> size_t;
    pub fn meow_fft_real(workset: *mut u8, fft_in: *mut c_float, fft_out: *mut FftComplex);
}

pub fn fft_generate_workset_real(fft_size: i32) -> Vec<u8> {
    let sz;
    unsafe {
        sz = meow_fft_generate_workset_real(fft_size as libc::c_int, ptr::null_mut::<u8>());
    }
    let mut buf = vec![0u8; sz];
    unsafe {
        meow_fft_generate_workset_real(fft_size as libc::c_int, buf.as_mut_ptr());
    }
    return buf;
}

pub fn fft_real(workset: &mut Vec<u8>, fft_in: &mut [c_float], fft_out: &mut [FftComplex]) {
    unsafe {
        meow_fft_real(
            workset.as_mut_ptr(),
            fft_in.as_mut_ptr(),
            fft_out.as_mut_ptr(),
        );
    }
}
