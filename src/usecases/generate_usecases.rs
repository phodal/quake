use std::error::Error;
use std::path::PathBuf;

use quake_core::errors::QuakeError;
use quake_core::quake::QuakeTransflowNode;

pub fn generate_by_flow(flow: &str) -> Result<(), Box<dyn Error>> {
    let flow = format!("transflow generate {{ {:} }}", flow);
    let node = QuakeTransflowNode::from_text(&flow)?;
    let route = &node.routes[0];

    // pre condition
    let source_dir = PathBuf::from(&route.from[0]);
    if !source_dir.exists() {
        return Err(Box::new(QuakeError(format!(
            "path {:?} don't exists",
            source_dir
        ))));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::generate_by_flow;

    #[test]
    fn return_absolute_when_file_exists() {
        generate_by_flow("from('examples').to('papers').filter('*.pdf')").expect("");
    }
}
