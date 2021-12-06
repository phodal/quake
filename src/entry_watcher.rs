use crate::helper::meili_exec::feed_entry;
use futures::channel::mpsc::{channel, Receiver};
use futures::{SinkExt, StreamExt};
use notify::event::ModifyKind;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use quake_core::entry::entry_file::EntryFile;
use quake_core::errors::QuakeError;
use quake_core::usecases::file_filter::type_from_md_path;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

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
pub async fn async_watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    println!("start watch: {:?}", path.as_ref());
    let (mut watcher, mut rx) = async_watcher()?;
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => {
                match feed_by_event(event) {
                    Ok(_) => {}
                    Err(err) => {
                        println!("watch error: {:?}", err)
                    }
                };
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn feed_by_event(event: Event) -> Result<(), Box<dyn Error>> {
    // only for data modify
    // todo: looking for better way
    match &event.kind {
        EventKind::Modify(modify) => match modify {
            ModifyKind::Data(_data) => {}
            _ => return Ok(()),
        },
        _ => return Ok(()),
    }

    println!("feed_by_event {:?}", &event);
    for path in event.paths {
        if path.is_dir() {
            continue;
        }

        if let Some(ext) = path.extension() {
            if !ext.eq("md") {
                continue;
            }
        }

        let (typ, file) = entry_file_by_path(&path)?;
        let string = serde_json::to_string(&file)?;
        feed_entry(&typ, &string)?;
    }

    Ok(())
}

pub fn entry_file_by_path(path: &PathBuf) -> Result<(String, EntryFile), Box<dyn Error>> {
    let typ = type_from_md_path(&path).ok_or("")?;
    let file_name = path.file_name().ok_or("")?;

    if file_name == "" || typ == "" {
        return Err(Box::new(QuakeError(format!(
            "emtpy typ {:?} or file_name {:?}",
            typ, file_name
        ))));
    }

    let id = EntryFile::id_from_name(format!("{:}", file_name.to_str().unwrap()).as_str())?;
    let content = fs::read_to_string(&path)?;
    let file = EntryFile::from(content.as_str(), id)?;
    Ok((typ, file))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::entry_watcher::entry_file_by_path;

    #[test]
    fn entry_by_path() {
        let buf = PathBuf::from("_fixtures")
            .join("todo")
            .join("0001-time-support.md");

        let (typ, file) = entry_file_by_path(&buf).unwrap();
        assert_eq!(typ, "todo".to_string());
        assert_eq!(1, file.id);
    }
}
