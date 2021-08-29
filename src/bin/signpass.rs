#![cfg(feature = "cli")]

use clap::{AppSettings, Clap};
use std::{ffi::OsStr, fs::File, path::Path, process::exit};

use wallet_pass::sign;

/// Sign an apple wallet pass with a given certificate
#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Lars Westermann <git@lars-westermann.de>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Path to the pass directory
    #[clap(short = 'p', long = "pass")]
    pass_path: String,
    /// Path to the certificate
    #[clap(short = 'c', long = "certificate")]
    certificate_path: String,
    /// Certificate password
    #[clap(short = 'w', long = "password")]
    certificate_password: String,
    /// Path to the wwdr intermediate certificate
    #[clap(short = 'i', long = "intermediate")]
    wwdr_intermediate_certificate_path: String,
    /// File location for the output
    #[clap(short = 'o', long = "output")]
    output_path: Option<String>,
    /// Force pass signing by removing manifest and signiture if needed
    #[clap(short = 'f', long = "force")]
    force_pass_signing: bool,
}

pub fn main() {
    let opts: Opts = Opts::parse();

    let output_path = if let Some(path) = opts.output_path {
        path
    } else {
        let pass_name = Path::new(&opts.pass_path)
            .file_stem()
            .unwrap_or_else(|| OsStr::new("Pass"))
            .to_str()
            .unwrap_or("Pass");
        format!("{}.pkpass", pass_name)
    };

    let path = Path::new(&output_path);
    let file = File::create(&path).unwrap();
    if let Err(e) = sign::sign_path(
        Path::new(&opts.pass_path),
        None,
        Path::new(&opts.certificate_path),
        &opts.certificate_password,
        Path::new(&opts.wwdr_intermediate_certificate_path),
        file,
        opts.force_pass_signing,
    ) {
        eprintln!("{:?}", e);
        exit(1);
    }
}
