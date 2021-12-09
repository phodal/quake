use std::error::Error;
use std::process::Command;

pub fn cmd_runner(editor_cmd: String) -> Result<(), Box<dyn Error>> {
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
