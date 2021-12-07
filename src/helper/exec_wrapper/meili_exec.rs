use std::error::Error;
use std::process::Command;
use tracing::info;

pub fn feed_command(index_name: &String, search_url: &String) -> Result<(), Box<dyn Error>> {
    let url = format!("{:}/indexes/{:}/documents", search_url, index_name);
    let cmd_line = format!(
        "curl -i -X POST '{:}' \
  --header 'content-type: application/json' \
  --data-binary @dump.json",
        url
    );

    info!("{:?}", cmd_line);
    Command::new("/bin/sh")
        .arg("-c")
        .arg(cmd_line)
        .spawn()?
        .wait()?;

    Ok(())
}

pub fn feed_settings(index_name: &String, search_url: &String) -> Result<(), Box<dyn Error>> {
    let url = format!("{:}/indexes/{:}/settings", search_url, index_name);
    let cmd_line = format!(
        "curl -i -X POST '{:}' \
  --header 'content-type: application/json' \
  --data-binary @resources/search_rule.json",
        url
    );

    info!("{:?}", cmd_line);
    Command::new("/bin/sh")
        .arg("-c")
        .arg(cmd_line)
        .spawn()?
        .wait()?;

    Ok(())
}

pub fn feed_entry(
    index_name: &String,
    content: &String,
    search_url: &String,
) -> Result<(), Box<dyn Error>> {
    let url = format!("{:}/indexes/{:}/documents", search_url, index_name);
    let cmd_line = format!(
        "curl -i -X POST '{:}' \
  --header 'content-type: application/json' \
  --data-binary {:?}",
        url, content
    );

    info!("{:?}", cmd_line);
    Command::new("/bin/sh")
        .arg("-c")
        .arg(cmd_line)
        .spawn()?
        .wait()?;

    Ok(())
}

// todo: add sort by date
