use std::error::Error;
use std::process::Command;

pub fn edit_file(editor: String, file: String) -> Result<(), Box<dyn Error>> {
    let editor_cmd = format!("{:} {:?}", editor, file);

    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", editor_cmd.as_str()])
            .spawn()?
            .wait()?;
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(editor_cmd)
            .spawn()?
            .wait()?;
    };


    Ok(())
}
