use clap::{Parser, Subcommand};
use std::process::{exit, Command, Stdio};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build {
        #[arg(short, long)]
        board: String,
    },
    Run {
        #[arg(short, long)]
        board: String,
    },
    ListBoards,
}

const BOARDS: &[(&str, &str, &str, &str)] = &[
    (
        "microbit",
        "ferrino/board+microbit",
        "nRF52833_xxAA",
        "thumbv7em-none-eabihf",
    ),
    (
        "rpi-pico",
        "ferrino/board+rpi-pico",
        "RP2040",
        "thumbv6m-none-eabi",
    ),
    (
        "rpi-pico-w",
        "ferrino/board+rpi-pico-w",
        "RP2040",
        "thumbv6m-none-eabi",
    ),
];

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::ListBoards => {
            println!("Supported boards:");
            for b in BOARDS {
                println!("\t{}", b.0);
            }
        }
        Commands::Build { board } => {
            let mut features = None;
            let mut target = None;
            for b in BOARDS {
                if b.0 == board {
                    features.replace(b.1.to_string());
                    target.replace(b.3.to_string());
                }
            }

            if features.is_none() || target.is_none() {
                println!("Selected board not found: {}", board);
                std::process::exit(-1);
            }

            let features = features.unwrap();
            let target = target.unwrap();

            let mut output = Command::new("cargo")
                .arg("build")
                .arg("--release")
                .arg("--target")
                .arg(target)
                .arg("--features")
                .arg(features)
                .stdin(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .expect("error building firmware");

            let status = output.wait();
            if let Ok(code) = status {
                exit(code.code().unwrap());
            } else {
                exit(-1)
            }
        }
        Commands::Run { board } => {
            let mut features = None;
            let mut chip = None;
            let mut target = None;
            for b in BOARDS {
                if b.0 == board {
                    features.replace(b.1.to_string());
                    chip.replace(b.2.to_string());
                    target.replace(b.3.to_string());
                }
            }

            if features.is_none() || chip.is_none() || target.is_none() {
                println!("Selected board not found: {}", board);
                std::process::exit(-1);
            }

            let features = features.unwrap();
            let target = target.unwrap();
            let chip = chip.unwrap();

            let mut output = Command::new("cargo")
                .arg("run")
                .arg("--release")
                .arg("--target")
                .arg(target)
                .arg("--features")
                .arg(features)
                .arg("--")
                .arg("--chip")
                .arg(chip)
                .stdin(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .expect("error running firmware");

            let status = output.wait();
            if let Ok(code) = status {
                exit(code.code().unwrap());
            } else {
                exit(-1)
            }
        }
    }
}
