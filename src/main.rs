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

    let mut scene = file_to_scene(config.get_input())?;
    let (width, height) = scene.get_dimensions();
    println!(
        "Loaded file '{}'; starting render with dimensions: {}x{}.",
        config.get_input(),
        width,
        height
    );

    let mut img = image::Image::new(width, height, scene.get_frames());

    let (tx, rx) = mpsc::channel();

    // start thread for printing progress bar
    // necessary, since `img.par_init_each_pixel(..)` blocks the main thread
    if config.progress_bar() {
        let mut frame = 1;
        let mut progress = ProgressBar::new((width * height) as usize, String::from("Frame 1:"));

        std::thread::spawn(move || {
            while let Ok(r) = rx.recv() {
                if r == 1 {
                    frame += 1;
                    progress.reset(format!("Frame {}:", frame));
                } else {
                    progress.next();
                }
            }
        });
    }

    // render image
    for frame in 0..scene.get_frames() {
        img.par_init_pixels(frame, |(x, y)| {
            let tx = tx.clone();
            // invert y to 'unflip' the image
            let ret = scene.trace_pixel(*x, height - *y).to_rgb();
            let _ = tx.send(0u8);
            ret
        });
        scene.next_frame();
        let _ = tx.send(1u8);
    }

    let mut outpath = PathBuf::new();
    outpath.push(config.outdir());
    outpath.push(scene.get_output());

    if scene.is_animated() {
        img.save_apng(&mut outpath)?;
    } else if config.ppm() {
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
