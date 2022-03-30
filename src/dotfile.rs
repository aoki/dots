use anyhow::anyhow;
use std::{fs::read_link, path::PathBuf, str::FromStr};

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
    from: Option<PathBuf>,

    /// リンク先のパスです
    to: Option<PathBuf>,

    /// リンクの状態です
    state: State,
}

fn parse_tilde_and_dot(path: &PathBuf) -> anyhow::Result<PathBuf> {
    let s = path.to_string_lossy();
    PathBuf::from_str(&shellexpand::tilde(&s)).map_err(|e| anyhow!(e))
    // let dot = fs::canonicalize::<PathBuf>(tilde).map_err(|e| anyhow!(e));
}

impl Dotfile {
    pub fn new(from: Option<PathBuf>, to: Option<PathBuf>, is_ignore: bool) -> Self {
        let parsed_from = from.map(|path| parse_tilde_and_dot(&path).ok()).flatten();
        let parsed_to = to.map(|path| parse_tilde_and_dot(&path).ok()).flatten();

        let status = if is_ignore == true {
            State::Ignored
        } else {
            match &parsed_from {
                Some(from) => {
                    match &parsed_to {
                        Some(_) => match read_link(from) {
                            Ok(_) => State::Linked,
                            Err(e) => {
                                // link error
                                eprintln!("LinkERR: {:?}, {:?}", from, e);
                                State::Other
                            }
                        },
                        None => State::Unliked,
                    }
                }
                None => State::Unliked,
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
    use crate::dotfile;
    use std::{os::unix::fs::symlink, str::FromStr};

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
        let dotfile_name = ".samplerc";
        let from = PathBuf::from(format!("./test-resources/test-home/{}", dotfile_name));
        let to = PathBuf::from(format!("./test-resources/test-conf.d/{}", dotfile_name));

        symlink(&to, &from).ok();

        let actual = Dotfile::new(Some(from.clone()), Some(to.clone()));
        let expect = Dotfile {
            from: Some(from),
            to: Some(to),
            state: State::Linked,
        };
        assert_eq!(actual, expect);
    }
}
