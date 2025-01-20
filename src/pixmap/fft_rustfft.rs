use realfft::{num_complex::Complex32, RealFftPlanner};
use std::sync::Arc;

pub struct Fft {
    planner: Arc<dyn realfft::RealToComplex<f32>>,
    input: Vec<f32>,
    output: Vec<Complex32>,
}

impl Fft {
    pub fn new(fft_size: usize) -> Fft {
        let mut instance = RealFftPlanner::<f32>::new();
        let p =  instance.plan_fft_forward(fft_size);
        let input = p.make_input_vec();
        let output = p.make_output_vec();

        Fft {
            planner: p,
            input: input,
            output : output,
        }
    }

    pub fn input_set_real(&mut self, i: usize, val: f32) {
        self.input[i] =  val
    }

    pub fn output_power(&self, i: usize) -> f32 {
        self.output[i].re * self.output[i].re + self.output[i].im * self.output[i].im
    }

    pub fn process(&mut self) {
        let _ = self.planner.process(&mut self.input, &mut self.output);
    }
}
