use lazy_static::lazy_static;
use regex::Regex;
use std::path::PathBuf;
use std::process::Command;

lazy_static! {
    static ref CHANGE_REGEX: Regex = Regex::new(
        r"(?x)
\s(?P<date>\d{10})
\s(?P<author>.*?)<(?P<email>.*?)>
\s(?P<message>.*)
        "
    )
    .unwrap();
}

pub fn last_modify(exec_path: Option<String>, path: PathBuf) -> String {
    let mut command = Command::new("git");

    if let Some(path) = exec_path {
        command.arg("-C").arg(path);
    }

    let git_cmd = command
        .arg("log")
        .arg("--pretty=format: %at %aN<%ae> %s")
        .arg("--date=unix")
        .arg(format!("{}", path.display()));

    let output = git_cmd.output().expect("ls command failed to start");
    return String::from_utf8_lossy(&*output.stdout).to_string();
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FileChange {
    pub author: String,
    pub email: String,
    pub date: i64,
    pub message: String,
}

impl Default for FileChange {
    fn default() -> Self {
        FileChange {
            author: "".to_string(),
            email: "".to_string(),
            date: 0,
            message: "".to_string(),
        }
    }
}

pub fn parse_changes(last: String) -> Vec<FileChange> {
    let mut changes = vec![];
    for line in last.lines() {
        if let Some(captures) = CHANGE_REGEX.captures(line) {
            let date = &captures["date"];
            let author = &captures["author"];
            let email = &captures["email"];
            let message = &captures["message"];

            changes.push(FileChange {
                author: author.to_string(),
                email: email.to_string(),
                date: date.parse().unwrap(),
                message: message.to_string(),
            });
        }
    }
    changes
}

#[cfg(test)]
mod tests {
    use crate::helper::git_cmd::{last_modify, parse_changes};
    use std::path::PathBuf;

    #[test]
    #[ignore]
    fn last_message() {
        let last = last_modify(None, PathBuf::from("README.md"));
        let changes = parse_changes(last);
        assert!(changes.len() > 10);
    }

    #[test]
    fn parse_messages() {
        let str = " 1637816262 Phodal Huang<h@phodal.com> refactor: reset entry process logic
 1637766252 Phodal Huang<h@phodal.com> refactor: move modules";

        let changes = parse_changes(str.to_string());
        assert_eq!(1637816262, changes[0].date);
        assert_eq!("Phodal Huang", changes[0].author);
        assert_eq!("h@phodal.com", changes[0].email);
        assert_eq!("refactor: reset entry process logic", changes[0].message);
    }
}
