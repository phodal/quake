#[macro_use]
extern crate serde;

use std::fs;
use std::path::PathBuf;

use clap::Parser;

pub mod onenote;
pub mod sqlite_to_file;
pub mod todo_to_file;

#[derive(Parser)]
#[clap(version = "0.0.1", author = "Phodal HUANG<h@phodal.com>")]
struct Opts {
    #[clap(subcommand)]
    cmd: ImportCmd,
}

#[derive(Parser)]
enum ImportCmd {
    SQLITE(SQLite),
    MicrosoftTodo(MicrosoftTodo),
}

#[derive(Parser)]
pub struct MicrosoftTodo {
    #[clap(short, long)]
    path: String,

    #[clap(short, long, default_value = "")]
    output: String,
}

#[derive(Parser)]
pub struct SQLite {
    #[clap(short, long)]
    path: String,

    #[clap(short, long, default_value = "")]
    output: String,

    #[clap(short, long, default_value = "")]
    inside_type: String,

    #[clap(short, long, default_value = "")]
    sql: String,
}

/// refs: https://www.swiftforensics.com/2018/02/reading-notes-database-on-macos.html
pub fn dump_apple_notes(db_path: &str, path: PathBuf) {
    let sql = "
SELECT Title as title, Snippet as description, Folder as category, Created as created_date,
 LastModified as updated_date, Data as content, User as author
  from Notes
";

    let _ = fs::create_dir(&path);
    if let Err(err) = sqlite_to_file::export(db_path, sql, path) {
        println!("{:?}", err);
    };
}

pub fn dump_phodal_com(db_path: &str, path: PathBuf) {
    let sql = "SELECT blog_blogpost.keywords_string as keywords, blog_blogpost.title, blog_blogpost.description, blog_blogpost.slug, blog_blogpost.content,
       auth_user.first_name, auth_user.last_name, auth_user.email, created as created_date, updated as updated_date
FROM blog_blogpost
         INNER JOIN auth_user
                    ON blog_blogpost.user_id = auth_user.id
";

    let _ = fs::create_dir(&path);
    if let Err(err) = sqlite_to_file::export(db_path, sql, path) {
        println!("{:?}", err);
    };
}

fn main() {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        ImportCmd::SQLITE(sqlite) => {
            let output = PathBuf::from(sqlite.output);
            let path = sqlite.path.as_str();

            match sqlite.inside_type.as_str() {
                "mezzanine" => {
                    dump_phodal_com(path, output);
                    return;
                }
                "apple-notes" => {
                    dump_apple_notes(path, output);
                    return;
                }
                &_ => {}
            }

            if sqlite.sql.len() > 0 {
                let _ = fs::create_dir(&output);
                if let Err(err) = sqlite_to_file::export(path, &*sqlite.sql, output) {
                    println!("{:?}", err);
                };
            }
        }
        ImportCmd::MicrosoftTodo(_) => {}
    }
}

#[cfg(test)]
mod tests {
    use quake_core::entry::entry_file::EntryFile;
    use std::fs;
    use std::path::PathBuf;

    use crate::dump_apple_notes;
    use crate::dump_phodal_com;
    use crate::onenote::SectionVO;
    use crate::todo_to_file::{dump_microsoft_todo, OutputList};

    #[ignore]
    #[test]
    fn dump_test() {
        let path = PathBuf::from("..").join("examples").join("phodal_com");
        let _ = dump_phodal_com("../dbs/phodal.dev", path);
    }

    #[ignore]
    #[test]
    fn dump_notes() {
        let path = PathBuf::from("..").join("examples").join("notes");
        let _ = dump_apple_notes("../dbs/mac_apt.db", path);
    }

    #[ignore]
    #[test]
    fn dump_local_todo() {
        let output_dir = PathBuf::from("../").join("examples").join("microsoft_todo");

        let input = PathBuf::from("..").join("dbs").join("todo-output.json");
        let todo = fs::read_to_string(format!("{:}", input.display())).unwrap();
        let vec: Vec<OutputList> = serde_json::from_str(&*todo).unwrap();

        let _ = dump_microsoft_todo(vec, &output_dir);
    }

    #[ignore]
    #[test]
    fn dump_onenote() {
        let source = PathBuf::from("..").join("dbs");
        let target = PathBuf::from("..").join("examples").join("onenote");
        fs::create_dir_all(&target).unwrap();

        let content_path = PathBuf::from("..").join("dbs").join("content");
        let src = source.join("onenote-output.json");

        let todo = fs::read_to_string(format!("{:}", src.display())).unwrap();
        let sections: Vec<SectionVO> = serde_json::from_str(&*todo).unwrap();

        let mut index = 1;
        for section in sections {
            for page in section.pages {
                let mut file = EntryFile::default();
                file.name = EntryFile::file_name(index, &*page.title);

                file.add_field("category", format!("{:?}", section.display_name).as_str());
                file.add_field("notebook", format!("{:?}", section.parent_name).as_str());
                file.add_field("title", format!("{:?}", page.title).as_str());
                file.add_field("created_date", page.created_date_time.clone().as_str());
                file.add_field("updated_date", page.created_date_time.as_str());

                file.content = "\n\n".to_string();

                let source_file = content_path.join(format!("{:}.md", page.id));
                println!("{:}", &source_file.display());
                let content = fs::read_to_string(source_file).unwrap_or("".to_string());

                file.content.push_str(content.as_str());

                match fs::write(&target.join(file.name.clone()), file.to_string()) {
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

    #[test]
    fn dump_todo() {
        let output_dir = PathBuf::from("test_temp").join("todo");
        fs::create_dir_all(&output_dir).unwrap();

        let input = PathBuf::from("../")
            .join("_fixtures")
            .join("import_test")
            .join("todo.json");

        let todo = fs::read_to_string(format!("{:}", input.display())).unwrap();
        let vec: Vec<OutputList> = serde_json::from_str(&*todo).unwrap();
        let _ = dump_microsoft_todo(vec, &output_dir);

        let str = fs::read_to_string(output_dir.join("0001-game-develop.md")).unwrap();
        let file = EntryFile::from(str.as_str(), 1).unwrap();

        assert_eq!(file.field("title").unwrap(), "Game Develop");
        fs::remove_dir_all(output_dir).unwrap();
    }
}
