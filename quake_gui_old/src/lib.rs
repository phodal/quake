use dioxus::prelude::*;
use quake_core::entry::{entry_paths::EntryPaths, EntryDefines};
use std::path::PathBuf;

pub fn launch(workspace: PathBuf) {
    dioxus::desktop::launch_with_props(app, AppProps { workspace }, |c| {
        c.with_custom_head("<script src=\"https://cdn.tailwindcss.com\"></script>".to_string())
    });
}

fn app(cx: Scope<AppProps>) -> Element {
    let defines = EntryDefines::from_path(&cx.props.workspace.join(EntryPaths::entries_define()));

    rsx!(cx, div {
        main {
            defines.entries.iter().map(|define| {
                rsx!(
                    div {
                        key: "{define.entry_type}",
                        "{define.entry_type}"
                    }
                )
            })
        }
    })
}

struct AppProps {
    workspace: PathBuf,
}
