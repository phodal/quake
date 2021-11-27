use std::fs;
use std::path::PathBuf;

use quake_microsoft_todo::tasks::{TodoTask, WellknownListName};

use quake_core::entry::entry_file::EntryFile;
use quake_core::entry::FrontMatter;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OutputList {
    pub display_name: String,
    pub id: String,
    pub wellknown_list_name: WellknownListName,
    pub children: Vec<TodoTask>,
}

pub fn dump_microsoft_todo(todos_lists: Vec<OutputList>, path: &PathBuf) {
    let _ = fs::create_dir(&path);
    let mut index = 1;
    for list in todos_lists {
        for todo in list.children {
            let mut file = EntryFile::default();
            let mut matter = FrontMatter::default();


            let title = todo.title;
            matter.fields.insert("category".to_string(), format!("{:?}", list.display_name));
            matter.fields.insert("title".to_string(), format!("{:?}", title.clone()));
            matter.fields.insert("created_date".to_string(), todo.created_date_time);
            matter.fields.insert("updated_date".to_string(), todo.last_modified_date_time);

            matter.fields.insert("reminder_date".to_string(), format!("{:?}", match todo.reminder_date_time {
                None => {"".to_string()}
                Some(dat) => { dat.date_time}
            }));

            matter.fields.insert("completed_date".to_string(), format!("{:?}", match todo.completed_date_time {
                None => {"".to_string()}
                Some(dat) => { dat.date_time}
            }));

            matter.fields.insert("due_date".to_string(), format!("{:?}", match todo.due_date_time {
                None => {"".to_string()}
                Some(dat) => { dat.date_time}
            }));

            matter.fields.insert("importance".to_string(), format!("{:?}", todo.importance));
            matter.fields.insert("status".to_string(), format!("{:?}", todo.status));

            file.name = EntryFile::file_name(index, &*title);
            file.front_matter = matter;
            file.content = todo.body.content;

            match fs::write(path.join(file.name.clone()), file.to_string()) {
                Ok(_) => {}
                Err(err) => {
                    println!("{:?}", file.name.clone());
                    println!("{:?}", err);
                }
            }

            index = index + 1
        }
    }
}