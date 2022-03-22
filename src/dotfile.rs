use anyhow::anyhow;
use std::{fs, path::PathBuf, str::FromStr};

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

        Dotfile {
            from: parsed_from,
            to: parsed_to,
            state: State::Other,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn new_none() {
        let actual = Dotfile::new(None, None);
        let expect = Dotfile {
            from: None,
            to: None,
            state: State::Other,
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
            state: State::Other,
        };
        assert_eq!(actual, expect);
    }

    #[test]
    fn new_valid_link() {
        let from = PathBuf::from_str("~").ok();
        let to = PathBuf::from_str("./src").ok();
        println!(">>>>> {:?}, {:?}", from, to);
        let actual = Dotfile::new(from, to);
        let expect = Dotfile {
            from: PathBuf::from_str("/Users/aoki").ok(), // TODO: Change the test path
            to: PathBuf::from_str("/Users/aoki/work/src/github.com/aoki/dots/src").ok(),
            state: State::Other,
        };
        assert_eq!(actual, expect);
    }
}
