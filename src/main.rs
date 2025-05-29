use std::{env, process};

use lab3::input::{file_to_scene, Config};

fn main() {
    let args: Vec<_> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Error while parsing arguments:\n{err}");
        process::exit(1);
    });

    let scene = file_to_scene(config.get_input()).unwrap_or_else(|err| {
        eprintln!("Error while parsing file '{}':\n{err}", config.get_input());
        process::exit(1);
    });

    let (width, height) = scene.get_dimensions();
    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // height - y to unflip the image
        *pixel = scene.trace_pixel(x, height - y).to_rgb()
    }

    let mut outpath = "output/".to_string();
    outpath.push_str(scene.get_output());
    imgbuf.save(&outpath).unwrap_or_else(|err| {
        eprintln!("Error while saving image to '{outpath}'\n{err}");
        process::exit(1);
    });
    println!("Successfully saved image to {outpath}");
}
