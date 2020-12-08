#[path = "defaults.rs"] mod defaults;

pub struct Screen {
    width: usize,
    height: usize,
    pixels: Vec<PixelValue>,
}

#[derive(Copy, Clone)]
pub enum PixelValue {
    OFF,
    DIM,
    MEDIUM,
    BRIGHT,
}


impl Screen {
    pub fn new() -> Self {
        Screen {
            width: defaults::screen::WIDTH,
            height: defaults::screen::HEIGHT,
            pixels: Vec::new()
        }
    }

    pub fn with_resolution(h_resolution: usize, v_resolution: usize) -> Self {
        Screen {
            width: h_resolution,
            height: v_resolution,
            pixels: Vec::new()
        }
    }

    pub fn set_pixels(&mut self, pixel_data: Vec<PixelValue>) {
        self.pixels = pixel_data;
    }

    pub fn draw(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                draw_pixel(self.pixels[y * self.width + x]);
            }
            println!();
        }
    }
}

fn draw_pixel(value: PixelValue) {
    match value {
        PixelValue::OFF => print!(" "),
        PixelValue::DIM => print!("-"),
        PixelValue::MEDIUM => print!("o"),
        PixelValue::BRIGHT => print!("#"),
    };
}