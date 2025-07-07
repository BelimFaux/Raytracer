use std::{env, path::PathBuf, process, sync::mpsc};

use rt::{
    image,
    input::{file_to_scene, Config, InputError},
    misc::progress::ProgressBar,
};

fn main() -> process::ExitCode {
    match run() {
        Ok(()) => process::ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            process::ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), InputError> {
    let args: Vec<_> = env::args().collect();
    let config = Config::build(&args)?;
    if config.is_none() {
        return Ok(());
    }
    // is safe, since we asserted that config is not None
    let config = unsafe { config.unwrap_unchecked() };

    let scene = file_to_scene(config.get_input())?;
    let (width, height) = scene.get_dimensions();
    println!(
        "Loaded file '{}'; starting render with dimensions: {}x{}.",
        config.get_input(),
        width,
        height
    );

    let mut img = image::Image::new(width, height);

    let (tx, rx) = mpsc::channel();

    // start thread for printing progress bar
    // necessary, since `img.par_init_each_pixel(..)` blocks the main thread
    if config.progress_bar() {
        let mut progress = ProgressBar::new((width * height) as usize);

        std::thread::spawn(move || {
            while rx.recv().is_ok() {
                progress.next();
            }
        });
    }

    // render image
    img.par_init_pixels(|(x, y)| {
        let tx = tx.clone();
        // invert y to 'unflip' the image
        let ret = scene.trace_pixel(*x, height - *y).to_rgb();
        let _ = tx.send(());
        ret
    });

    let mut outpath = PathBuf::new();
    outpath.push(config.outdir());
    outpath.push(scene.get_output());

    if config.ppm() {
        img.save_ppm(&mut outpath)?
    } else {
        img.save_png(&mut outpath)?
    };

    println!(
        "Successfully saved image to {}",
        outpath.to_str().unwrap_or("<INVALID PATH>")
    );

    Ok(())
}
