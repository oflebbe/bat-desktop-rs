#![cfg(not(feature = "meow"))]

use rustfft::{num_complex::Complex32, FftPlanner};
use std::sync::Arc;


pub struct Fft {
    planner : Arc<dyn rustfft::Fft<f32>>,
    buffer : Vec<Complex32>,
}

impl Fft {
    pub fn new(fft_size: usize) -> Fft {
        let mut instance = FftPlanner::new();
        
        Fft {
            planner : instance.plan_fft_forward(fft_size),
            buffer : vec![Complex32 { re: 0.0, im: 0.0 }; fft_size] }
    }

    pub fn input_set_real(&mut self, i: usize, val: f32) {
        self.buffer[i]=Complex32{ re : val, im: 0.0}
    }

    pub fn output_power(&self, i : usize) -> f32 {
        self.buffer[i].re* self.buffer[i].re +  self.buffer[i].im * self.buffer[i].im
    }
    
    pub fn process(&mut self) {
       self.planner.process(&mut self.buffer);
    }
}