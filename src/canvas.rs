use std::fs::File;
use std::io::{BufWriter, Write};

pub type Color = [u8; 3];

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    data: Vec<Color>,
    z_buffer: Vec<f32>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![[0; 3]; width * height],
            z_buffer: vec![0.0; width * height],
        }
    }

    pub fn write_pixel(
        &mut self,
        x: usize,
        y: usize,
        z: f32,
        color: &Color,
    ) -> Result<(), &'static str> {
        if x >= self.width || y >= self.height {
            return Err("Tried to write a pixel outside the canvas bounds");
        }

        self.data[((self.height - 1) - y) * self.width + x] = *color;
        self.z_buffer[((self.height - 1) - y) * self.width + x] = z;

        Ok(())
    }

    pub fn read_depth(&self, x: usize, y: usize) -> f32 {
        self.z_buffer[((self.height - 1) - y) * self.width + x]
    }

    pub fn write_to_file(&self, filename: &str) {
        let f = File::create(filename).expect("Could not open file");
        let mut f = BufWriter::new(f);

        // Write ppm header
        write!(&mut f, "P6\n{} {}\n255\n", self.width, self.height)
            .expect("Could not write file header");

        self.data.iter().for_each(|c| {
            f.write(c).expect("Could not write color");
        });

        f.flush().expect("could not flush write buffer");
    }
}
