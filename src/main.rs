use lab3::math::Color;

fn main() {
    let width = 256;
    let height = 256;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = x as f32 / (width - 1) as f32;
        let g = y as f32 / (height - 1) as f32;
        let b = 0.2;
        let color = Color::new(r, g, b);

        *pixel = color.to_rgb();
    }

    imgbuf.save("output/test.png").unwrap();
}
