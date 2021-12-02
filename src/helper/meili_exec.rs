use std::error::Error;
use std::process::Command;

pub fn feed_command(path_name: &String) -> Result<(), Box<dyn Error>> {
    let url = format!("http://127.0.0.1:7700/indexes/{:}/documents", path_name);
    Command::new("/bin/sh")
        .arg("-c")
        .arg(format!("curl -i -X POST '{:}' \
  --header 'content-type: application/json' \
  --data-binary @dump.json", url))
        .spawn()?
        .wait()?;

    Ok(())
}

// todo: add sort by date

