use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::Duration;

use futures::channel::mpsc::{channel, Receiver};
use futures::{SinkExt, StreamExt};
use notify::event::ModifyKind;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use rocket::info;
use tracing::{debug, error};

use quake_core::entry::entry_file::EntryFile;
use quake_core::entry::entry_paths::EntryPaths;
use quake_core::entry::EntryDefines;
use quake_core::errors::QuakeError;
use quake_core::helper::file_filter::type_from_md_path;
use quake_core::helper::quake_time;
use quake_core::meta::MetaProperty;

use crate::helper::exec_wrapper::meili_exec::feed_document;

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);
    let mut watcher = RecommendedWatcher::new(move |res| {
        futures::executor::block_on(async {
            tx.send(res).await.unwrap();
        })
    })?;

    let _ = watcher.configure(notify::Config::OngoingEvents(Some(Duration::from_secs(2))));

    Ok((watcher, rx))
}

// todo: add type merge for ranges
pub async fn async_watch<P: AsRef<Path>>(workspace: P, search_url: String) -> notify::Result<()> {
    debug!("start watch: {:?}", workspace.as_ref());
    let (mut watcher, mut rx) = async_watcher()?;
    watcher.watch(workspace.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => {
                if let Err(err) = feed_by_event(event, &search_url, workspace.as_ref()) {
                    error!("watch error: {:?}", err)
                };
            }
            Err(e) => error!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn feed_by_event(event: Event, search_url: &str, workspace: &Path) -> Result<(), Box<dyn Error>> {
    // todo: looking for better way
    match &event.kind {
        EventKind::Modify(ModifyKind::Data(_data)) => {
            // do something
        }
        _ => return Ok(()),
    }

    debug!("feed_by_event {:?}", &event);
    for path in event.paths {
        if path.is_dir() {
            continue;
        }

        if let Some(ext) = path.extension() {
            // by rules
            // 1. match quake entry: file by suffix for start with index
            // 2. match file for engine?
            // generate
            if !ext.eq("md") {
                continue;
            }
        }

        let (typ, file) = entry_file_by_path(&path, workspace)?;
        feed_document(search_url, &typ, &file)?;
    }

    Ok(())
}

pub fn entry_file_by_path(
    path: &Path,
    workspace: &Path,
) -> Result<(String, EntryFile), Box<dyn Error>> {
    let entry_type = type_from_md_path(path).ok_or("")?;
    let file_name = path.file_name().ok_or("")?;

    if file_name.is_empty() || entry_type.is_empty() {
        return Err(Box::new(QuakeError(format!(
            "empty type {:?} or file_name {:?}",
            entry_type, file_name
        ))));
    }

    let id = EntryFile::id_from_name(file_name.to_str().unwrap().to_string().as_str())?;
    let content = fs::read_to_string(&path)?;

    let mut file = EntryFile::from(content.as_str(), id)?;
    let defines = EntryDefines::from_path(&*workspace.join(EntryPaths::entries_define()));
    if let Some(define) = defines.find(&*entry_type) {
        for (key, prop) in define.to_field_type() {
            if let MetaProperty::Date(_date) = prop {
                let text = &*file.property(&key).unwrap();
                let value = quake_time::string_date_to_unix(text);
                file.update_property(&key, &value);
                info!("update {:} date: from {:} to {:}", key, text, value);
            }
        }
    }

    Ok((entry_type, file))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::helper::entry_watcher::entry_file_by_path;

    #[test]
    fn entry_by_path() {
        let workspace = PathBuf::from("examples");
        let buf = workspace.join("todo").join("0001-time-support.md");

        let (typ, file) = entry_file_by_path(&buf, &workspace).unwrap();
        assert_eq!(typ, "todo".to_string());
        assert_eq!(1, file.id);
        assert_eq!("1637781250", file.property("created_date").unwrap());
    }
}
