use clap::Parser;
use colored::Colorize;
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
    // ignore lists
    // ignores: str,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let dot_dir_path = cli.path;
    let home_dir_path = cli.home;

    println!("");
    println!(
        "{} {}",
        "Dotfile directory:".bold(),
        &dot_dir_path.to_string_lossy().green()
    );
    println!(
        "{} {}",
        "Target home directory:".bold(),
        &home_dir_path.to_string_lossy().green()
    );

    println!("");
    println!("{}", "Dotfiles".bold());
    match fs::read_dir(&dot_dir_path) {
        Err(e) => eprintln!(
            "'{}' {}: {}",
            &dot_dir_path.to_string_lossy().bold().red(),
            "Can't read dotfile directory".red(),
            e
        ),
        Ok(paths) => {
            for path in paths {
                match path {
                    Err(e) => eprintln!("{:?}, {}: {}", e, "Can't access a path".red(), e),
                    Ok(p) => {
                        println!("{}", p.file_name().to_string_lossy());

                        let mut name = PathBuf::new();
                        name.push(&home_dir_path);
                        name.push(p.file_name());
                        println!("\t=> {:?}", &name.to_string_lossy());
                        match fs::read_link(Path::new(&name)) {
                            Ok(p) => println!("Link: {:?}", p),
                            Err(e) => println!(
                                "{} is not symlinked yet.: {:?}",
                                &name.to_string_lossy().red().bold(),
                                e
                            ),
                        }
                    }
                }
            }
        }
    }

    println!("");
    println!("{}", "---Read link test".bold());

    let test = Path::new("./foo.link");
    let x = fs::read_link(test)?;
    println!("{:?}", x);
    Ok(())
}
