use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

// Eg: cargo run -- main src/main.rs
fn main() {
    // call eg function
    // eg_function();

    // - cargo run -- route src/generated/generic_file.txt
    // find_pattern_in_file();
    
}

#[allow(dead_code)]
fn find_pattern_in_file() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}

#[allow(dead_code)]
fn eg_function() {
    /*mut anotation*/
    let mut a = 'a';
    println!("{}", a);
    a = 'b';
    println!("{}", a);
}
