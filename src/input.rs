use std::error::Error;
use std::ffi::OsStr;
use std::fs::File;

pub struct ScaleOffset {
    pub scale: usize,
    pub offset: usize,
}

#[cfg(feature = "mmap")]
#[derive(Default)]
pub struct Input {
    buf: Option<memmap2::Mmap>,
    _sampling: f32, // sampling freq
    pub channels: Vec<ScaleOffset>,
}

#[cfg(not(feature = "mmap"))]
pub struct Input {
    buf: Vec<u8>,
    _sampling: f32, // sampling freq
    channels: Vec<ScaleOffset>,
}

#[cfg(feature = "mmap")]
impl Input {
    pub fn new(filename: &OsStr) -> Result<Input, Box<dyn Error>> {
        let file = File::open(filename)?;
        Ok(Input {
            buf: unsafe { memmap2::Mmap::map(&file).ok() },
            _sampling: 250e3,
            channels: vec![
                ScaleOffset {
                    scale: 2,
                    offset: 0,
                },
                ScaleOffset {
                    scale: 2,
                    offset: 1,
                },
            ],
        })
    }
}

#[cfg(not(feature = "mmap"))]
pub impl Input {
    fn new(filename: &OsStr) -> Result<Input, Box<dyn Error>> {
        Ok(Input {
            buf: std::fs::read(filename)?,
            sampling: 250e3,
            channels: vec![
                ScaleOffset {
                    scale: 2,
                    offset: 0,
                },
                ScaleOffset {
                    scale: 2,
                    offset: 1,
                },
            ],
        })
    }
}

impl Input {
    pub fn get(&self) -> &[u16] {
        let (_, data16, _) = unsafe { self.buf.as_ref().unwrap().align_to::<u16>() }; // holy cow
        data16
    }
}
