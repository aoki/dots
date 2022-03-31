use anyhow::anyhow;
use clap::{Parser, Subcommand};
use colored::Colorize;
use skim::{
    prelude::{SkimItemReader, SkimOptionsBuilder},
    Skim,
};
use std::{
    collections::HashSet,
    fmt::Display,
    fs::{self, read_link},
    io::Cursor,
    path::PathBuf,
    str::FromStr,
};
use std::{fs::remove_file, os::unix::fs as unix_fs};

fn canonicalize(path: &PathBuf) -> anyhow::Result<PathBuf> {
    let s = path.to_string_lossy();
    let tilde = PathBuf::from_str(&shellexpand::tilde(&s))?;
    fs::canonicalize::<PathBuf>(tilde).map_err(|e| anyhow!(e))
}

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
        default_value = "test-resources/test-conf.d"
    )]
    path: PathBuf,

    #[clap(
        short,
        long,
        parse(from_os_str),
        value_name = "HOME_DIRECTORY",
        env = "DOTS_HOME_DIR",
        // default_value = "~",
        default_value = "test-resources/test-home"
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

fn finder(file_list: Vec<Dot>) -> anyhow::Result<Vec<Dot>> {
    let skim_options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .build()
        .map_err(|e| anyhow!(e))?;

    let file_list_string: String = file_list
        .iter()
        .map(|file| file.file_name())
        .collect::<Vec<String>>()
        .join("\n");
    let item_reader = SkimItemReader::default().of_bufread(Cursor::new(file_list_string));
    let selected_items = Skim::run_with(&skim_options, Some(item_reader))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());
    let selected = selected_items
        .iter()
        .map(|i| i.output().to_string())
        .collect::<HashSet<String>>();

    Ok(file_list
        .iter()
        .filter(|file| selected.contains(&file.file_name()))
        .map(|file| file.clone())
        .collect::<Vec<Dot>>())
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum DotState {
    Linked,
    Unlinked,
    Ignored,
    LinkedToOtherDirctory,

    Error,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Dot {
    from: Option<PathBuf>,
    to: Option<PathBuf>,
    file: Option<String>,
    state: DotState,
    // e: Option<Error>,
}

impl Dot {
    fn file_name(&self) -> String {
        self.file.clone().unwrap_or("".to_string())
    }
    fn from(&self) -> anyhow::Result<PathBuf> {
        let file = match self.file.clone() {
            Some(f) => Ok(f),
            None => Err(anyhow!("file not found")),
        }?;
        match self.from.clone() {
            Some(from) => Ok(from.join(file)),
            None => Err(anyhow!("path not found")),
        }
    }
    fn to(&self) -> anyhow::Result<PathBuf> {
        let file = match self.file.clone() {
            Some(f) => Ok(f),
            None => Err(anyhow!("file not found")),
        }?;
        match self.to.clone() {
            Some(to) => Ok(to.join(file)),
            None => Err(anyhow!("path not found")),
        }
    }
}

fn display_target_info(
    dot_dir_path: &PathBuf,
    home_dir_path: &PathBuf,
    ignore_file_list: &String,
) -> () {
    println!(
        "\n{} {}",
        "    Dotfile directory:".bold(),
        &dot_dir_path.to_string_lossy().green()
    );
    println!(
        "{} {}",
        "Target home directory:".bold(),
        &home_dir_path.to_string_lossy().green()
    );
    println!(
        "{} {}",
        "         Ignore files:".bold(),
        ignore_file_list.green()
    );
}

impl Display for Dot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file = self.file.as_ref().unwrap_or(&"".to_string()).clone();
        let icon = match self.state {
            DotState::Linked => ("✔︎".green().bold(), file.white()),
            DotState::Unlinked => ("✖︎".red().bold(), file.white()),
            DotState::Ignored => ("-".black().bold(), format!("{}: ignored", file).black()),
            DotState::LinkedToOtherDirctory => (
                "-".black().bold(),
                format!("{}: already linked to other file", file).black(),
            ),
            DotState::Error => ("-".black().bold(), file.red()),
        };
        write!(f, "{} {}", icon.0, icon.1)
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Resolve `~`, `.`, `..` and link.
    // Guaranteed existence.
    // TODO: error handling
    let dot_dir_path = canonicalize(&cli.path)?;
    let home_dir_path = canonicalize(&cli.home)?;

    let ignore_file_list: HashSet<String> = cli
        .ignores
        .split(",")
        .into_iter()
        .map(|e| e.to_string())
        .collect();

    display_target_info(&dot_dir_path, &home_dir_path, &cli.ignores);

    // TODO: Error handling
    let read_dir = fs::read_dir(&dot_dir_path)?;
    let dotfile_entry: Vec<Dot> = read_dir
        .into_iter()
        .map(|res| match res {
            Ok(entry) => {
                let file = entry.file_name();
                let full_from = PathBuf::from(&home_dir_path).join(&file);
                let full_to = PathBuf::from(&dot_dir_path).join(&file);

                let state = if ignore_file_list.contains(&file.to_string_lossy().to_string()) {
                    DotState::Ignored
                } else {
                    // Cehck symlinks in the home_dir
                    match read_link(&full_from) {
                        Ok(to) => {
                            if to == full_to {
                                DotState::Linked
                            } else {
                                DotState::LinkedToOtherDirctory
                            }
                        }
                        Err(_) => DotState::Unlinked,
                    }
                };

                Dot {
                    from: Some(home_dir_path.clone()),
                    to: Some(dot_dir_path.clone()),
                    file: Some(entry.file_name().to_string_lossy().to_string()),
                    state: state,
                }
            }
            Err(_) => Dot {
                from: None,
                to: None,
                file: None,
                state: DotState::Error,
            },
        })
        .collect();

    match &cli.command {
        Some(Commands::Test {}) | None => {
            println!("{}", "Dot-files status".green().bold());
            dotfile_entry.iter().for_each(|entry| println!("{}", entry));
        }
        Some(Commands::Link {}) => {
            println!(
                "{}",
                "Create a symlink in the home directory to the dot config directory".green()
            );
            let unlinked = dotfile_entry
                .iter()
                .filter(|z| z.state == DotState::Unlinked)
                .map(|file| file.clone())
                .collect::<Vec<Dot>>();
            let selected_items = finder(unlinked)?;
            selected_items
                .iter()
                .for_each(|entry| println!("{}", entry));

            create_symlink(selected_items)?;
        }
        Some(Commands::Unlink {}) => {
            println!("{}", "Remove sysmlink in home directory".green());

            let linked = dotfile_entry
                .iter()
                .filter(|z| z.state == DotState::Linked)
                .map(|file| file.clone())
                .collect::<Vec<Dot>>();

            let selected_items = finder(linked)?;
            selected_items
                .iter()
                .for_each(|entry| println!("{}", entry));

            remove_symlink(selected_items);
        }
    }

    Ok(())
}

fn create_symlink(target_list: Vec<Dot>) -> anyhow::Result<Vec<()>> {
    target_list
        .iter()
        .map(|file| {
            let from: PathBuf = file.from()?;
            let to: PathBuf = file.to()?;
            unix_fs::symlink(&to, &from).map_err(|e| anyhow!(e))
        })
        .collect::<anyhow::Result<Vec<_>>>()
}

fn remove_symlink(target_list: Vec<Dot>) -> Vec<anyhow::Result<()>> {
    target_list
        .iter()
        .map(|file| {
            let from: PathBuf = file.from()?;
            remove_file(&from).map_err(|e| anyhow!(e))
        })
        .collect::<Vec<anyhow::Result<_>>>()
}
