use std::fs::File;
use std::fs;
use std::io::Write;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    action: String,
    file_name: String,
    extension: String,
}

// how to run:
// cargo run -- screen homeScreen tsx
// cargo run -- atom homeAtom ts
// cargo run -- component homeComponent tsx


fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let formated_file_name = format!("./{}/{}.{}", &args.action, &args.file_name, &args.extension);
    println!("file name: {}", formated_file_name);
    let _created_dir = fs::create_dir_all(format!("./{}", &args.action)).unwrap();

    let mut created_file = File::create(formated_file_name)?;
    created_file.write_all("Hello, world!".as_bytes())?;
    Ok(())
}