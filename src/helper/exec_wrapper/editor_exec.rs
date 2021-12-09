use crate::helper::exec_wrapper::exec_runner;
use std::error::Error;

pub fn edit_file(editor: String, file: String) -> Result<(), Box<dyn Error>> {
    if editor == "~" || editor == "" {
        return Ok(());
    }

    let cmd = format!("{:} {:?}", editor, file);
    exec_runner::cmd_runner(cmd)?;

    Ok(())
}
