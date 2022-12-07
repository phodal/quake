use std::error::Error;
use std::path::Path;
use std::time::Duration;

use futures::channel::mpsc::{channel, Receiver};
use futures::{SinkExt, StreamExt};
use notify::event::ModifyKind;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use tracing::{debug, error};

use quake_core::entry::entry_by_path;

use crate::helper::exec_wrapper::meili_exec::feed_document;

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);
    let mut watcher = notify::recommended_watcher(move |res| {
        futures::executor::block_on(async {
            tx.send(res).await.unwrap();
        })
    })?;

    let _ = watcher.configure(
        notify::Config::default()
            .with_poll_interval(Duration::from_secs(2))
            .with_compare_contents(true),
    );

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

    for path in &event.paths {
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

        debug!("feed_by_event {:?}", &event);
        let (typ, file) = entry_by_path::entry_file_dump(path, workspace)?;
        feed_document(search_url, &typ, &file)?;
    }

    Ok(())
}
