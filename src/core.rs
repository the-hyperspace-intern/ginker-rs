use colored::*;
use std::path::Path;
use std::fs;

pub fn copy_template(template_folder: &Path, target: &Path) {
    if template_folder.exists() && template_folder.is_dir() {
        println!("{} {}", "Starting copy of : ".green().italic(), template_folder.display());

        let paths = fs::read_dir(&template_folder).unwrap();
        for entry in paths {
            let path = entry.unwrap().path();
            let pathname = path.file_name().unwrap();
            if !path.is_dir() {
                let dest_file = target.join(&pathname);
                match fs::copy(&path, dest_file) {
                    Err(e) => panic!("{:?}", e),
                    _ => println!("{}", path.display()),
                }
            } else {
                let new_target = target.join(&pathname);
                match std::fs::create_dir_all(&new_target) {
                    Err(e) => panic!("{:?}", e),
                    _ => println!("{} {}", "Creating folder : ".green(), new_target.display()),
                }
                copy_template(&path, &new_target);
            }
        }
    } else {
        println!("{} {}", "Template file not found in : ".yellow().bold(), template_folder.display())
    }
}