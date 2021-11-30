use std::error::Error;
use std::process::Command;

pub fn edit_file(editor: String, file: String) -> Result<(), Box<dyn Error>>{
    // todo: split os
    Command::new("/bin/sh")
        .arg("-c")
        .arg(format!("{:} {:?}", editor, file))
        .spawn()?
        .wait()?;

    Ok(())
}
