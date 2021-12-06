use std::error::Error;
use std::process::Command;

pub fn feed_command(index_name: &String) -> Result<(), Box<dyn Error>> {
    // todo: check curl
    let url = format!("http://127.0.0.1:7700/indexes/{:}/documents", index_name);
    Command::new("/bin/sh")
        .arg("-c")
        .arg(format!(
            "curl -i -X POST '{:}' \
  --header 'content-type: application/json' \
  --data-binary @dump.json",
            url
        ))
        .spawn()?
        .wait()?;

    Ok(())
}

pub fn feed_settings(index_name: &String) -> Result<(), Box<dyn Error>> {
    // todo: check curl
    let url = format!("http://127.0.0.1:7700/indexes/{:}/settings", index_name);
    Command::new("/bin/sh")
        .arg("-c")
        .arg(format!(
            "curl -i -X POST '{:}' \
  --header 'content-type: application/json' \
  --data-binary @resources/search_rule.json",
            url
        ))
        .spawn()?
        .wait()?;

    Ok(())
}

pub fn feed_entry(index_name: &String, content: &String) -> Result<(), Box<dyn Error>> {
    let url = format!("http://127.0.0.1:7700/indexes/{:}/documents", index_name);
    let cmd_line = format!(
        "curl -i -X POST '{:}' \
  --header 'content-type: application/json' \
  --data-binary {:?}",
        url, content
    );
    println!("{:?}", cmd_line);
    Command::new("/bin/sh")
        .arg("-c")
        .arg(cmd_line)
        .spawn()?
        .wait()?;

    Ok(())
}

// todo: add sort by date
