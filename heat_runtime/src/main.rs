
use std::fs::{File, read};
use clap::Parser;
use uuid::Uuid;
use libvirt::constraints::Constraints;
use libvirt::frame::Frame;
use libvirt::instruction::Instruction;
use libvirt::interpreter::Interpreter;

/// The heat runtime is an program to execute heat bin package files
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Location of the file to run
    #[clap(short, long)]
    file: String,

    /// Maximum allocations per stack (bits) NOTE: set to 0 to turn off limit
    #[clap(short, long, default_value_t = 0)]
    max_stack_allocation: u64,
}

fn main() {
    let args: Args = Args::parse();

    let f = File::open(args.file);
    if f.is_err() {
        eprintln!("{}", f.unwrap_err());
        return;
    }

    let f = f.unwrap();
    let mut archive = tar::Archive::new(f);

    let extract_location = std::env::temp_dir().join(Uuid::new_v4().to_simple().to_string());
    archive.unpack(&extract_location).unwrap();

    let bin_file = read(extract_location.join("/bin")).unwrap();

    let bin_file_iter = bin_file.split(|num| num % lib_heat_spec::instruction::SIZE == 0);


    let i = Interpreter::new(Constraints::new(0, args.max_stack_allocation));
    let mut main_frame = Frame::default();

    for encoded_instruction in bin_file_iter {
        main_frame.instructions.push(Instruction::from(encoded_instruction));
    }

    i.execute_frame(&mut main_frame);
}
