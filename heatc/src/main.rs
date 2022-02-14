mod compiler;

use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::Path;
use clap::Parser;
use crate::compiler::Instruction;

/// The heat compiler is an program to compile HeatASM files to Heat byte code
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Location of the source files to compile
    #[clap(short, long)]
    sources: Vec<String>,

    /// Location of the directory to store the compiled files
    #[clap(short, long, default_value = "./build")]
    build_location: String,
}

fn main() {
    let args: Args = Args::parse();
    let build_location = Path::new(&args.build_location);

    for source in args.sources {
        let source = Path::new(&source);
        let contents = read_to_string(&source).unwrap();
        let lines: Vec<&str> = contents.split("\n").collect();

        let mut file = File::create(build_location.join(source.file_stem().unwrap())).unwrap();

        for line_index in 0..lines.len() {
            let instruction = Instruction::from(lines[line_index].to_string());
            let instruction = match instruction{
                Ok(i) => i,
                Err(err) => {
                    panic!("{}:{}:0 {}", &source.display(), line_index, err);
                },
            };

            let byte_code = instruction.to_byte_code();
            let byte_code = match byte_code {
                Ok(b) => b,
                Err(err) => {
                    panic!("{}:{}:0 {}", &source.display(), line_index, err);
                },
            };

            file.write(&byte_code).unwrap();
        }
    }
}