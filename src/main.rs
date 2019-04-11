#[macro_use]
extern crate clap;
extern crate colored;

use std::env;
use colored::*;

mod app;
mod core;

fn main() {
    let matches = app::build().get_matches();

    let template_type = matches
        .value_of("TEMPLATE_TYPE")
        .expect("failted to retrieve the template type");

    let project_name = matches
        .value_of("PROJECT_NAME")
        .expect("failed to retrieve the");

    let project_path = env::current_dir().unwrap().join(&project_name);
    if project_path.exists() {
        println!("{}", "Project folder already exists".yellow().bold());
    } else {
        println!("{}", "Ready to create the project boilerplate".green());

        std::fs::create_dir_all(&project_path).expect("Couldn't create the project folder");

        match dirs::home_dir() {
            Some(home_path) => {
                let home_template = home_path.join("Templates");
                if home_template.exists() {
                    let template_path = home_template.join(&template_type);
                    core::copy_template(&template_path, &project_path);
                } else {
                    println!("{}", "Couldn't find any Templates, retry after creating templates.".yellow().bold())
                }
            },
            None => println!("{}", "Couldn't find any HOME folder".yellow().bold())
        }
    }
}