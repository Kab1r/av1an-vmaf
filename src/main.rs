use std::{env::temp_dir, fs::copy, path::Path};

use av1an_core::vmaf::{plot_vmaf_score_file, run_vmaf};
use clap::{arg, command, value_parser};

fn main() {
    let cmd = command!().args([
        arg!(-d --distorted <FILE> "distorted file").required(true),
        arg!(-r --reference <FILE> "reference file").required(true),
        arg!(-o --output <FILE> "output json or svg plot file").required(true),
        arg!(-m --model <MODEL> "vmaf model to use").required(false),
        arg!(--res <RES> "resolution to run VMAF with")
            .required(false)
            .default_value("1920x1080"),
        arg!(-s --scaler <SCALER> "scaler to use")
            .required(false)
            .default_value("bicubic"),
        arg!(-t --threads <THREADS> "number of threads to use")
            .required(false)
            .default_value("0")
            .value_parser(value_parser!(usize)),
    ]);
    let matches = cmd.get_matches();
    env_logger::init();

    let distorted = matches.get_one::<String>("distorted").unwrap();
    let reference = matches.get_one::<String>("reference").unwrap();
    let output = matches.get_one::<String>("output").unwrap();
    let model = matches.get_one::<String>("model");
    let res = matches.get_one::<String>("res").unwrap();
    let scaler = matches.get_one::<String>("scaler").unwrap();
    let threads: &usize = matches.get_one("threads").unwrap();

    let stat_file = {
        let tmp = temp_dir();
        tmp.join(distorted).with_extension("json")
    };

    let (distorted, output) = (Path::new(distorted), Path::new(output));
    log::info!("Running VMAF on {} and {}", distorted.display(), reference);
    if let Err(e) = run_vmaf(
        distorted,
        &[
            "ffmpeg",
            "-i",
            reference,
            "-strict",
            "-1",
            "-f",
            "yuv4mpegpipe",
            "-",
        ],
        &stat_file,
        model,
        res,
        scaler,
        1,
        None,
        *threads,
    ) {
        eprintln!("Error running VMAF: {}", e);
    };
    if output.extension().unwrap_or_default() == "svg" {
        log::info!("Plotting VMAF score to {}", output.display());
        if let Err(e) = plot_vmaf_score_file(&stat_file, output) {
            eprintln!("Error plotting VMAF: {}", e);
        }
    } else {
        log::info!(
            "Copying VMAF score file from {} to {}",
            stat_file.display(),
            output.display()
        );
        if let Err(e) = copy(&stat_file, output) {
            eprintln!("Error copying VMAF score file: {}", e);
        }
    }
}
