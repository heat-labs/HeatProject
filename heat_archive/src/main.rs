
use std::fs::{File};
use std::path::Path;
use clap::Parser;

/// Heat archive is an utility to pack heat byte code
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Location of the binary to pack
    #[clap(short, long)]
    binary: String,

    /// Location of the directory to store the archive
    #[clap(short, long, default_value = "./build")]
    archive_location: String,
}

fn main() {
    let args: Args = Args::parse();
    let build_location = Path::new(&args.archive_location);
    let binary_location = Path::new(&args.binary);

    let file = File::create(build_location.join("build.har")).unwrap();
    let mut f = tar::Builder::new(file);

    let mut bin = File::open(binary_location).unwrap();

    f.append_file("bin",&mut bin).unwrap();
}