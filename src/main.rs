use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use skim::{
    prelude::{SkimItemReader, SkimOptionsBuilder},
    Skim, SkimItem,
};
use std::{
    collections::HashSet,
    fs::{self, ReadDir},
    io::Cursor,
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
        default_value = ".DS_Store,.gitignore,.sample.yml"
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

fn finder(file_list: &Vec<String>) -> anyhow::Result<Vec<String>> {
    let skim_options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .build()
        .map_err(|e| anyhow!(e))?;

    let item_reader = SkimItemReader::default().of_bufread(Cursor::new(file_list.join("\n")));
    let selected_items = Skim::run_with(&skim_options, Some(item_reader))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    for item in selected_items.iter() {
        println!("Selected item: {}", item.output());
    }
    Ok((vec![]))
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

fn display_target_info(
    dot_dir_path: &PathBuf,
    home_dir_path: &PathBuf,
    ignore_file_list: &HashSet<String>,
) -> anyhow::Result<()> {
    println!(
        "\n{} {}",
        "Dotfile directory:".bold(),
        &dot_dir_path.to_string_lossy().green()
    );
    println!(
        "{} {}",
        "Target home directory:".bold(),
        &home_dir_path.to_string_lossy().green()
    );
    println!(
        "{} {}",
        "Ignore files:".bold(),
        ignore_file_list
            .clone()
            .into_iter()
            .collect::<Vec<String>>()
            .join(",")
            .green()
    );
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let dot_dir_path = cli.path;
    let home_dir_path = cli.home;
    let ignore_file_list: HashSet<String> = cli
        .ignores
        .split(",")
        .into_iter()
        .map(|e| e.to_string())
        .collect();

    display_target_info(&dot_dir_path, &home_dir_path, &ignore_file_list)?;

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
                let res = create_symlink(paths, &home_dir_path, &ignore_file_list)?;
                let r = finder(&res)?;
                println!("{:?}", res);
            }
            Some(Commands::Unlink {}) => {
                println!("{}", "Remove sysmlink in home directory".green())
            }
        },
    }
    Ok(())
}

fn create_symlink(
    paths: ReadDir,
    home_dir_path: &PathBuf,
    ignore_file_list: &HashSet<String>,
) -> anyhow::Result<Vec<String>> {
    let file_list: Vec<String> = paths
        .filter(|path| path.is_ok())
        .map(|path| path.unwrap())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();

    let filtered_list = file_filter(&file_list, ignore_file_list)?;

    Ok(filtered_list)
}

fn file_filter(
    target_list: &Vec<String>,
    ignore_list: &HashSet<String>,
) -> anyhow::Result<Vec<String>> {
    Ok(target_list
        .iter()
        .filter(|&f| !ignore_list.contains(f))
        .map(|f| f.clone())
        .collect())
}
