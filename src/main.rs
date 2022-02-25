use clap::Parser;
use std::{env, fs, path::PathBuf};

#[derive(Parser)]
#[clap(version, about)]
struct Cli {
    #[clap(short, long, parse(from_os_str), value_name = "CONFIG_DIRECTORY")]
    path: Option<PathBuf>,

    #[clap(
        short,
        long,
        parse(from_os_str),
        value_name = "HOME_DIRECTORY",
        default_value = "~"
    )]
    home: PathBuf,

    #[clap(
        short,
        long,
        value_name = "ENV",
        env = "DOTS_ENV",
        default_value = "def"
    )]
    en: String,
}

fn main() {
    let cli = Cli::parse();

    println!("{:?}", cli.en);

    let dot_dir_path = match cli.path {
        Some(path) => path,
        None => match env::var("DOTS_CONFIG_DIR") {
            Ok(p) => PathBuf::from(p),
            Err(_) => PathBuf::from("."),
        },
    };

    // let home_dir_path = match cli.home {
    //     Some(h) => h,
    //     None => match env::var("DOTS_HOME_DIR") {
    //         Ok(h) => PathBuf::from(h),
    //         Err(_) => PathBuf::from("~"),
    //     },
    // };

    println!("{:?}", dot_dir_path);

    match fs::read_dir(dot_dir_path) {
        Err(e) => println!("{:?}", e.kind()),
        Ok(paths) => {
            for path in paths {
                match path {
                    Ok(p) => println!(
                        "{}",
                        p.path().into_os_string().to_string_lossy().to_string()
                    ),
                    Err(_) => (),
                }
                // println!("{:?}", path);
            }
        }
    }
}
