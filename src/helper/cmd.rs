use std::process::Command;

pub fn edit_file(editor: String, file: String) {
    // todo: split os
    Command::new("/bin/sh")
        .arg("-c")
        .arg(format!("{:} {:?}", editor, file))
        // .arg(file)
        .spawn()
        .expect("Error: Failed to run editor")
        .wait()
        .expect("failed to execute process");
}
