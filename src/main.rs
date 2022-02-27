use clap::Parser;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Parser)]
#[clap(version, about)]
struct Cli {
    #[clap(
        short,
        long,
        parse(from_os_str),
        value_name = "CONFIG_DIRECTORY",
        env = "DOTS_CONFIG_DIR",
        default_value = "."
    )]
    path: PathBuf,

    #[clap(
        short,
        long,
        parse(from_os_str),
        value_name = "HOME_DIRECTORY",
        env = "DOTS_HOME_DIR",
        default_value = "~"
    )]
    home: PathBuf,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let dot_dir_path = cli.path;
    let home_dir_path = cli.home;

    println!("{:?}", dot_dir_path);
    println!("{:?}", home_dir_path);

    match fs::read_dir(dot_dir_path) {
        Err(e) => println!("{:?}", e.kind()),
        Ok(paths) => {
            for path in paths {
                match path {
                    Ok(p) => {
                        println!("{}", p.file_name().to_string_lossy().to_string());
                        let mut name = PathBuf::new();
                        name.push(&home_dir_path);
                        name.push(p.file_name());
                        println!("{:?}", &name);
                        fs::read_link(Path::new(&name)).is_err(println!("{:?}", &name););
                    }
                    Err(_) => (),
                }
            }
        }
    }

    let test = Path::new("./foo.link");
    let x = fs::read_link(test)?;
    println!("{:?}", x);
    Ok(())
}
