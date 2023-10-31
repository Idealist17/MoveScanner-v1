use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::process::Command;
use ansi_term::Colour;
pub fn compile(source_path: &PathBuf) -> bool {
    let project_path = source_path.parent().unwrap();
    let toml_path = project_path.join("Move.toml");
    let mut build_command = "";

    if let Ok(file) = File::open(toml_path) {
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents).unwrap();

        if contents.contains("sui") {
            // println!("sui project");
            build_command = "sui move build";
        } else if contents.contains("aptos") {
            // println!("move project");
            build_command = "aptos move compile";
        } else {
            println!("not a valid move project");
            return false;
        }
    }
    let output = Command::new("sh")
        .current_dir(project_path)
        .arg("-c")
        .arg(build_command)
        .output();
    if let Ok(result) = output {
        if result.status.success() {
            println!("{}", Colour::Green.paint(format!("compile {} success", project_path.display())));
            return true;
        }
    }
    println!("{}", Colour::Red.paint(format!("compile {} failed", project_path.display())));
    // println!("compile {} failed", project_path.display());
    return false;
}
