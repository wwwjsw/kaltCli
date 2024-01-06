use std::fs::File;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
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

fn create_file_buf(folder: &String, filename: &String, ext: &String) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(folder);
    path.push(filename);
    path.set_extension(ext);


    return path
}

fn check_if_folder_exists(folder: &String) -> bool {
    let folder_path = format!("./{}", folder);
    let exist_path = fs::metadata(&folder_path).is_ok();
    println!("exist path: {}", exist_path);
    return exist_path;
    // return fs::metadata(format!("./{}", folder)).unwrap().is_file();
}

fn create_folder(folder: &String) {
    fs::create_dir_all(format!("./{}", folder)).unwrap();
}


fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let [action, file_name, extension] = [args.action, args.file_name, args.extension];
    
    let file_buf = create_file_buf(&action, &file_name, &extension);
    let folder_exists = check_if_folder_exists(&action);
    // let file_exists = check_if_file_exists(&formated_file_name);


    // create folder if it doesn't exist
    if folder_exists {
        println!("folder exists");
    } else {
        create_folder(&action);
        println!("folder created");
    }

    if file_buf.exists() {
        println!("File exists!")
    } else {
        let mut _file = File::create(&file_buf).expect("Unable to create file");
        _file.write_all("Hello, world!".as_bytes()).expect("Unable to write data");
    // file.write_all("Hello, world!".as_bytes())?;
    }

    Ok(())
}