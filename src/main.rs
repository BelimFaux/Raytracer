use std::{env, path::PathBuf, process, sync::mpsc};

use lab3::{
    image,
    input::{file_to_scene, Config},
    misc::ProgressBar,
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

    let mut progress = ProgressBar::new((width * height) as usize);
    let (tx, rx) = mpsc::channel();

    // thread for printing progress bar
    // necessary, since `imgbuf.par_init_each_pixel(..)` blocks the thread
    std::thread::spawn(move || {
        while rx.recv().is_ok() {
            progress.next();
        }
    });

    imgbuf.par_init_pixels(|(x, y)| {
        let tx = tx.clone();
        // invert y to 'unflip' the image
        let ret = scene.trace_pixel(*x, height - *y).to_rgb();
        let _ = tx.send(1);
        ret
    });

    let mut outpath = PathBuf::new();
    outpath.push("output/");
    outpath.push(scene.get_output());

    imgbuf.save(&mut outpath).unwrap_or_else(|err| {
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
