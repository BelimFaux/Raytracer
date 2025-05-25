use lab3::input::file_to_scene;

fn main() {
    let scene = file_to_scene("scenes/example1.xml");
    let (width, height) = scene.get_dimensions();
    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = scene.trace_pixel(x, height - y).to_rgb()
    }

    let mut outpath = "output/".to_string();
    outpath.push_str(scene.get_output());
    imgbuf.save(outpath).unwrap();
}
