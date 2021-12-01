use std::error::Error;
use std::process::Command;

pub fn feed_command(path_name: &String) -> Result<(), Box<dyn Error>> {
    let url = format!("{:?}", path_name);
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

