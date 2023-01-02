use structopt::StructOpt; // command argument parser
use std::fs::File;
use std::path::PathBuf;
use std::io::Read; // when i dont add this, compiler giving an error on windows

#[derive(StructOpt)]
#[structopt(name = "catrs")] // declaring what is the command
struct Cat {
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>, // creating a vector for multiple file choices
}

fn main() {
    let args = Cat::from_args();
    for file in args.files {
        let mut f = File::open(file).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("something went wrong reading the file");
        println!("{}", contents);
    }
}
