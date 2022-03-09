use clap::{Parser, Subcommand};
use colored::Colorize;
use std::{
    fs::{self},
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
        // default_value = "."
        default_value = "test-conf.d"
    )]
    path: PathBuf,

    #[clap(
        short,
        long,
        parse(from_os_str),
        value_name = "HOME_DIRECTORY",
        env = "DOTS_HOME_DIR",
        // default_value = "`",
        default_value = "test-home"
    )]
    home: PathBuf,

    #[clap(
        short,
        long,
        value_name = "IGNORE_FILE_LIST",
        env = "DOTS_IGNORE_FILES",
        default_value = ".DS_Store,.gitignore"
    )]
    ignores: String,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Test {},
    Link {},
    Unlink {},
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let dot_dir_path = cli.path;
    let home_dir_path = cli.home;
    let ignore_file_list: Vec<&str> = cli.ignores.split(",").collect();
    println!("{:?}", &ignore_file_list);

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

    match &cli.command {
        Commands::Test {} => println!("{}", "Show current link status".green()),
        Commands::Link {} => println!(
            "{}",
            "Create symlink in home directory from dot config directory".green()
        ),
        Commands::Unlink {} => println!("{}", "Remove sysmlink in home directory".green()),
    }

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
                        let mut name = PathBuf::new();
                        name.push(&home_dir_path);
                        name.push(p.file_name());
                        match fs::read_link(Path::new(&name)) {
                            Ok(p) => {
                                println!(
                                    "{} {}",
                                    "✔︎".green().bold(),
                                    p.file_name().unwrap().to_string_lossy()
                                )
                            }
                            Err(_) => {
                                println!("{} {}", "✖︎".red().bold(), p.file_name().to_string_lossy())
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
