//! image module
//! responsible for interacting with images, such as manipulating, saving and loading

use std::io::{self, Write};
use std::iter::zip;
use std::path::Path;
use std::{fs::File, io::BufWriter, path::PathBuf};

use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::input::InputError;

/// Represents a pixel in Rgb with 3 values from 0 to 255
pub type Rgb = [u8; 3];

/// Represents an Image which holds its width and height and the appropriate amount of Rgb pixels
#[derive(Debug, Clone)]
pub struct Image {
    width: u32,
    height: u32,
    buf: Vec<Vec<Rgb>>,
}

impl Image {
    /// Create a new Image with the given dimensions
    /// The Image gets initialized black
    pub fn new(width: u32, height: u32, frames: usize) -> Image {
        Image {
            width,
            height,
            buf: vec![vec![[0; 3]; (width * height) as usize]; frames],
        }
    }

    /// Load a png from the given path into an `Image`
    /// returns an InputError if the file cannot be read or is not a valid png file
    pub fn load_png(path: &PathBuf) -> Result<Image, InputError> {
        let file = File::open(path).map_err(|err| {
            Self::io_err_to_input_err(err, path, "Error while reading image from")
        })?;
        let decoder = png::Decoder::new(file);
        let mut reader = decoder.read_info().map_err(|err| {
            Self::io_err_to_input_err(err.into(), path, "Error while decoding image")
        })?;

        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf).map_err(|err| {
            Self::io_err_to_input_err(err.into(), path, "Error while decoding image")
        })?;
        let bytes = &buf[..info.buffer_size()];
        let imgbuf: Vec<_> = bytes.chunks(3).map(|a| [a[0], a[1], a[2]]).collect();
        let width = info.width;
        let height = info.height;

        Ok(Image {
            width,
            height,
            buf: vec![imgbuf],
        })
    }

    /// Return the images `Rgb` value at the given Texel `(u, v)`
    /// will panic if `u` or `v` are not in range 0..1
    pub fn get_pixel(&self, frame: usize, u: f32, v: f32) -> Rgb {
        let (x, y) = (
            (u * self.width as f32) as u32,
            (v * self.height as f32) as u32,
        );
        *self
            .buf
            .get(frame)
            .unwrap()
            .get((x + self.width * y) as usize)
            .unwrap()
    }

    /// Set each pixel from the corresponding x and y value
    /// Will try to use a parallel iterator for better performance
    pub fn par_init_pixels<OP>(&mut self, frame: usize, op: OP)
    where
        OP: Fn(&mut (u32, u32)) -> Rgb + Sync + Send,
    {
        assert!(self.buf.len() >= frame);
        let mut x = 0;
        let mut y = 0;

        let mut coords: Vec<_> = self
            .buf
            .get(frame)
            .unwrap()
            .iter()
            .map(|_| {
                if x < self.width - 1 {
                    x += 1;
                } else {
                    y += 1;
                    x = 0;
                }

                (x, y)
            })
            .collect();
        let f = self.buf.get_mut(frame).unwrap();
        *f = coords.par_iter_mut().map(op).collect();
    }

    /// format io error to input error
    fn io_err_to_input_err(err: io::Error, path: &Path, msg: &str) -> InputError {
        InputError::new(
            format!("{} {}", msg, path.to_str().unwrap_or("<INVALID_PATH>")),
            err.to_string(),
        )
    }

    /// average all frames in the image and place the result in the first frame
    /// for single frame images this shouldn't change anything. For images with multiple frames
    /// (animations) this will 'blur' any movement between the images
    pub fn average_frames(&mut self) {
        let mut t = self
            .buf
            .iter_mut()
            // convert to a data type that can hold higher numbers, so no overflow happens when
            // adding (u64::max() / u8::max() ~= 7.2e16; should be enough frames)
            .map(|frame| {
                frame
                    .iter()
                    .map(|px| [px[0] as u64, px[1] as u64, px[2] as u64])
                    .collect()
            })
            .reduce(|acc: Vec<[u64; 3]>, frame| {
                zip(acc, frame)
                    .map(|z| [z.0[0] + z.1[0], z.0[1] + z.1[1], z.0[2] + z.1[2]])
                    .collect()
            })
            .expect("Image should contain atleast one frame");
        let frames = self.buf.len() as u64;
        let t: Vec<_> = t
            .iter_mut()
            .map(|px| [px[0] / frames, px[1] / frames, px[2] / frames])
            .map(|px| [px[0] as u8, px[1] as u8, px[2] as u8])
            .collect();
        self.buf[0] = t;
    }

    /// Save the image as an animated png with the specified framerate
    /// for this to have any effect, the buffer should contain multiple frames
    pub fn save_apng(self, path: &mut PathBuf, fps: u16) -> Result<(), InputError> {
        path.set_extension("png");
        let file = File::create(&path)
            .map_err(|err| Self::io_err_to_input_err(err, path, "Error while saving image to"))?;
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
        encoder
            .set_animated(self.buf.len() as u32, 0)
            .map_err(|err| {
                Self::io_err_to_input_err(err.into(), path, "Error while saving image to")
            })?;
        encoder.set_frame_delay(1, fps).map_err(|err| {
            Self::io_err_to_input_err(err.into(), path, "Error while saving image to")
        })?;
        let mut writer = encoder.write_header().map_err(|err| {
            Self::io_err_to_input_err(err.into(), path, "Error while saving image to")
        })?;

        for frame in self.buf {
            writer
                .write_image_data(frame.as_flattened())
                .map_err(|err| {
                    Self::io_err_to_input_err(err.into(), path, "Error while saving image to")
                })?;
        }

        writer.finish().map_err(|err| {
            Self::io_err_to_input_err(err.into(), path, "Error while saving image to")
        })
    }

    /// Saves the image as a png image to the specified path
    /// If the path does not already have the .png extension, it will be added
    pub fn save_png(self, path: &mut PathBuf) -> Result<(), InputError> {
        path.set_extension("png");
        let file = File::create(&path)
            .map_err(|err| Self::io_err_to_input_err(err, path, "Error while saving image to"))?;
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
        let mut writer = encoder.write_header().map_err(|err| {
            Self::io_err_to_input_err(err.into(), path, "Error while saving image to")
        })?;

        writer
            .write_image_data(
                self.buf
                    .first()
                    .expect("image should contain atleast one frame")
                    .as_flattened(),
            )
            .map_err(|err| {
                Self::io_err_to_input_err(err.into(), path, "Error while saving image to")
            })?;

        writer.finish().map_err(|err| {
            Self::io_err_to_input_err(err.into(), path, "Error while saving image to")
        })
    }

    /// Saves the image as a ppm image to the specified path
    /// If the path does not already have the .ppm extension, it will be added
    pub fn save_ppm(self, path: &mut PathBuf) -> Result<(), InputError> {
        path.set_extension("ppm");
        let file = File::create(&path)
            .map_err(|err| Self::io_err_to_input_err(err, path, "Error while saving image to"))?;
        let mut w = BufWriter::new(file);

        w.write_all(format!("P6 {} {} 255\n", self.width, self.height).as_bytes())
            .map_err(|err| Self::io_err_to_input_err(err, path, "Error while saving image to"))?;

        for pixel in self
            .buf
            .first()
            .expect("image should contain atleast one frame")
            .as_slice()
        {
            w.write_all(pixel).map_err(|err| {
                Self::io_err_to_input_err(err, path, "Error while saving image to")
            })?;
        }

        Ok(())
    }
}
