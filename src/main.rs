use std::{env, path::PathBuf, process};

use lab3::{
    image,
    input::{file_to_scene, Config},
};

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
    let mut imgbuf = image::Image::new(width, height);

    for y in 0..height {
        for x in 0..width {
            *imgbuf.get_pixel_mut(x, y) = scene.trace_pixel(x, height - y).to_rgb();
        }
    }

    let mut outpath = PathBuf::new();
    outpath.push("output/");
    outpath.push(scene.get_output());

    imgbuf.save_png(&mut outpath).unwrap_or_else(|err| {
        eprintln!(
            "Error while saving image to '{}'\n{err}",
            outpath.to_str().unwrap_or("<INVALID PATH>")
        );
        process::exit(1);
    });

    println!(
        "Successfully saved image to {}",
        outpath.to_str().unwrap_or("<INVALID PATH>")
    );
}
