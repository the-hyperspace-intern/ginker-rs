use clap::{Arg, App};

pub fn build() -> App<'static, 'static> {
    App::new("ginker")
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("PROJECT_NAME")
                .help("Name of the Project you're initializing")
                .required(true)
                .index(1))
        .arg(Arg::with_name("TEMPLATE_TYPE")
                .help("Name of the template type you want to initialize")
                .required(true)
                .index(2))
}