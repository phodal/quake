use std::path::PathBuf;
use std::process::Command;

pub fn last_modify(exec_path: Option<String>, path: PathBuf) -> String {
    let mut command = Command::new("git");

    if let Some(path) = exec_path {
        command.arg("-C").arg(path);
    }

    let git_cmd = command
        .arg("log")
        .arg("--pretty=format: %at %aN<%ae> %d %s")
        .arg("--date=unix")
        .arg(format!("{}", path.display()));

    let output = git_cmd.output().expect("ls command failed to start");
    return String::from_utf8_lossy(&*output.stdout).to_string();
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::helper::git_cmd::last_modify;

    #[test]
    fn last() {
        let last = last_modify(None, PathBuf::from("README.md"));
        println!("{:?}", last);
    }
}
