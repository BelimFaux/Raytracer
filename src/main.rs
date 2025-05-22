fn main() {
    let width = 1920;
    let height = 1080;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([0u8, 0u8, 0u8]);
    }

    imgbuf.save("output/test.png").unwrap();
}
