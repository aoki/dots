use clap::{Parser, Subcommand};
use colored::Colorize;
use std::{
    any,
    fs::{self, ReadDir},
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
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Test {},
    Link {},
    Unlink {},
}

fn test_symlink(paths: ReadDir, home_dir_path: &PathBuf) -> anyhow::Result<()> {
    for path in paths {
        match path {
            Err(e) => eprintln!("{:?}, {}: {}", e, "Can't access a path".red(), e),
            Ok(p) => {
                let mut name = PathBuf::new();
                name.push(&home_dir_path);
                name.push(p.file_name());
                match fs::read_link(Path::new(&name)) {
                    Ok(p) => println!(
                        "{} {}",
                        "✔︎".green().bold(),
                        p.file_name().unwrap().to_string_lossy()
                    ),
                    Err(_) => println!("{} {}", "✖︎".red().bold(), p.file_name().to_string_lossy()),
                }
            }
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let dot_dir_path = cli.path;
    let home_dir_path = cli.home;
    let ignore_file_list = cli.ignores.split(",").map(|e| e.to_string()).collect();
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

    println!("");
    println!("{}", "Dotfiles".bold());
    match fs::read_dir(&dot_dir_path) {
        Err(e) => eprintln!(
            "'{}' {}: {}",
            &dot_dir_path.to_string_lossy().bold().red(),
            "Can't read dotfile directory".red(),
            e
        ),
        Ok(paths) => match &cli.command {
            Some(Commands::Test {}) | None => test_symlink(paths, &home_dir_path)?,
            Some(Commands::Link {}) => {
                println!(
                    "{}",
                    "Create symlink in home directory from dot config directory".green()
                );
                create_symlink(paths, &home_dir_path, &ignore_file_list);
            }
            Some(Commands::Unlink {}) => {
                println!("{}", "Remove sysmlink in home directory".green())
            }
        },
    }
    Ok(())
}

fn ignore_filter(paths: ReadDir, ignore_file_list: &Vec<String>) -> anyhow::Result<()> {
    println!("IGNOREFILTER");
    println!(
        "Paths: {:?}",
        paths
            .filter_map(|entry| {
                println!("{:?}", entry);
                entry.ok().and_then(|e| {
                    e.path()
                        .file_name()
                        .and_then(|n| n.to_str().map(|s| String::from(s)))
                })
            })
            .collect::<Vec<String>>()
    );
    Ok(())
}

fn create_symlink(
    paths: ReadDir,
    home_dir_path: &PathBuf,
    ignore_file_list: &Vec<String>,
) -> anyhow::Result<()> {
    // filter
    home_dir_path;
    ignore_filter(paths, &ignore_file_list);
    Ok(())
}
