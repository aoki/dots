use anyhow::anyhow;
use std::{
    fs::{self},
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

    /// 無視されたファイルです
    Ignored,

    /// エラー
    Error,

    /// それ以外の状態です（例: 読み取り不可など）
    Other,
}

/// 設定ファイルを表す構造体です
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Dotfile {
    /// リンク元のパスです
    pub from: Option<PathBuf>,

    /// リンク先のパスです
    pub to: Option<PathBuf>,

    /// リンクの状態です
    pub state: State,
}

fn parse_tilde_and_dot(path: &PathBuf) -> anyhow::Result<PathBuf> {
    let s = path.to_string_lossy();
    let tilde = PathBuf::from_str(&shellexpand::tilde(&s)).map_err(|e| anyhow!(e))?;
    let dot = fs::canonicalize::<PathBuf>(tilde).map_err(|e| anyhow!(e));
    dot
}

impl Dotfile {
    pub fn new(from_dir: Option<PathBuf>, to_dir: Option<PathBuf>, file: &PathBuf) -> Self {
        let parsed_from_dir = from_dir
            .map(|path| parse_tilde_and_dot(&path).ok())
            .flatten();
        let parsed_to_dir = to_dir.map(|path| parse_tilde_and_dot(&path).ok()).flatten();

        println!("FROM: {:?}", &parsed_from_dir);
        println!("  TO: {:?}", &parsed_to_dir);

        let to = parsed_to_dir.map(|dir| PathBuf::new().join(&dir).join(&file));
        let from = parsed_from_dir.map(|dir| PathBuf::new().join(&dir).join(&file));

        Dotfile {
            from: from,
            to: to,
            state: State::Other,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn resolve_path_wtih_dot_and_tilde() {
    //     // unimplemented!()
    // }
    // #[test]
    // fn new_none() {
    //     let actual = Dotfile::new(None, None);
    //     let expect = Dotfile {
    //         from: None,
    //         to: None,
    //         state: State::Unliked,
    //     };
    //     assert_eq!(actual, expect);
    // }

    // #[test]
    // fn new_invalid_link() {
    //     let from = PathBuf::from_str("../invalid/link").ok();
    //     let to = PathBuf::from_str("../invalid/link").ok();

    //     let actual = Dotfile::new(from, to);
    //     let expect = Dotfile {
    //         from: None,
    //         to: None,
    //         state: State::Unliked,
    //     };
    //     assert_eq!(actual, expect);
    // }

    #[test]
    fn new_valid_link() {
        let dotfile_name = PathBuf::from(".samplerc");
        let valid_from = PathBuf::from("./test-resources/test-home");
        let valid_to = PathBuf::from("./test-resources/test-conf.d");

        // Ignored
        let expect = Dotfile {
            from: Some(valid_from.clone()),
            to: Some(valid_to.clone()),
            state: State::Ignored,
        };
        let actual = Dotfile::new(
            Some(valid_from.clone()),
            Some(valid_to.clone()),
            &dotfile_name,
        );
        assert_eq!(actual, expect);

        // None None
        let expect = Dotfile {
            from: None,
            to: None,
            state: State::Other,
        };
        let actual = Dotfile::new(None, None, &dotfile_name);
        assert_eq!(actual, expect);

        // None Some(Valid)
        let expect = Dotfile {
            from: None,
            to: Some(valid_to.clone()),
            state: State::Unliked,
        };
        let actual = Dotfile::new(None, Some(valid_to.clone()), &dotfile_name);
        assert_eq!(actual, expect);
    }
}
