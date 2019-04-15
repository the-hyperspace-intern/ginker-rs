extern crate dirs;
extern crate colored;

use colored::*;

use std::env;
use std::fs;
use std::path::Path;

const TEMPLATE_PROJECT: &str = "p9";

fn main() {
    let targetpath = env::current_dir().unwrap().join("jink");

    if targetpath.exists() {
        println!("{}", "Project folder already exists".yellow().bold());
    } else {
        println!("{}", "Ready to create project.".green().bold().italic());
        match std::fs::create_dir_all(&targetpath) {
            Err(e) => panic!("{:?}", e),
            _ => println!("{} : {}", "Creating Project folder".green(), targetpath.display())
        }
        match dirs::home_dir() {
            Some(path) => {
                let template_dir = path.join("Templates");
                if template_dir.exists() {
                    let template_folder = template_dir.join(&TEMPLATE_PROJECT);
                    copy_template(&template_folder, &targetpath)
                } else {
                    println!("{} {}" , "Couldn't find any Templates folder in : ".yellow().bold(), path.display())
                }
            }
            None => println!("{}", "Couldn't find any HOME folder".yellow().bold()),
        }
    }
}

fn copy_template(template_folder: &Path, target: &Path) {
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
    /*         match fs::copy(&template_file, &dest_file) {
        Err(e) => panic!("{:?}", e),
        _ => println!("Succesfully copied")
    } */
    } else {
        println!("{} {}", "Template file not found in : ".yellow().bold(), template_folder.display())
    }
}
