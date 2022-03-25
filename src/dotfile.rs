use anyhow::anyhow;
use std::os::unix::fs as unix_fs;
use std::{
    fs::{self, read_link},
    path::PathBuf,
    str::FromStr,
};

// #![warn(missing_docs)]

/// ファイルのリンク状態を表します
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum State {
    /// リンクされている状態です
    Linked,

    /// リンクされていない状態です
    Unliked,

    /// それ以外の状態です（例: 読み取り不可など）
    Other,
}

/// 設定ファイルを表す構造体です
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Dotfile {
    /// リンク元のパスです
    from: Option<PathBuf>,

    /// リンク先のパスです
    to: Option<PathBuf>,

    /// リンクの状態です
    state: State,
}

fn parse_tilde_and_dot(path: &PathBuf) -> anyhow::Result<PathBuf> {
    let tilde = PathBuf::from_str(shellexpand::tilde(path.to_string_lossy().as_ref()).as_ref())?;
    fs::canonicalize::<PathBuf>(tilde).map_err(|e| anyhow!(e))
}

impl Dotfile {
    pub fn new(from: Option<PathBuf>, to: Option<PathBuf>) -> Self {
        let parsed_from = from.map(|path| parse_tilde_and_dot(&path).ok()).flatten();
        let parsed_to = to.map(|path| parse_tilde_and_dot(&path).ok()).flatten();

        let status = match &parsed_from {
            Some(from) => {
                match parsed_to {
                    Some(_) => match read_link(from) {
                        Ok(_) => State::Linked,
                        Err(_) => {
                            // link error
                            State::Other
                        }
                    },
                    None => State::Unliked,
                }
            }
            None => {
                println!("from parsed failed");
                State::Unliked
            }
        };

        Dotfile {
            from: parsed_from,
            to: parsed_to,
            state: status,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{env::VarError, os::unix::fs::symlink, str::FromStr};

    #[test]
    fn new_none() {
        let actual = Dotfile::new(None, None);
        let expect = Dotfile {
            from: None,
            to: None,
            state: State::Unliked,
        };
        assert_eq!(actual, expect);
    }

    #[test]
    fn new_invalid_link() {
        let from = PathBuf::from_str("../foo/bar").ok();
        let to = PathBuf::from_str("../buz/qux").ok();
        println!(">>>>> {:?}, {:?}", from, to);

        let actual = Dotfile::new(from, to);
        let expect = Dotfile {
            from: None,
            to: None,
            state: State::Unliked,
        };
        assert_eq!(actual, expect);
    }

    #[test]
    fn new_valid_link() -> Result<(), VarError> {
        let from = PathBuf::from_str("./dot-test").ok();
        let to = PathBuf::from_str("./src").ok();
        println!(">>>>> {:?}, {:?}", from, to);

        let home = std::env::var("HOME")?;
        let actual = Dotfile::new(from, to);
        let expect = Dotfile {
            from: PathBuf::from_str(&home).ok(),
            to: PathBuf::from_str(format!("{}/work/src/github.com/aoki/dots/src", home).as_ref())
                .ok(),
            state: State::Linked,
        };
        assert_eq!(actual, expect);
        Ok(())
    }
}
