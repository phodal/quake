use crate::widgets::MainWidget;
use quake_core::entry::entry_file::EntryFile;
use quake_core::helper::file_filter;
use quake_core::parser::quake::QuakeActionNode;
use quake_core::usecases::entry_usecases;
use quake_core::QuakeConfig;
use std::fs;
use std::path::PathBuf;

pub fn action_result_to_main_widget(
    action_str: &str,
    config: &QuakeConfig,
) -> Result<MainWidget, String> {
    QuakeActionNode::from_text(action_str)
        .map_err(|_| format!("Unknown command: {}", action_str))
        .and_then(|node| match node.action.as_str() {
            "add" => entry_usecases::create_entry(&config.workspace, &node.object, &node.text)
                .map(|(_, file)| MainWidget::Editor {
                    entry_type: node.object.clone(),
                    id: file.id,
                    content: "".to_string(),
                })
                .map_err(|e| format!("Can't create entry: {}", e)),
            "edit" => {
                let base_path = PathBuf::from(&config.workspace).join(&node.object);
                let index = node.parameters[0].parse().unwrap();
                let prefix = EntryFile::file_prefix(index);
                let paths = file_filter::filter_by_prefix(base_path, prefix);
                if paths.is_empty() {
                    return Err(format!("Entry file {} not found!", index));
                }
                let file_path = paths[0].clone();

                fs::read_to_string(file_path)
                    .map(|content| MainWidget::Editor {
                        entry_type: node.object.clone(),
                        id: index,
                        content,
                    })
                    .map_err(|e| format!("Read entry error: {}", e))
            }
            _ => Err(format!("Not implemented action: {}", action_str)),
        })
}

#[cfg(test)]
mod tests {
    use super::action_result_to_main_widget;
    use crate::widgets::MainWidget;
    use quake_core::QuakeConfig;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_add_entry_action() {
        let mut config = QuakeConfig::default();
        config.workspace = "../_fixtures/demo_quake".to_string();
        let result = action_result_to_main_widget("tui_test.add: hello", &config);
        let editor = MainWidget::Editor {
            entry_type: "tui_test".to_string(),
            id: 1,
            content: "".to_string(),
        };
        assert_eq!(result, Ok(editor));

        fs::remove_dir_all(PathBuf::from(&config.workspace).join("tui_test")).unwrap();
    }
}
