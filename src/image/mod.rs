//! Module for interacting with images

use std::{fs::File, io::BufWriter, path::Path};

/// Represents a pixel in Rgb with 3 values from 0 to 255
pub type Rgb = [u8; 3];

/// Represents an Image which holds its width and height and the appropriate amount of Rgb pixels
pub struct Image {
    width: u32,
    height: u32,
    buf: Vec<Rgb>,
}

impl Image {
    /// Create a new Image with the given dimensions
    /// The Image gets initialized black
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            width,
            height,
            buf: vec![[0; 3]; (width * height) as usize],
        }
    }

    /// Returns a mutable reference to a pixel in the image
    /// If the x and y values are out of bounds, the function will panic
    pub fn get_pixel_mut(&mut self, x: u32, y: u32) -> &mut Rgb {
        self.buf
            .get_mut((x + self.width * y) as usize)
            .expect("I'll trust the caller...")
    }

    /// Saves the image as a png image to the specified path
    pub fn save_png(self, path: &Path) -> Result<(), std::io::Error> {
        let file = File::create(path)?;
        let w = &mut BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455));
        let source_chromaticities = png::SourceChromaticities::new(
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000),
        );
        encoder.set_source_chromaticities(source_chromaticities);
        let mut writer = encoder.write_header()?;

        writer.write_image_data(self.buf.as_flattened())?;

        Ok(())
    }
}
