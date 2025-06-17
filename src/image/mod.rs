//! image module
//! responsible for interacting with images, such as manipulating, saving and loading

#[cfg(not(feature = "png"))]
use std::io::Write;
use std::{fs::File, io::BufWriter, path::PathBuf, slice::IterMut};

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

    /// Creates an Iterator that traverses all pixels and their position in a cache friendly manner
    pub fn enumerate_pixels_mut(&mut self) -> EnumeratePixelsMut {
        EnumeratePixelsMut {
            iter: self.buf.iter_mut(),
            width: self.width,
            x: 0,
            y: 0,
        }
    }

    /// Saves the image as a png image to the specified path
    /// If the path does not already have the .png extension, it will be added
    #[cfg(feature = "png")]
    pub fn save(self, path: &mut PathBuf) -> Result<(), std::io::Error> {
        path.set_extension("png");
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

    /// Saves the image as a ppm image to the specified path
    /// If the path does not already have the .ppm extension, it will be added
    #[cfg(not(feature = "png"))]
    pub fn save(self, path: &mut PathBuf) -> Result<(), std::io::Error> {
        path.set_extension("ppm");
        let file = File::create(path)?;
        let mut w = BufWriter::new(file);

        w.write_all(b"P3\n\n")?;
        w.write_all(format!("{} {} 255\n", self.width, self.height).as_bytes())?;

        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self
                    .buf
                    .get((x + self.width * y) as usize)
                    .unwrap_or(&[0u8; 3]);

                w.write_all(format!("{} {} {}\n", pixel[0], pixel[1], pixel[2]).as_bytes())?;
            }
        }

        Ok(())
    }
}

/// Enumerates pixels of an image
pub struct EnumeratePixelsMut<'a> {
    iter: IterMut<'a, Rgb>,
    width: u32,
    x: u32,
    y: u32,
}

impl<'a> Iterator for EnumeratePixelsMut<'a> {
    /// x, y, pixel
    type Item = (u32, u32, &'a mut Rgb);

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = (self.x, self.y);
        if self.x < self.width - 1 {
            self.x += 1;
        } else {
            self.y += 1;
            self.x = 0;
        }

        self.iter.next().map(|p| (x, y, p))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
