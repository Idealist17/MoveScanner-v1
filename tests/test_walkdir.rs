use std::path::{Path,PathBuf};
use walkdir::WalkDir;
use std::fs;
#[test]
pub fn test1(){
    let package_dir = WalkDir::new(Path::new("/home/wlq/Project/MoveBit/MoveScanner-v1/res/examples_project/sui/fungible_tokens"));
    // for entry in package_dir.into_iter().filter_map(|e| e.ok()){
    //     if entry.file_type().is_dir(){
    //         println!("dirs: {}", entry.path().display());
    //         println!("dirs: {:?}", entry.file_name());
    //     }
    // }

    Path::new("/home/wlq/Project/MoveBit/MoveScanner-v1/res/examples_project/sui/fungible_tokens:12");
    if let Ok(entries) = fs::read_dir(Path::new("/home/wlq/Project/MoveBit/MoveScanner-v1/res/examples_project/sui/fungible_tokens")) {
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.path().is_dir() {
                    println!("{}", entry.path().display());
                    println!("dirs: {:?}", entry.file_name());

                }
            }
        }
    }
}

