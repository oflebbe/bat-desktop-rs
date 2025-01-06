#![cfg(feature = "meow")]

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
    unsafe {
    let sz;
    
        sz = meow_fft_generate_workset_real(fft_size as libc::c_int, ptr::null_mut::<u8>());
        let mut buf = vec![0u8; sz];
        meow_fft_generate_workset_real(fft_size as libc::c_int, buf.as_mut_ptr());
    return buf;
    }
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


pub struct Fft {
    planner : Vec<u8>,
    fft_in : Vec<f32>,
    fft_out : Vec<FftComplex>,
}

impl Fft {
    pub fn new(fft_size: usize) -> Fft {
        Fft {
            planner: fft_generate_workset_real(fft_size as i32),
            fft_in: vec![0.0 as libc::c_float; fft_size],
            fft_out: vec![FftComplex { r: 0.0, j: 0.0 }; fft_size],
        }
    }

    pub fn input_set_real(&mut self, i: usize, val: f32) {
        self.fft_in[i]=val;
    }

    pub fn output_power(&self, i : usize) -> f32 {
        self.fft_out[i].r* self.fft_out[i].r +  self.fft_out[i].j * self.fft_out[i].j
    }

    pub fn process(&mut self) {
        fft_real(&mut self.planner, &mut self.fft_in, &mut self.fft_out);
     }
}