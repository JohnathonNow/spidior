use walkdir::WalkDir;
use std::fs;

fn main() -> Result<(), ()> {
    for entry in WalkDir::new(".")
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {

        let path = entry.path();
        if path.is_file() {
            let contents = fs::read_to_string(path).expect("");
             
            let f_name = entry.file_name().to_string_lossy();
            println!("{}\n{}", f_name, contents);
        }
    }

    Ok(())
}
